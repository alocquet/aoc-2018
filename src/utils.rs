use std::fs::File;
use std::io::prelude::*;

pub fn read_file(input: Option<&str>) -> String {
    let file_name = input.expect("input is needed");

    let mut file = File::open(file_name).expect("file not found");
    let mut result = String::new();
    file.read_to_string(&mut result).expect("something went wrong reading the file");

    result
}