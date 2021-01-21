extern crate clap;

use clap::{App, Arg};

pub struct CLI;

impl CLI {
    pub fn new<'a>() -> clap::ArgMatches<'a> {
        let arg1: Arg = Arg::with_name("config")
            .short("c")
            .long("config")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true);

        App::new("Blockshain client interface")
            .version("0.1")
            .author("Rached MEJRI <r.mejri74100@gmail.com")
            .about("")
            .arg(arg1)
            .get_matches()
    }
}
