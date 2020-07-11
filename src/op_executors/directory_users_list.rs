use crate::configure::OpContext;
use crate::args::ArgParser;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use crate::AnyError;
use clap::ArgMatches;

pub struct DirectoryUsersListOpExecutor {}

impl OpExecutor for DirectoryUsersListOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches, context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query_from_args(matches, context)),
            body: None,
        })
    }
}
