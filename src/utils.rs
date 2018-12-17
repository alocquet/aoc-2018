use std::fs::File;
use std::io::prelude::*;
use std::ops::AddAssign;
use std::ops::Add;
use std::cmp::Ordering;

pub fn read_file(file_name: &str) -> String {
    let mut file = File::open(file_name).expect("file not found");
    let mut result = String::new();
    file.read_to_string(&mut result).expect("something went wrong reading the file");

    result
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl AddAssign for Point {
    fn add_assign(&mut self, other: Point) {
        *self = Point {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

impl Add for Point {
    type Output = Point;
    fn add(self, other: Point) -> Point{
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Self) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub const NORTH: Point = Point { x: 0, y: -1 };
pub const SOUTH: Point = Point { x: 0, y: 1 };
pub const EAST: Point = Point { x: 1, y: 0 };
pub const WEST: Point = Point { x: -1, y: 0 };

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
