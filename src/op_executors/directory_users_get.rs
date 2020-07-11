use crate::configure::OpContext;
use crate::AnyError;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;

pub struct DirectoryUsersGetOpExecutor {}

impl OpExecutor for DirectoryUsersGetOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches, context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: matches.value_of("id"),
            query: None,
            body: None,
        })
    }
}
