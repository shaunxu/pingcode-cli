#[macro_use]
extern crate clap;

use clap::{Arg, ArgGroup, SubCommand};
use std::error::Error;

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
        .arg(
            Arg::with_name("api-endpoint")
                .short("e")
                .long("api-endpoint")
                .help("Worktile REST API endpoint")
                .takes_value(true)
                .required(true)
                .default_value("https://open.worktile.com")
                .display_order(1),
        )
        .arg(
            Arg::with_name("client_id")
                .short("c")
                .long("client-id")
                .help("The Client ID in Worktile REST API application")
                .takes_value(true)
                .required(true)
                .display_order(2),
        )
        .arg(
            Arg::with_name("client_secret")
                .short("s")
                .long("client-secret")
                .help("The Client Secret in Worktile REST API application")
                .takes_value(true)
                .required(true)
                .display_order(3),
        )
        .arg(
            Arg::with_name("content")
                .short("x")
                .long("content")
                .help("Message content send to Worktile REST API in JSON format")
                .takes_value(true)
                .display_order(4),
        )
        .subcommand(
            SubCommand::with_name("test")
                .about("Test the connective and verify client_id/client_secret"),
        )
        .subcommand(
            SubCommand::with_name("devops")
                .about("Manage DevOps information which displayed in Agile workitem dialog")
                .subcommand(
                    SubCommand::with_name("scm")
                        .arg(
                            Arg::with_name("create")
                                .short("c")
                                .long("create")
                                .help("Create a new SCM product")
                                .display_order(1),
                        )
                        .arg(
                            Arg::with_name("update")
                                .short("u")
                                .long("update")
                                .help("Update an existing SCM product")
                                .display_order(2),
                        )
                        .arg(
                            Arg::with_name("get")
                                .short("g")
                                .long("get")
                                .help("Get one SCM product by its ID")
                                .display_order(3),
                        )
                        .arg(
                            Arg::with_name("list")
                                .short("l")
                                .long("list")
                                .help("List all SCM products")
                                .display_order(4),
                        )
                        .group(
                            ArgGroup::with_name("action")
                                .args(&["create", "update", "get", "list"]),
                        ),
                )
                .subcommand(SubCommand::with_name("user"))
                .subcommand(SubCommand::with_name("repo"))
                .subcommand(SubCommand::with_name("commit"))
                .subcommand(SubCommand::with_name("branch"))
                .subcommand(SubCommand::with_name("pr")),
        )
        .get_matches();

    let api_endpoint = String::from(clap.value_of("api-endpoint").unwrap());
    let client_id = String::from(clap.value_of("client_id").unwrap());
    let client_secret = String::from(clap.value_of("client_secret").unwrap());
    let mut client = wt_client::WTClient::new(
        api_endpoint.clone(),
        client_id.clone(),
        client_secret.clone(),
    );

    if let Some(_) = clap.subcommand_matches("test") {
        print!("Connecting {} ... ", api_endpoint);
        let res = client.ping().await;
        match res {
            Ok(pong) => println!("Ok: {}", pong),
            Err(e) => println!("Failed: {}", e),
        }
    } else {
        println!("{}", clap.usage());
    };

    Ok(())
}
