use crate::common::op::Op;
use clap::ArgMatches;
use clap::Arg;
use crate::args::GeneralArgs;
use crate::common::op::OpRequest;

pub struct GetOp {
    area_name: String,
    resource_name: String,
}

impl GetOp {
    pub fn new(area_name: &str, resource_name: &str) -> GetOp {
        GetOp {
            area_name: String::from(area_name),
            resource_name: String::from(resource_name),
        }
    }
}

impl Op for GetOp {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }

    fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_name(&self) -> &str {
        "get"
    }

    fn get_description(&self) -> &str {
        "Get a project by id"
    }

    fn get_args(&self) -> std::vec::Vec<Arg> {
        vec![
            Arg::with_name("id")
                .long("id")
                .takes_value(true)
                .required(true)
                .help("The id of the project will be get"),
            GeneralArgs::pretty(),
        ]
    }

    fn on_do_op<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a> {
        OpRequest {
            method: reqwest::Method::GET,
            param: matches.value_of("id"),
            query: None,
            body: None,
        }
    }

}
