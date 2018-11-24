use std::fs::File;
use std::io::prelude::*;

pub fn run(input: Option<&str>) {
    let input = input.expect("input is needed");
    println!("Day 01 : {}", input);

    let mut f = File::open(input).expect("file not found");
    let mut input = String::new();
    f.read_to_string(&mut input).expect("something went wrong reading the file");

    println!("Day 01 file content : {}", input);
}