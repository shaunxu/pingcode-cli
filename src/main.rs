#[macro_use]
extern crate clap;
extern crate semver;

use clap::{Arg, SubCommand};
use std::error::Error;

mod args;
mod configure;
mod json_printer;
mod op_executors;
mod wt_client;
mod wt_error;
mod updater;

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
    app = app.subcommand(
        SubCommand::with_name("update")
            .about("Fetch and upgrade to the latest version")
            .arg(
                Arg::with_name("dry_run")
                    .long("dry-run")
                    .help("Check the latest version but not update")
                    .required(false),
            ),
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

        // if let Some((platform, bin_name)) = match std::env::consts::OS {
        //     "linux" => Some(("linux", "pc")),
        //     "macos" => Some(("darwin", "pc")),
        //     "windows" => Some(("win", "pc.exe")),
        //     _ => None,
        // } {
        //     let tmp_tarball_path = format!("./pc-{}-x64.tar.gz", platform);
        //     let tmp_tarball = std::fs::OpenOptions::new()
        //         .read(true)
        //         .write(true)
        //         .create(true)
        //         .open(&tmp_tarball_path)?;
            
        //     let fut = reqwest::Client::new().request(reqwest::Method::GET, "https://api.github.com/repos/shaunxu/pingcode-cli/releases/assets/22968110").send();
        //     if let Ok(res) = tokio::runtime::Runtime::new()?.block_on(fut) {

        //     }

        //     self_update::Download::from_url(
        //         "https://api.github.com/repos/shaunxu/pingcode-cli/releases/assets/22968110",
        //     )
        //     .show_progress(true)
        //     .download_to(&tmp_tarball)?;

        // let data = std::fs::File::open(&tmp_tarball_path)?;
        // let decompressed = flate2::read::GzDecoder::new(data);
        // let mut archive = tar::Archive::new(decompressed);
        // for file in archive.entries().unwrap() {
        //     let file = file.unwrap();
        //     println!("{:?}", file.header().path().unwrap());
        //     println!("{}", file.header().size().unwrap());
        // }

        //     let releases = self_update::backends::github::ReleaseList::configure()
        //         .repo_owner("shaunxu")
        //         .repo_name("pingcode-cli")
        //         .build()?
        //         .fetch()?;
        //     if let Some(latest_rel) = releases.first() {
        //         let current_version = semver::Version::parse(crate_version!())?;
        //         let latest_version = semver::Version::parse(&latest_rel.version)?;
        //         if latest_version > current_version {
        //             println!(
        //                 "Latest version found v{}, while you have v{} installed",
        //                 latest_version, current_version
        //             );
        //             if !subcommand.is_present("dry_run") {
        //                 let asset_name = format!("pc-{}-x64.tar.gz", platform);
        //                 if let Some(asset) = latest_rel.asset_for(&asset_name) {
        //                     let tmp_dir = tempfile::Builder::new()
        //                         .prefix("pc_update_")
        //                         .tempdir_in(std::env::current_dir()?)?;
        //                     let tmp_tarball_path = tmp_dir.path().join(&asset.name);
        //                     let tmp_tarball = std::fs::OpenOptions::new()
        //                         .read(true)
        //                         .write(true)
        //                         .create(true)
        //                         .open(&tmp_tarball_path)?;
        //                     println!("{:#?}", asset);

        //                     // self_update::Download::from_url(&asset.download_url)
        //                     //     .show_progress(true)
        //                     //     .download_to(&tmp_tarball)?;
        //                     // // let bin_name = std::path::PathBuf::from(bin_name);
        //                     // println!("tmp_tarball_path = {}", tmp_tarball_path.to_str().unwrap());
        //                     // println!("tmp_dir = {}", tmp_dir.path().to_str().unwrap());
        //                     // println!("bin_name = {}", bin_name);

        //                     // let data = std::fs::File::open(&tmp_tarball_path)?;
        //                     // let decompressed = flate2::read::GzDecoder::new(data);
        //                     // let mut archive = tar::Archive::new(decompressed);
        //                     // for file in archive.entries().unwrap() {
        //                     //     let file = file.unwrap();
        //                     //     println!("{:?}", file.header().path().unwrap());
        //                     //     println!("{}", file.header().size().unwrap());
        //                     // }
        //                 // if let Some(entry) = archive.entries()?.find(|x| x.as_ref().unwrap().header().path().unwrap().to_str().unwrap() == bin_name) {
        //                 //     println!("unpacking");
        //                 //     entry?.unpack_in(tmp_dir.path().to_str().unwrap()).unwrap();
        //                 //     println!("unpacked");
        //                 // }

        //                 // self_update::Extract::from_source(&tmp_tarball_path)
        //                 //     .archive(self_update::ArchiveKind::Tar(Some(
        //                 //         self_update::Compression::Gz,
        //                 //     )))
        //                 //     .extract_file(&tmp_dir.path(), &bin_name)?;
        //                 // let tmp_file = tmp_dir.path().join("replacement_tmp");
        //                 // let bin_path = tmp_dir.path().join(bin_name);

        //                 // println!("bin_path = {}", bin_path.to_str().unwrap());
        //                 // println!("tmp_file = {}", tmp_file.to_str().unwrap());
        //                 // println!(
        //                 //     "current_exe = {}",
        //                 //     &std::env::current_exe()?.to_str().unwrap()
        //                 // );

        //                 // self_update::Move::from_source(&bin_path)
        //                 //     .replace_using_temp(&tmp_file)
        //                 //     .to_dest(&std::env::current_exe()?)?;
        //                 } else {
        //                     println!("Cannot find asset by name {}", asset_name);
        //                 }
        //             }
        //         } else {
        //             println!("You are running the latest version, enjoy. (latest version = {}, running version = {})", latest_version, current_version);
        //         }
        //     } else {
        //         println!("No releases available");
        //     }
        // } else {
        //     println!("Invalid operation system found");
        // }
    }

    for area in areas.iter() {
        match area.match_subcommand(&clap, &executors) {
            Err(e) => println!("{:#?}", e),
            Ok(_) => {}
        }
    }

    Ok(())
}
