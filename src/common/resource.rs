use crate::common::op::Op;
use clap::{App, SubCommand, ArgMatches};

pub trait Resource {
    fn get_area_name(&self) -> &str;

    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;

    fn get_ops(&self) -> &std::vec::Vec<Box<dyn Op>>;

    fn to_subcommand(&self) -> App {
        let mut subcommand = SubCommand::with_name(self.get_name()).about(self.get_description());
        for resource in self.get_ops().iter() {
            subcommand = subcommand.subcommand(resource.to_subcommand());
        }
        subcommand
    }

    fn match_subcommand(&self, matches: &ArgMatches) -> () {
        if let Some(subcommand) = matches.subcommand_matches(self.get_name()) {
            for op in self.get_ops().iter() {
                op.match_subcommand(subcommand);
            }
        }
    }
}
