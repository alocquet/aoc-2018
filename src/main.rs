extern crate clap;

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
        .arg(Arg::with_name("INPUT")
            .short("i")
            .long("input")
            .value_name("input")
            .required(false)
            .takes_value(true)
            .help("input"))
        .get_matches();

    let day = matches.value_of("DAY").unwrap();
    let input = matches.value_of("INPUT");
    algos::run_day(day, input);
}
