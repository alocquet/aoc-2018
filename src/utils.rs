use std::fs::File;
use std::io::prelude::*;

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut result = String::new();
    file.read_to_string(&mut result).expect("something went wrong reading the file");

    result
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn read_a_test_file() {
        let content = read_file("inputs/test.txt");
        assert_eq!(content, "test");
    }

    #[test]
    #[should_panic]
    fn read_file_which_not_exist_should_manic() {
        read_file("inputs/dummy.txt");
    }

}