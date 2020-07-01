use crate::args;
use crate::op_executors::OpExecutors;
use crate::AnyError;
use clap::ArgMatches;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Serialize, Deserialize)]
pub struct Arg {
    name: String,
    take_value: Option<bool>,
    required: Option<bool>,
    description: Option<String>,
    possible_values: Option<std::vec::Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Op {
    name: String,
    description: String,
    route: Option<String>,
    args: std::vec::Vec<Arg>,
}

#[derive(Debug)]
pub struct OpContext {
    pub key: String,
    pub area_route: String,
    pub resource_route: String,
}

impl OpContext {
    pub fn from_area(area: &Area) -> OpContext {
        OpContext {
            key: format!("{}", area.name),
            area_route: area.get_route(),
            resource_route: String::default(),
        }
    }

    pub fn assign_resource(&self, resource: &Resource) -> OpContext {
        OpContext {
            key: format!("{}_{}", self.key, resource.name),
            area_route: self.area_route.clone(),
            resource_route: resource.get_route(),
        }
    }

    pub fn assign_op(&self, op: &Op) -> OpContext {
        OpContext {
            key: format!("{}_{}", self.key, op.name),
            area_route: self.area_route.clone(),
            resource_route: self.resource_route.clone(),
        }
    }
}

impl Op {
    pub fn match_subcommand(
        &self,
        matches: &ArgMatches,
        ctx: OpContext,
        op_executors: &OpExecutors,
    ) -> () {
        if let Some(subcommand) = matches.subcommand_matches(&self.name) {
            op_executors.execute(&ctx.assign_op(self), subcommand)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Resource {
    name: String,
    description: String,
    route: Option<String>,
    ops: std::vec::Vec<Op>,
}

impl Resource {
    pub fn get_route(&self) -> String {
        self.route.clone().unwrap_or(self.name.clone())
    }

    pub fn match_subcommand(
        &self,
        matches: &ArgMatches,
        ctx: OpContext,
        op_executors: &OpExecutors,
    ) -> () {
        if let Some(subcommand) = matches.subcommand_matches(&self.name) {
            for op in self.ops.iter() {
                op.match_subcommand(subcommand, ctx.assign_resource(&self), op_executors);
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Area {
    name: String,
    description: String,
    route: Option<String>,
    resources: std::vec::Vec<Resource>,
}

impl Area {
    pub fn get_route(&self) -> String {
        self.route.clone().unwrap_or(self.name.clone())
    }

    pub fn match_subcommand(&self, matches: &ArgMatches, op_executors: &OpExecutors) -> () {
        if let Some(subcommand) = matches.subcommand_matches(&self.name) {
            for resource in self.resources.iter() {
                resource.match_subcommand(subcommand, OpContext::from_area(self), op_executors);
            }
        }
    }
}

pub struct Installer {}
impl Installer {
    pub fn load(path: Option<&str>) -> Result<std::vec::Vec<Area>, AnyError> {
        let path = path.unwrap_or(".wt_installer.json");
        let content = std::fs::read_to_string(path)?;
        let json: std::vec::Vec<Area> = serde_json::from_str(&content)?;
        Ok(json)
    }

    pub fn generate_subcommands<'a, 'b, 'c: 'a + 'b>(
        areas: &'c std::vec::Vec<Area>,
    ) -> std::vec::Vec<clap::App<'a, 'b>> {
        let mut apps: std::vec::Vec<clap::App<'a, 'b>> = vec![];
        for area in areas.iter() {
            let mut area_subcommand =
                clap::SubCommand::with_name(&area.name).about(&*area.description);
            for resource in area.resources.iter() {
                let mut resource_subcommand =
                    clap::SubCommand::with_name(&resource.name).about(&*resource.description);
                for op in resource.ops.iter() {
                    let mut op_subcommand =
                        clap::SubCommand::with_name(&op.name).about(&*op.description);
                    for arg in op.args.iter() {
                        let build_in_args = args::BuildInArgs::get(&arg.name);
                        if build_in_args.len() > 0 {
                            op_subcommand = op_subcommand.args(&build_in_args);
                        } else {
                            let mut ca = clap::Arg::with_name(&arg.name).long(&arg.name);
                            if let Some(takes_value) = arg.take_value {
                                ca = ca.takes_value(takes_value);
                            }
                            if let Some(required) = arg.required {
                                ca = ca.required(required);
                            }
                            if let Some(description) = &arg.description {
                                ca = ca.help(&*description);
                            }
                            if let Some(possible_values) = &arg.possible_values {
                                for possible_value in possible_values.iter() {
                                    ca = ca.possible_value(possible_value);
                                }
                            }
                            op_subcommand = op_subcommand.arg(ca);
                        }
                    }
                    resource_subcommand = resource_subcommand.subcommand(op_subcommand);
                }
                area_subcommand = area_subcommand.subcommand(resource_subcommand);
            }
            apps.push(area_subcommand);
        }
        apps
    }
}
