#[macro_use] extern crate lazy_static;
extern crate clap;
extern crate regex;
extern crate rayon;

use clap::{Arg, App};

pub mod utils;
mod algos;

fn main() {
    let matches = App::new("aoc-2018")
        .version("0.1.0")
        .author("Arnaud locquet <alocquet@gmail.com>")
        .about("AoC 2018 written in Rust")
        .arg(Arg::with_name("DAY")
            .short("d")
            .long("day")
            .value_name("day**")
            .required(true)
            .takes_value(true)
            .help("day number"))
        .get_matches();

    let day = matches.value_of("DAY").unwrap();
    algos::run_day(day);
}
