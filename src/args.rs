use clap::{Arg};

pub struct GeneralArgs {}

impl GeneralArgs {
    pub fn pretty<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("pretty")
            .long("pretty")
            .required(false)
            .help("Indicates if the output json result in print pretty format or compact format")
    }

    pub fn page_index<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_index")
            .long("page-index")
            .required(false)
            .takes_value(true)
            .help("Indicates the page index (start from 0)")
    }

    pub fn page_size<'a, 'b>() -> Arg<'a, 'b> {
        Arg::with_name("page_size")
            .long("page-size")
            .required(false)
            .takes_value(true)
            .help("Indicates the page size")
    }
}
