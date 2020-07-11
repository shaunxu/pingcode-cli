use crate::configure::OpContext;
use crate::wt_error::WTError;
use crate::AnyError;
use clap::Arg;
use clap::ArgMatches;

pub struct ArgParser {}

impl ArgParser {
    pub fn parse_query(
        matches: &ArgMatches,
        keys: std::vec::Vec<String>,
    ) -> std::vec::Vec<(String, String)> {
        let mut query = std::vec::Vec::<(String, String)>::new();
        for key in keys.iter() {
            if let Some(value) = matches.value_of(key) {
                query.push((key.replace("-", "_"), String::from(value)));
            }
        }
        query
    }

    pub fn parse_query_from_args(
        matches: &ArgMatches,
        ctx: &OpContext,
    ) -> std::vec::Vec<(String, String)> {
        ArgParser::parse_query(matches, ctx.arg_names.clone())
    }

    pub fn parse_content(matches: &ArgMatches) -> Option<String> {
        if let Some(raw) = matches.value_of("content") {
            Some(String::from(raw))
        } else if let Some(path) = matches.value_of("in") {
            Some(std::fs::read_to_string(path).unwrap())
        } else {
            // never happened since either "--content" and "--in" must be specified
            None
        }
    }

    pub fn parse_content_to_json(
        matches: &ArgMatches,
    ) -> Result<Option<serde_json::Value>, AnyError> {
        if let Some(raw) = ArgParser::parse_content(matches) {
            match serde_json::from_str(&raw) {
                Ok(json) => Ok(Some(json)),
                Err(e) => Err(WTError::new_boxed(
                    "000000",
                    &format!("Failed to parse content in JSON format. {}", e),
                )),
            }
        } else {
            Ok(Some(serde_json::Value::default()))
        }
    }
}

pub struct BuildInArgs {}

impl BuildInArgs {
    fn page_index<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page-index")
            .long("page-index")
            .short("p")
            .required(false)
            .takes_value(true)
            .help("Indicates the page index (start from 0)")
    }

    fn page_size<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page-size")
            .long("page-size")
            .short("s")
            .required(false)
            .takes_value(true)
            .help("Indicates the page size")
    }

    fn content<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("content")
            .long("content")
            .short("x")
            .required(true)
            .takes_value(true)
            .conflicts_with("in")
            .help("JSON-formated string for content to be created or updated")
    }

    fn input<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("in")
            .long("in")
            .short("i")
            .required(true)
            .takes_value(true)
            .conflicts_with("content")
            .help("Input file for JSON-formated content to be created or updated")
    }

    // fn multiple<'a, 'b>() -> Arg<'a, 'b> {
    //     Arg::with_name("multiple")
    //         .long("multiple")
    //         .short("m")
    //         .required(false)
    //         .takes_value(false)
    //         .help("Indicates the content or input contains multiple value which should be performed in parallel")
    // }

    pub fn get<'a, 'b>(name: &String) -> std::vec::Vec<Arg<'a, 'b>> {
        if name == "page-index" {
            vec![BuildInArgs::page_index()]
        } else if name == "page-size" {
            vec![BuildInArgs::page_size()]
        } else if name == "content" {
            vec![BuildInArgs::content()]
        } else if name == "input" {
            vec![BuildInArgs::input()]
        } else if name == "content-and-input" {
            vec![BuildInArgs::content(), BuildInArgs::input()]
        } else {
            vec![]
        }
    }
}
