extern crate clap;
extern crate failure;

use clap::{App, Arg};

mod days;

fn main() -> Result<(), failure::Error> {
    let matches = App::new("Advent of Code 2019")
        .version("0.1.0")
        .author("Avery Harnish <averyharnish@gmail.com>")
        .about("Teaches argument parsing")
        .arg(
            Arg::with_name("day")
                .required(true)
                .takes_value(true)
                .index(1)
                .help("Day of challenge"),
        )
        .get_matches();

    let day = matches.value_of("day").unwrap();

    days::run(day)
}
