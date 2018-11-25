use utils;

pub fn run(input: Option<&str>) {
    let input = utils::read_file(input);
    println!("Day 01 file content : {}", input);
    println!("Day 01 content is empty : {}", is_empty(&input));
}

pub fn is_empty(value: &str) -> bool {
    value.is_empty()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_empty() {
        assert_eq!(is_empty("test"), false);
    }

    #[test]
    fn test_not_empty() {
        assert_eq!(is_empty(""), true);
    }

}