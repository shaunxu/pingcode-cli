use crate::args::ArgParser;
use crate::configure::OpContext;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use crate::AnyError;
use clap::ArgMatches;

pub struct AgileStateGetOpExecutor {}
impl OpExecutor for AgileStateGetOpExecutor {
        fn on_execute<'a>(&self, matches: &'a ArgMatches, _context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: matches.value_of("id"),
            query: None,
            body: None,
            parents: None,
        })
    }
}

pub struct AgileStateListOpExecutor {}
impl OpExecutor for AgileStateListOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches, context: &OpContext) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: Some(ArgParser::parse_query_from_args(matches, context)),
            body: None,
            parents: None,
        })
    }
}