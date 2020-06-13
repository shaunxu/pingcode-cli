use crate::common::op::Op;
use clap::ArgMatches;
use clap::Arg;
use crate::common::op::OpRequest;

pub struct DictionaryUsersGetOp {
    area_name: String,
    resource_name: String,
}

impl DictionaryUsersGetOp {
    pub fn new(area_name: &str, resource_name: &str) -> DictionaryUsersGetOp {
        DictionaryUsersGetOp {
            area_name: String::from(area_name),
            resource_name: String::from(resource_name),
        }
    }
}

impl Op for DictionaryUsersGetOp {
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
        "Get a user by id"
    }

    fn get_args(&self) -> std::vec::Vec<Arg> {
        vec![
            Arg::with_name("id")
                .long("id")
                .takes_value(true)
                .required(true)
                .help("The id of the project will be get"),
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
