use clap::Arg;
use clap::ArgMatches;

pub struct GeneralArgs {}

impl GeneralArgs {
    pub fn page_index<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_index")
            .long("page-index")
            .short("p")
            .required(false)
            .takes_value(true)
            .help("Indicates the page index (start from 0)")
    }

    pub fn page_size<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_size")
            .long("page-size")
            .short("s")
            .required(false)
            .takes_value(true)
            .help("Indicates the page size")
    }

    pub fn content<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("content")
            .long("content")
            .short("x")
            .required(true)
            .takes_value(true)
            .conflicts_with("in")
            .help("Content to be created or updated")
    }

    pub fn input<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("in")
            .long("in")
            .short("i")
            .required(true)
            .takes_value(true)
            .conflicts_with("content")
            .help("Content to be created or updated")
    }

    pub fn content_and_input<'a, 'b>() -> std::vec::Vec<Arg<'a, 'b>> {
        vec![GeneralArgs::content(), GeneralArgs::input()]
    }

    // pub fn multiple<'a, 'b>() -> Arg<'a, 'b> {
    //     Arg::with_name("multiple")
    //         .long("multiple")
    //         .short("m")
    //         .required(false)
    //         .takes_value(false)
    //         .help("Indicates the content or input contains multiple value which should be performed in parallel")
    // }
}

pub struct ArgParser {}

impl ArgParser {
    pub fn parse_query<'a>(
        matches: &'a ArgMatches,
        keys: std::vec::Vec<&'a str>,
    ) -> std::vec::Vec<(&'a str, String)> {
        let mut query = std::vec::Vec::<(&str, String)>::new();
        for key in keys.iter() {
            if let Some(value) = matches.value_of(key) {
                query.push((key, String::from(value)));
            }
        }
        query
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
}

pub struct BuildInArgs {}

impl BuildInArgs {
    fn page_index<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_index")
            .long("page-index")
            .short("p")
            .required(false)
            .takes_value(true)
            .help("Indicates the page index (start from 0)")
    }

    fn page_size<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_size")
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
            .help("Content to be created or updated")
    }

    fn input<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("in")
            .long("in")
            .short("i")
            .required(true)
            .takes_value(true)
            .conflicts_with("content")
            .help("Content to be created or updated")
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
        if name == "page_index" {
            vec![BuildInArgs::page_index()]
        } else if name == "page_size" {
            vec![BuildInArgs::page_size()]
        } else if name == "content" {
            vec![BuildInArgs::content()]
        } else if name == "input" {
            vec![BuildInArgs::input()]
        } else if name == "content_and_input" {
            vec![BuildInArgs::content(), BuildInArgs::input()]
        } else {
            vec![]
        }
    }
}
