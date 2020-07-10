pub mod agile_projects_get;
pub mod agile_projects_list;
pub mod op_executor;

use crate::AnyError;
use crate::configure::OpContext;
use clap::ArgMatches;
use crate::wt_error::WTError;

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
        es.insert(
            String::from("agile_projects_get"),
            Box::new(agile_projects_get::AgileProjectGetOpExecutor {}),
        );
        OpExecutors { executors: es }
    }

    pub fn execute(&self, ctx: &OpContext, matches: &ArgMatches) -> Result<(), AnyError> {
        if let Some(executor) = self.executors.get(&ctx.key) {
            executor.execute(matches, ctx)
        } else {
            Err(WTError::new_boxed("000000", &format!("Cannot find op through {}", ctx.key)))
        }
    }
}
