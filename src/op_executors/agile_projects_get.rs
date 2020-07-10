use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;

pub struct AgileProjectGetOpExecutor {}

impl OpExecutor for AgileProjectGetOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a> {
        OpRequest {
            method: reqwest::Method::GET,
            param: matches.value_of("id"),
            query: None,
            body: None,
        }
    }
}
