use crate::AnyError;
use crate::args::ArgParser;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;

pub struct DirectoryUsersListOpExecutor {}

impl OpExecutor for DirectoryUsersListOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query(
                matches,
                vec!["name", "page_index", "page_size"],
            )),
            body: None,
        })
    }
}
