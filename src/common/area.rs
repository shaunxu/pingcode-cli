use crate::common::resource::Resource;
use clap::{App, SubCommand, ArgMatches};

pub trait Area {
    fn get_name(&self) -> &str;

    fn get_description(&self) -> &str;

    fn to_subcommand(&self) -> App {
        let mut subcommand = SubCommand::with_name(self.get_name()).about(self.get_description());
        for resource in self.get_resources().iter() {
            subcommand = subcommand.subcommand(resource.to_subcommand());
        }
        subcommand
    }

    fn get_resources(&self) -> &std::vec::Vec<Box<dyn Resource>>;

    fn match_subcommand(&self, matches: &ArgMatches) -> () {
        if let Some(subcommand) = matches.subcommand_matches(self.get_name()) {
            for resource in self.get_resources().iter() {
                resource.match_subcommand(subcommand);
            }
        }
    }
}
