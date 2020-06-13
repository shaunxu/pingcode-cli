use crate::json_printer::JSONPrinter;
use crate::wt_client::WTClient;
use clap::{App, Arg, ArgMatches, SubCommand};

pub struct OpRequest<'a> {
    pub method: reqwest::Method,
    pub param: Option<&'a str>,
    pub query: Option<std::vec::Vec<(&'a str, String)>>,
    pub body: Option<serde_json::Value>,
}

pub trait Op {
    fn get_area_name(&self) -> &str;

    fn get_resource_name(&self) -> &str;

    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;

    fn get_args(&self) -> std::vec::Vec<Arg>;

    fn to_subcommand(&self) -> App {
        let mut subcommand = SubCommand::with_name(self.get_name()).about(self.get_description());
        for arg in self.get_args().iter() {
            subcommand = subcommand.arg(arg);
        }
        subcommand
    }

    fn match_subcommand(&self, matches: &ArgMatches) -> () {
        if let Some(subcommand) = matches.subcommand_matches(self.get_name()) {
            self.do_op(subcommand)
        }
    }

    fn on_do_op<'a>(&self, matches: &'a ArgMatches) -> OpRequest<'a>;

    fn do_op(&self, matches: &ArgMatches) -> () {
        let req = self.on_do_op(matches);
        let fut = WTClient::request(
            req.method,
            Some(self.get_area_name()),
            self.get_resource_name(),
            req.param,
            req.query,
            req.body.as_ref(),
        );
        let res = futures::executor::block_on(fut).unwrap();
        JSONPrinter::print_by_arg(res, matches);
    }
    
}
