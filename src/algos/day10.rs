use utils;
use regex::Regex;
use std::cmp::Ordering;
use std::cmp::PartialOrd;
use std::fmt;
use std::cell::RefCell;

pub fn run() -> (String, usize) {
    run_from_file("inputs/day10.txt")
}


fn run_from_file(file: &str) -> (String, usize) {
    let input = utils::read_file(file);

    let mut board = parse(&input);
    let mut min_height = board.height();

    let mut duration = 0;

    for time in 0.. {
        board.next();
        if board.height() < min_height {
            min_height = board.height();
        } else {
            board.prev();
            duration = time;
            break;
        }
    }
    println!("{}", &board);
    (format!("{}", &board), duration)
}

#[derive(Eq, PartialEq, Clone, Debug)]
struct Point {
    x: isize,
    y: isize,
    velocity_x: isize,
    velocity_y: isize,
}

impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Point) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Point {
    fn cmp(&self, other: &Point) -> Ordering {
        if self.y == other.y {
            self.x.cmp(&other.x)
        } else {
            self.y.cmp(&other.y)
        }
    }
}

impl Point {
    fn next(&mut self) -> &Self {
        self.x += self.velocity_x;
        self.y += self.velocity_y;
        self
    }
    fn prev(&mut self) -> &Self {
        self.x -= self.velocity_x;
        self.y -= self.velocity_y;
        self
    }
}

struct Board {
    points: RefCell<Vec<Point>>
}

impl Board {
    fn height(&self) -> isize {
        self.points.borrow().iter().map(|point| point.y).max().unwrap() - self.points.borrow().iter().map(|point| point.y).min().unwrap()
    }
    fn next(&mut self) -> &Self {
        self.points.borrow_mut().iter_mut().for_each(|p| { p.next(); });
        self
    }
    fn prev(&mut self) -> &Self {
        self.points.borrow_mut().iter_mut().for_each(|p| { p.prev(); });
        self
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut points = self.points.borrow_mut();
        points.sort();
        let min_y = points.iter().map(|point| point.y).min().unwrap();
        let max_y = points.iter().map(|point| point.y).max().unwrap();
        let min_x = points.iter().map(|point| point.x).min().unwrap();
        let max_x = points.iter().map(|point| point.x).max().unwrap();

        let mut points_iterator = points.iter();
        let mut current = points_iterator.next();

        let mut result = String::new();

        for y in 0..=max_y - min_y {
            for x in 0..=max_x - min_x {
                if current.is_some() && current.unwrap().x == min_x + x && current.unwrap().y == min_y + y {
                    result.push('#');
                    current = points_iterator.next();
                    while current.is_some() && current.unwrap().x == min_x + x && current.unwrap().y == min_y + y {
                        current = points_iterator.next();
                    }
                } else {
                    result.push('.');
                }
            }
            result.push('\n');
        }
        writeln!(f, "{}", result)
    }
}

fn parse(input: &str) -> Board {
    let regex = Regex::new(r"^position=<\s*(.*?),\s*(.*?)> velocity=<\s*(.*?),\s*(.*?)>$").unwrap();

    Board {
        points: RefCell::new(input.lines().map(|line| {
            let capture = regex.captures(line).unwrap();
            Point { x: capture[1].parse::<isize>().unwrap(), y: capture[2].parse::<isize>().unwrap(), velocity_x: capture[3].parse::<isize>().unwrap(), velocity_y: capture[4].parse::<isize>().unwrap() }
        }).collect())
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run_from_file("inputs/day10-example.txt"), ("#...#..###\n#...#...#.\n#...#...#.\n#####...#.\n#...#...#.\n#...#...#.\n#...#...#.\n#...#..###\n\n".to_owned(), 3));
    }

    #[test]
    fn input() {
        assert_eq!(run(), (".####...#####......###..#####...#....#..#....#...####...######\n#....#..#....#......#...#....#..##...#..#...#...#....#..#.....\n#.......#....#......#...#....#..##...#..#..#....#.......#.....\n#.......#....#......#...#....#..#.#..#..#.#.....#.......#.....\n#.......#####.......#...#####...#.#..#..##......#.......#####.\n#.......#...........#...#..#....#..#.#..##......#.......#.....\n#.......#...........#...#...#...#..#.#..#.#.....#.......#.....\n#.......#.......#...#...#...#...#...##..#..#....#.......#.....\n#....#..#.......#...#...#....#..#...##..#...#...#....#..#.....\n.####...#........###....#....#..#....#..#....#...####...#.....\n\n".to_owned(), 10345));
    }
}
