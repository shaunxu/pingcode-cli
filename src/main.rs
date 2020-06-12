#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use std::error::Error;

mod areas;
mod common;

use common::area::Area;

mod args;
mod json_printer;
mod wt_client;
use wt_client::WTClient;

type AnyError = Box<dyn Error>;

const CLAP_TEMPLATE: &'static str = r"
{about} v{version}
by [{author}]

USAGE:
{usage}

{all-args}";

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let dictionary_area = areas::dictionary::DictionaryArea::new();

    let mut app = app_from_crate!()
        .template(CLAP_TEMPLATE)
        .help_message("Help")
        .version_message("Version");
    app = app.subcommand(dictionary_area.to_subcommand());
    app = app.subcommand(
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
            )
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .help("Worktile REST API version")
                    .takes_value(true)
                    .required(true)
                    .default_value("1"),
            ),
    );
    app = app.subcommand(
        SubCommand::with_name("test")
            .about("Test the connective and verify authentication information"),
    );
    app = app.subcommand(
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
    );
    // app = app.subcommand(
    //     SubCommand::with_name("dictionary")
    //         .about("Manage dictionary infomation (user, role, etc.)")
    //         .subcommand(
    //             SubCommand::with_name("user")
    //                 .about("Manage users")
    //                 .subcommand(
    //                     SubCommand::with_name("get")
    //                         .about("Get a user by id")
    //                         .arg(
    //                             Arg::with_name("id")
    //                                 .long("id")
    //                                 .takes_value(true)
    //                                 .required(true)
    //                                 .help("The id of the user will be get"),
    //                         )
    //                         .arg(args::GeneralArgs::pretty()),
    //                 )
    //                 .subcommand(
    //                     SubCommand::with_name("list")
    //                         .about("Get all projects")
    //                         .arg(
    //                             Arg::with_name("name")
    //                                 .long("name")
    //                                 .takes_value(true)
    //                                 .required(false)
    //                                 .help("The name of the user"),
    //                         )
    //                         .arg(args::GeneralArgs::page_index())
    //                         .arg(args::GeneralArgs::page_size())
    //                         .arg(args::GeneralArgs::pretty()),
    //                 ),
    //         ),
    // );

    let clap = app.get_matches();

    if let Some(subcommand) = clap.subcommand_matches("login") {
        let client_id = String::from(subcommand.value_of("client_id").unwrap());
        let client_secret = String::from(subcommand.value_of("client_secret").unwrap());
        let api_endpoint = String::from(subcommand.value_of("api_endpoint").unwrap());
        let version = String::from(subcommand.value_of("version").unwrap());
        match WTClient::auth(&client_id, &client_secret, &api_endpoint, &version).await {
            Ok(()) => println!("Login successful."),
            Err(e) => println!("Failed: {}", e),
        }
    }

    if let Some(_) = clap.subcommand_matches("test") {
        print!("Connecting ... ");
        let res = WTClient::ping().await;
        match res {
            Ok(pong) => println!("Ok: {}", pong),
            Err(e) => println!("Failed: {}", e),
        }
    }

    if let Some(subcommand) = clap.subcommand_matches("agile") {
        if let Some(subcommand) = subcommand.subcommand_matches("project") {
            if let Some(subcommand) = subcommand.subcommand_matches("get") {
                let id = String::from(subcommand.value_of("id").unwrap());
                let res = WTClient::get_project_by_id(&id).await?;
                json_printer::JSONPrinter::print_by_arg(res, subcommand);
            } else if let Some(subcommand) = subcommand.subcommand_matches("list") {
                let res = WTClient::get_projects(
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
        }
    }

    // if let Some(subcommand) = clap.subcommand_matches("dictionary") {
    //     if let Some(subcommand) = subcommand.subcommand_matches("user") {
    //         if let Some(subcommand) = subcommand.subcommand_matches("get") {
    //             let id = String::from(subcommand.value_of("id").unwrap());
    //             let res = WTClient::get_user_by_id(&id).await?;
    //             json_printer::JSONPrinter::print_by_arg(res, subcommand);
    //         } else if let Some(subcommand) = subcommand.subcommand_matches("list") {
    //             let res = WTClient::get_users(
    //                 subcommand.value_of("name"),
    //                 subcommand.value_of("page_index"),
    //                 subcommand.value_of("page_size"),
    //             )
    //             .await?;
    //             json_printer::JSONPrinter::print_by_arg(res, subcommand);
    //         } else {
    //             println!("{}", subcommand.usage());
    //         }
    //     } else {
    //         println!("{}", subcommand.usage());
    //     }
    // }

    dictionary_area.match_subcommand(&clap);

    Ok(())
}
