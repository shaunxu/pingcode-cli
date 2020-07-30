#[macro_use]
extern crate clap;
#[macro_use]
extern crate magic_crypt;
#[macro_use]
extern crate log;
extern crate semver;

use clap::{Arg, SubCommand};
use std::error::Error;

mod args;
mod configure;
mod json_printer;
mod op_executors;
mod updater;
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

const PC_CONFIGURE_JSON: &'static str = include_str!("../.pc_configure.json");

fn main() -> Result<(), AnyError> {
    env_logger::init();

    let mut app = app_from_crate!().template(CLAP_TEMPLATE).help_message("Help").version_message("Version");
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
    app = app.subcommand(SubCommand::with_name("test").about("Test the connective and verify authentication information"));
    app = app.subcommand(
        SubCommand::with_name("update")
            .about("Fetch and upgrade to the latest version")
            .arg(Arg::with_name("dry_run").long("dry-run").help("Check the latest version but not update").required(false)),
    );

    let areas = configure::Configure::load(PC_CONFIGURE_JSON)?;
    let commands = configure::Configure::generate_subcommands(&areas);
    let executors = op_executors::OpExecutors::initialize();

    app = app.subcommands(commands);
    let clap = app.get_matches();

    if let Some(subcommand) = clap.subcommand_matches("login") {
        let client_id = String::from(subcommand.value_of("client_id").unwrap());
        let client_secret = String::from(subcommand.value_of("client_secret").unwrap());
        let api_endpoint = String::from(subcommand.value_of("api_endpoint").unwrap());
        let version = String::from(subcommand.value_of("version").unwrap());
        let fut = wt_client::WTClient::auth(&client_id, &client_secret, &api_endpoint, &version);
        match tokio::runtime::Runtime::new()?.block_on(fut) {
            Ok(()) => println!("Login successful."),
            Err(e) => println!("Failed: {}", e),
        }
    }

    if let Some(_) = clap.subcommand_matches("test") {
        print!("Connecting ... ");
        let fut = wt_client::WTClient::ping();
        match tokio::runtime::Runtime::new()?.block_on(fut) {
            Ok(pong) => println!("Ok: {}", pong),
            Err(e) => println!("Failed: {}", e),
        }
    }

    if let Some(subcommand) = clap.subcommand_matches("update") {
        let fut = updater::Updater::update(subcommand.is_present("dry_run"));
        if let Err(e) = tokio::runtime::Runtime::new()?.block_on(fut) {
            println!("Failed: {}", e);
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
