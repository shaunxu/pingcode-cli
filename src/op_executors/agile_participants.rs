use crate::args::ArgParser;
use crate::configure::OpContext;
use crate::op_executors::op_executor::OpExecutor;
use crate::op_executors::op_executor::OpRequest;
use crate::AnyError;
use clap::ArgMatches;

pub struct AgileParticipantsListOpExecutor {}
impl OpExecutor for AgileParticipantsListOpExecutor {
    fn on_execute<'a>(
        &self,
        matches: &'a ArgMatches,
        _context: &OpContext,
    ) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::GET,
            param: None,
            query: None,
            body: None,
            parents: Some(ArgParser::parse_parents(
                matches,
                vec![("work_items", "work-item-id")],
            )),
        })
    }
}

pub struct AgileParticipantsAddOpExecutor {}
impl OpExecutor for AgileParticipantsAddOpExecutor {
    fn on_execute<'a>(
        &self,
        matches: &'a ArgMatches,
        _context: &OpContext,
    ) -> Result<OpRequest<'a>, AnyError> {
        let mut json = serde_json::Map::new();
        json.insert(
            String::from("user_id"),
            serde_json::Value::String(String::from(matches.value_of("uid").unwrap())),
        );
        Ok(OpRequest {
            method: reqwest::Method::POST,
            param: None,
            query: None,
            body: Some(serde_json::Value::Object(json)),
            parents: Some(ArgParser::parse_parents(
                matches,
                vec![("work_items", "work-item-id")],
            )),
        })
    }
}

pub struct AgileParticipantsRemoveOpExecutor {}
impl OpExecutor for AgileParticipantsRemoveOpExecutor {
    fn on_execute<'a>(
        &self,
        matches: &'a ArgMatches,
        _context: &OpContext,
    ) -> Result<OpRequest<'a>, AnyError> {
        Ok(OpRequest {
            method: reqwest::Method::DELETE,
            param: matches.value_of("uid"),
            query: None,
            body: None,
            parents: Some(ArgParser::parse_parents(
                matches,
                vec![("work_items", "work-item-id")],
            )),
        })
    }
}
