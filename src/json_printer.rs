pub struct JSONPrinter {}

impl JSONPrinter {
    pub fn print(json: serde_json::Value, pretty: bool) -> () {
        println!("{}", if pretty {
            serde_json::to_string_pretty(&json).unwrap()
        } else {
            json.to_string()
        })
    }

    pub fn print_by_arg(json: serde_json::Value, subcommand: &clap::ArgMatches) -> () {
        JSONPrinter::print(json, subcommand.is_present("pretty"))
    }
}