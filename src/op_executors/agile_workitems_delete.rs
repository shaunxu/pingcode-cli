use crate::configure::OpContext;
use crate::AnyError;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;

pub struct AgileWorkitemsDeleteOpExecutor {}

impl OpExecutor for AgileWorkitemsDeleteOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches, _context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::DELETE,
            param: matches.value_of("id"),
            query: None,
            body: None,
        })
    }
}
