#[macro_use]
extern crate clap;

use clap::{Arg, SubCommand};
use std::error::Error;

mod args;
mod configure;
mod json_printer;
mod op_executors;
mod wt_client;
mod wt_error;

type AnyError = Box<dyn Error>;

const CLAP_TEMPLATE: &'static str = r"
    ____  _             ______          __   
   / __ \(_)___  ____ _/ ____/___  ____/ /__ 
  / /_/ / / __ \/ __ `/ /   / __ \/ __  / _ \
 / ____/ / / / / /_/ / /___/ /_/ / /_/ /  __/
/_/   /_/_/ /_/\__, /\____/\____/\__,_/\___/ 
              /____/                         

{about}
v{version}
by [{author}]

USAGE:
{usage}

{all-args}";

#[tokio::main]
async fn main() -> Result<(), AnyError> {
    let mut app = app_from_crate!()
        .template(CLAP_TEMPLATE)
        .help_message("Help")
        .version_message("Version");
    app = app.arg(
        Arg::with_name("pretty")
            .long("pretty")
            .required(false)
            .global(true)
            .help("Indicates if the output json result in print pretty format or compact format"),
    );
    app = app.subcommand(
        SubCommand::with_name("login")
            .about("Login PingCode REST API with client id and client secret")
            .arg(
                Arg::with_name("client_id")
                    .short("c")
                    .long("client-id")
                    .help("The Client ID in PingCode REST API application")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("client_secret")
                    .short("s")
                    .long("client-secret")
                    .help("The Client Secret in PingCode REST API application")
                    .takes_value(true)
                    .required(true),
            )
            .arg(
                Arg::with_name("api_endpoint")
                    .short("e")
                    .long("api-endpoint")
                    .help("PingCode REST API endpoint")
                    .takes_value(true)
                    .required(true)
                    .default_value("https://open.worktile.com"),
            )
            .arg(
                Arg::with_name("version")
                    .short("v")
                    .long("version")
                    .help("PingCode REST API version")
                    .takes_value(true)
                    .required(true)
                    .default_value("1"),
            ),
    );
    app = app.subcommand(
        SubCommand::with_name("test")
            .about("Test the connective and verify authentication information"),
    );

    let areas = configure::Configure::load(None)?;
    let commands = configure::Configure::generate_subcommands(&areas);
    let executors = op_executors::OpExecutors::initialize();

    app = app.subcommands(commands);
    let clap = app.get_matches();

    if let Some(subcommand) = clap.subcommand_matches("login") {
        let client_id = String::from(subcommand.value_of("client_id").unwrap());
        let client_secret = String::from(subcommand.value_of("client_secret").unwrap());
        let api_endpoint = String::from(subcommand.value_of("api_endpoint").unwrap());
        let version = String::from(subcommand.value_of("version").unwrap());
        match wt_client::WTClient::auth(&client_id, &client_secret, &api_endpoint, &version).await {
            Ok(()) => println!("Login successful."),
            Err(e) => println!("Failed: {}", e),
        }
    }

    if let Some(_) = clap.subcommand_matches("test") {
        print!("Connecting ... ");
        let res = wt_client::WTClient::ping().await;
        match res {
            Ok(pong) => println!("Ok: {}", pong),
            Err(e) => println!("Failed: {}", e),
        }
    }

    for area in areas.iter() {
        match area.match_subcommand(&clap, &executors) {
            Err(e) => println!("{:#?}", e),
            Ok(_) => {}
        }
    }

    Ok(())
}
