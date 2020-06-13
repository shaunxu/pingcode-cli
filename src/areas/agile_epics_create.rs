use crate::args::ArgParser;
use crate::args::GeneralArgs;
use crate::common::op::Op;
use crate::common::op::OpRequest;
use clap::Arg;
use clap::ArgMatches;

pub struct AgileEpicsCreateGetOp {
    area_name: String,
    resource_name: String,
}

impl AgileEpicsCreateGetOp {
    pub fn new(area_name: &str, resource_name: &str) -> AgileEpicsCreateGetOp {
        AgileEpicsCreateGetOp {
            area_name: String::from(area_name),
            resource_name: String::from(resource_name),
        }
    }
}

impl Op for AgileEpicsCreateGetOp {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }

    fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_name(&self) -> &str {
        "create"
    }

    fn get_description(&self) -> &str {
        "Create a new epic"
    }

    fn get_args(&self) -> std::vec::Vec<Arg> {
        GeneralArgs::content_and_input()
    }

    fn on_do_op<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a> {
        let raw: &str = &ArgParser::parse_content(matches).unwrap();
        let json: serde_json::Value = serde_json::from_str(&raw).unwrap();

        OpRequest {
            method: reqwest::Method::POST,
            param: None,
            query: None,
            body: Some(json),
        }
    }
}
