pub mod agile_projects_list;
pub mod op_executor;

use crate::installer::OpContext;
use clap::ArgMatches;

pub struct OpExecutors {
    executors: std::collections::HashMap<String, Box<dyn op_executor::OpExecutor>>,
}

impl OpExecutors {
    pub fn initialize() -> OpExecutors {
        let mut es: std::collections::HashMap<String, Box<dyn op_executor::OpExecutor>> =
            std::collections::HashMap::new();
        es.insert(
            String::from("agile_projects_list"),
            Box::new(agile_projects_list::AgileProjectListOpExecutor {}),
        );
        OpExecutors { executors: es }
    }

    pub fn execute(&self, ctx: &OpContext, matches: &ArgMatches) -> () {
        if let Some(executor) = self.executors.get(&ctx.key) {
            executor.execute(matches, ctx);
        } else {
            panic!("Cannot find op through {}", ctx.key);
        }
    }
}
