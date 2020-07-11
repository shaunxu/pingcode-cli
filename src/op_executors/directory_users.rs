use crate::configure::OpContext;
use crate::AnyError;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;
use crate::args::ArgParser;

pub struct DirectoryUsersGetOpExecutor {}
impl OpExecutor for DirectoryUsersGetOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches, _context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: matches.value_of("id"),
            query: None,
            body: None,
        })
    }
}

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
