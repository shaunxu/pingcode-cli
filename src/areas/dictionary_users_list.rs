use crate::args::GeneralArgs;
use crate::common::op::Op;
use crate::common::op::OpRequest;
use clap::Arg;
use clap::ArgMatches;
use crate::args::ArgParser;

pub struct DictionaryUsersListOp {
    area_name: String,
    resource_name: String,
}

impl DictionaryUsersListOp {
    pub fn new(area_name: &str, resource_name: &str) -> DictionaryUsersListOp {
        DictionaryUsersListOp {
            area_name: String::from(area_name),
            resource_name: String::from(resource_name),
        }
    }
}

impl Op for DictionaryUsersListOp {
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
        "Get all users"
    }

    fn get_args(&self) -> std::vec::Vec<Arg> {
        vec![
            GeneralArgs::page_index(),
            GeneralArgs::page_size(),
        ]
    }

    fn on_do_op<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a> {
        OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query(matches, vec!["page_index", "page_size"])),
            body: None,
        }
    }
}