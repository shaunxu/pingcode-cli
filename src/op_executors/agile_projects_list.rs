use crate::AnyError;
use crate::args::ArgParser;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;

pub struct AgileProjectListOpExecutor {}

impl OpExecutor for AgileProjectListOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query(
                matches,
                vec!["identifier", "type", "page_index", "page_size"],
            )),
            body: None,
        })
    }
}
