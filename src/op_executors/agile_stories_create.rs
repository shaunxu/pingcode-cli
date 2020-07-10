use crate::AnyError;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use clap::ArgMatches;
use crate::args::ArgParser;

pub struct AgileStoriesCreateOpExecutor {}

impl OpExecutor for AgileStoriesCreateOpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::POST,
            param: None,
            query: None,
            body: ArgParser::parse_content_to_json(matches)?,
        })
    }
}
