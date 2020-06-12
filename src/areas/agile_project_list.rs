use crate::args::ArgParser;
use crate::args::GeneralArgs;
use crate::common::op::Op;
use crate::common::op::OpRequest;
use clap::Arg;
use clap::ArgMatches;

pub struct ListOp {
    area_name: String,
    resource_name: String,
}

impl ListOp {
    pub fn new(area_name: &str, resource_name: &str) -> ListOp {
        ListOp {
            area_name: String::from(area_name),
            resource_name: String::from(resource_name),
        }
    }
}

impl Op for ListOp {
    fn get_area_name(&self) -> &str {
        &self.area_name
    }

    fn get_resource_name(&self) -> &str {
        &self.resource_name
    }

    fn get_name(&self) -> &str {
        "list"
    }

    fn get_description(&self) -> &str {
        "Get all projects"
    }

    fn get_args(&self) -> std::vec::Vec<Arg> {
        vec![
            Arg::with_name("identifier")
                .long("identifier")
                .takes_value(true)
                .required(false)
                .help("The identifier of the project"),
            Arg::with_name("type")
                .long("type")
                .takes_value(true)
                .required(false)
                .possible_values(&["scrum", "kanban", "bug"])
                .help("The type of projects"),
            GeneralArgs::page_index(),
            GeneralArgs::page_size(),
            GeneralArgs::pretty(),
        ]
    }

    fn on_do_op<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a> {
        OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query(
                matches,
                vec!["identifier", "type", "page_index", "page_size"],
            )),
            body: None,
        }
    }
}
