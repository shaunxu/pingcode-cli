use crate::configure::OpContext;
use crate::json_printer;
use crate::wt_client::Parent;
use crate::wt_client::WTClient;
use crate::AnyError;
use clap::ArgMatches;

pub struct OpRequest<'a> {
    pub method: reqwest::Method,
    pub param: Option<&'a str>,
    pub query: Option<std::vec::Vec<(String, String)>>,
    pub body: Option<serde_json::Value>,
    pub parents: Option<std::vec::Vec<Parent>>,
}

pub trait OpExecutor {
    fn on_execute<'a>(
        &self,
        matches: &'a ArgMatches,
        context: &OpContext,
    ) -> Result<OpRequest<'a>, AnyError>;

    fn execute(&self, matches: &ArgMatches, context: &OpContext) -> Result<(), AnyError> {
        let req = self.on_execute(matches, context)?;
        let fut = WTClient::request(
            req.method,
            Some(&context.area_route),
            &context.resource_route,
            req.param,
            req.parents,
            req.query,
            req.body.as_ref(),
        );
        let res = futures::executor::block_on(fut)?;
        Ok(json_printer::JSONPrinter::print_by_arg(res, matches))
    }
}
