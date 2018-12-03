use regex::*;
use std::cmp;
use std::collections::HashSet;
use utils;

pub fn run() -> (usize, usize) {
    // parse input
    let input = utils::read_file("inputs/day03.txt");

    let claims: Vec<&str> = input.lines().collect();
    execute_from_input(&claims)
}

fn execute_from_input(input: &[&str]) -> (usize, usize) {
    let mut claims: Vec<Square> = input.iter().map(|area| Square::from_str(&area)).collect();
    claims.sort_by(|a1, a2| a1.start.x.cmp(&a2.start.x));

    let mut overlaps = HashSet::new();
    let mut claim_in_overlap = vec!(false; claims.len());
    for (idx1, mut claim1) in claims.iter().enumerate() {
        for (_idx2, mut claim2) in claims[idx1 + 1..].iter().enumerate() {
            if claim1.end.x < claim2.start.x {
                break;
            }
            for overlap in compute_overlaps(&claim1, &claim2) {
                claim_in_overlap[claim1.n as usize - 1] = true;
                claim_in_overlap[claim2.n as usize - 1] = true;
                overlaps.insert(overlap);
            }
        }
    }
    let not_in_overlap = 1 + claim_in_overlap.iter().position(|in_overlap| !in_overlap).unwrap();

    (overlaps.len(), not_in_overlap)
}

fn compute_overlaps(s1: &Square, s2: &Square) -> Vec<Point> {
    let start = Point { x: cmp::max(s1.start.x, s2.start.x), y: cmp::max(s1.start.y, s2.start.y) };
    let end = Point { x: cmp::min(s1.end.x, s2.end.x), y: cmp::min(s1.end.y, s2.end.y) };

    let mut overlaps = vec!();

    if start.x <= end.x && start.y <= end.y {
        for idx in start.x..=end.x {
            for idy in start.y..=end.y {
                overlaps.push(Point { x: idx, y: idy });
            }
        }
    }

    overlaps
}


#[derive(Hash, Eq, PartialEq, Debug)]
struct Point {
    x: i16,
    y: i16,
}

#[derive(Debug)]
struct Square {
    n: i16,
    start: Point,
    end: Point,
}

impl Square {
    #[cfg_attr(tarpaulin, skip)]
    fn parse(value: &str) -> Captures {
        lazy_static! {
            static ref regex: Regex = Regex::new(r"^#(\d+?) @ (\d+?),(\d+?): (\d+?)x(\d+?)$").unwrap();
        }
        regex.captures(value).unwrap()
    }

    fn from_str(value: &str) -> Square {
        let area = Square::parse(value);
        Square {
            n: area[1].parse::<i16>().unwrap(),
            start: Point { x: area[2].parse::<i16>().unwrap(), y: area[3].parse::<i16>().unwrap() },
            end: Point { x: area[2].parse::<i16>().unwrap() + area[4].parse::<i16>().unwrap() - 1, y: area[3].parse::<i16>().unwrap() + area[5].parse::<i16>().unwrap() - 1 },
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let square = Square::from_str("#1 @ 1,3: 4x4");
        assert_eq!(square.n, 1);
        assert_eq!(square.start.x, 1);
        assert_eq!(square.start.y, 3);
        assert_eq!(square.end.x, 4);
        assert_eq!(square.end.y, 6);
    }

    #[test]
    fn example() {
        assert_eq!(execute_from_input(&vec!("#1 @ 1,3: 4x4", "#2 @ 3,1: 4x4", "#3 @ 5,5: 2x2")), (4, 3));
    }

    #[test]
    fn input() {
        assert_eq!(run(), (121259, 239));
    }
}