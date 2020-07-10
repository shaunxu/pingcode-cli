pub mod agile_projects_get;
pub mod agile_projects_list;
pub mod directory_users_get;
pub mod directory_users_list;
pub mod agile_epics_create;
pub mod agile_features_create;
pub mod agile_stories_create;
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
        es.insert(
            String::from("directory_users_get"),
            Box::new(directory_users_get::DirectoryUsersGetOpExecutor {}),
        );
        es.insert(
            String::from("directory_users_list"),
            Box::new(directory_users_list::DirectoryUsersListOpExecutor {}),
        );
        es.insert(
            String::from("agile_epics_create"),
            Box::new(agile_epics_create::AgileEpicCreateOpExecutor {}),
        );
        es.insert(
            String::from("agile_features_create"),
            Box::new(agile_features_create::AgileFeaturesCreateOpExecutor {}),
        );
        es.insert(
            String::from("agile_stories_create"),
            Box::new(agile_stories_create::AgileStoriesCreateOpExecutor {}),
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
