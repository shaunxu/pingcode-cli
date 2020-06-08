#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use std::error::Error;

mod args;
mod json_printer;
mod wt_client;

type AnyError = Box<dyn Error>;

const CLAP_TEMPLATE: &'static str = r"
{about} v{version}
by [{author}]

USAGE:
{usage}

{all-args}";

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let clap = app_from_crate!()
        .template(CLAP_TEMPLATE)
        .help_message("Help")
        .version_message("Version")
        .subcommand(
            SubCommand::with_name("login")
                .about("Login Worktile REST API with client id and client secret")
                .arg(
                    Arg::with_name("client_id")
                        .short("c")
                        .long("client-id")
                        .help("The Client ID in Worktile REST API application")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("client_secret")
                        .short("s")
                        .long("client-secret")
                        .help("The Client Secret in Worktile REST API application")
                        .takes_value(true)
                        .required(true),
                )
                .arg(
                    Arg::with_name("api_endpoint")
                        .short("e")
                        .long("api-endpoint")
                        .help("Worktile REST API endpoint")
                        .takes_value(true)
                        .required(true)
                        .default_value("https://open.worktile.com"),
                ),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Test the connective and verify authentication information"),
        )
        .subcommand(
            SubCommand::with_name("dictionary")
                .about("Manage dictionary infomation (user, role, etc.)")
                .subcommand(
                    SubCommand::with_name("user")
                        .about("Manage users")
                        .subcommand(
                            SubCommand::with_name("get")
                                .about("Get a user by id")
                                .arg(
                                    Arg::with_name("id")
                                        .long("id")
                                        .takes_value(true)
                                        .required(true)
                                        .help("The id of the user will be get"),
                                )
                                .arg(args::GeneralArgs::pretty()),
                        )
                        .subcommand(
                            SubCommand::with_name("list")
                                .about("Get all projects")
                                .arg(
                                    Arg::with_name("name")
                                        .long("name")
                                        .takes_value(true)
                                        .required(false)
                                        .help("The name of the user"),
                                )
                                .arg(args::GeneralArgs::page_index())
                                .arg(args::GeneralArgs::page_size())
                                .arg(args::GeneralArgs::pretty()),
                        ),
                )
        )
        .subcommand(
            SubCommand::with_name("agile")
                .about("Manage projects and workitems in agile application")
                .subcommand(
                    SubCommand::with_name("project")
                        .about("Manage projects")
                        .subcommand(
                            SubCommand::with_name("get")
                                .about("Get a project by id")
                                .arg(
                                    Arg::with_name("id")
                                        .long("id")
                                        .takes_value(true)
                                        .required(true)
                                        .help("The id of the project will be get"),
                                )
                                .arg(args::GeneralArgs::pretty()),
                        )
                        .subcommand(
                            SubCommand::with_name("list")
                                .about("Get all projects")
                                .arg(
                                    Arg::with_name("identifier")
                                        .long("identifier")
                                        .takes_value(true)
                                        .required(false)
                                        .help("The identifier of the project"),
                                )
                                .arg(
                                    Arg::with_name("type")
                                        .long("type")
                                        .takes_value(true)
                                        .required(false)
                                        .possible_values(&["scrum", "kanban", "bug"])
                                        .help("The type of projects"),
                                )
                                .arg(args::GeneralArgs::page_index())
                                .arg(args::GeneralArgs::page_size())
                                .arg(args::GeneralArgs::pretty()),
                        ),
                ),
        )
        // .subcommand(
        //     SubCommand::with_name("devops")
        //         .about("Manage DevOps information which displayed in Agile workitem dialog")
        //         .subcommand(
        //             SubCommand::with_name("scm")
        //                 .arg(
        //                     Arg::with_name("create")
        //                         .short("c")
        //                         .long("create")
        //                         .help("Create a new SCM product"),
        //                 )
        //                 .arg(
        //                     Arg::with_name("update")
        //                         .short("u")
        //                         .long("update")
        //                         .help("Update an existing SCM product"),
        //                 )
        //                 .arg(
        //                     Arg::with_name("get")
        //                         .short("g")
        //                         .long("get")
        //                         .help("Get one SCM product by its ID"),
        //                 )
        //                 .arg(
        //                     Arg::with_name("list")
        //                         .short("l")
        //                         .long("list")
        //                         .help("List all SCM products"),
        //                 )
        //                 .group(
        //                     ArgGroup::with_name("action")
        //                         .args(&["create", "update", "get", "list"]),
        //                 ),
        //         )
        //         .subcommand(SubCommand::with_name("user"))
        //         .subcommand(SubCommand::with_name("repo"))
        //         .subcommand(SubCommand::with_name("commit"))
        //         .subcommand(SubCommand::with_name("branch"))
        //         .subcommand(SubCommand::with_name("pr")),
        // )
        .get_matches();

    let mut client = wt_client::WTClient::new(None);

    if let Some(subcommand) = clap.subcommand_matches("login") {
        let client_id = String::from(subcommand.value_of("client_id").unwrap());
        let client_secret = String::from(subcommand.value_of("client_secret").unwrap());
        let api_endpoint = String::from(subcommand.value_of("api_endpoint").unwrap());
        match client.auth(&client_id, &client_secret, &api_endpoint).await {
            Ok(()) => println!("Login successful."),
            Err(e) => println!("Failed: {}", e),
        }
    } else if let Some(_) = clap.subcommand_matches("test") {
        print!("Connecting ... ");
        let res = client.ping().await;
        match res {
            Ok(pong) => println!("Ok: {}", pong),
            Err(e) => println!("Failed: {}", e),
        }
    } else if let Some(subcommand) = clap.subcommand_matches("dictionary") {
        if let Some(subcommand) = subcommand.subcommand_matches("project") {
            if let Some(subcommand) = subcommand.subcommand_matches("get") {
                let id = String::from(subcommand.value_of("id").unwrap());
                let res = client.get_project_by_id(&id).await?;
                json_printer::JSONPrinter::print_by_arg(res, subcommand);
            } else if let Some(subcommand) = subcommand.subcommand_matches("list") {
                let res = client
                    .get_projects(
                        subcommand.value_of("identifier"),
                        subcommand.value_of("type"),
                        subcommand.value_of("page_index"),
                        subcommand.value_of("page_size"),
                    )
                    .await?;
                json_printer::JSONPrinter::print_by_arg(res, subcommand);
            } else {
                println!("{}", subcommand.usage());
            }
        } else if let Some(subcommand) = subcommand.subcommand_matches("user") {
            if let Some(subcommand) = subcommand.subcommand_matches("get") {
                let id = String::from(subcommand.value_of("id").unwrap());
                let res = client.get_user_by_id(&id).await?;
                json_printer::JSONPrinter::print_by_arg(res, subcommand);
            } else if let Some(subcommand) = subcommand.subcommand_matches("list") {
                let res = client
                    .get_users(
                        subcommand.value_of("name"),
                        subcommand.value_of("page_index"),
                        subcommand.value_of("page_size"),
                    )
                    .await?;
                json_printer::JSONPrinter::print_by_arg(res, subcommand);
            } else {
                println!("{}", subcommand.usage());
            }
        } else {
            println!("{}", subcommand.usage());
        }
    } else {
        println!("{}", clap.usage());
    }

    Ok(())
}
