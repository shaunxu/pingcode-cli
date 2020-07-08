use crate::configure::OpContext;
use crate::json_printer;
use crate::wt_client::WTClient;
use clap::ArgMatches;

pub struct OpRequest<'a> {
    pub method: reqwest::Method,
    pub param: Option<&'a str>,
    pub query: Option<std::vec::Vec<(&'a str, String)>>,
    pub body: Option<serde_json::Value>,
}

pub trait OpExecutor {
    fn on_execute<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a>;

    fn execute(&self, matches: &ArgMatches, context: &OpContext) -> () {
        let req = self.on_execute(matches);
        let fut = WTClient::request(
            req.method,
            Some(&context.area_route),
            &context.resource_route,
            req.param,
            req.query,
            req.body.as_ref(),
        );
        let res = futures::executor::block_on(fut).unwrap();
        json_printer::JSONPrinter::print_by_arg(res, matches);
    }
}
