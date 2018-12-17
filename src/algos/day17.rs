use regex::Regex;
use utils::Point;
use utils::SOUTH;
use utils::WEST;
use utils::EAST;
use utils::read_file;
use utils::NORTH;
use std::collections::VecDeque;

pub fn run() -> (usize, usize) {
    let input = &read_file("inputs/day17.txt");
    let mut map = parse(input);
    count_tiles_reached_by_water(&mut map)
}

fn count_tiles_reached_by_water(map: &mut Map) -> (usize, usize) {
    let point = Point { x: 500, y: map.min.y };

    let mut already_done = vec!();
    let mut todos = VecDeque::new();
    todos.push_back(point);
    already_done.push(point);

    let mut i = 0;

    while !todos.is_empty() {
        i += 1;
        if i > 20 {
          //  println!("{:?}", todos.pop_front().unwrap());
        //    break;
        }
        let mut current = todos.pop_front().unwrap();
        while map.is_sand(current).unwrap_or(false) || map.is_falling_water(current).unwrap_or(false) {
            map.set(current, '|');
            current += SOUTH;
        }
        if !map.is_sand(current).unwrap_or(true) {
            let mut move_up = true;
            while move_up {
                // reach clay..., check if water is retain or not
                current += NORTH;
                let limits = get_limits_horizontally(current, map);
                if map.is_clay(limits[0]).unwrap_or(false) && map.is_clay(limits[1]).unwrap_or(false) {
                    for x in limits[0].x + 1..limits[1].x {
                        map.set(Point { x, y: current.y }, '~');
                    }
                    // go up
                    move_up = true;
                } else {
                    for x in limits[0].x + 1..limits[1].x {
                        map.set(Point { x, y: current.y }, '|');
                    }
                    // add to queue
                    if !map.is_clay(limits[0]).unwrap_or(true) && !already_done.contains(&limits[0]) {
                        todos.push_back(limits[0]);
                        already_done.push(limits[0]);
                    }
                    if !map.is_clay(limits[1]).unwrap_or(true) && !already_done.contains(&limits[1]) {
                        todos.push_back(limits[1]);
                        already_done.push(limits[1]);
                    }
                    move_up = false;
                }
            }
        }
    }

    map.display();

    let not_at_rest = map.coords.iter().fold(0, |acc, line| acc + line.iter().filter(|&&cell| cell == '|').count());
    let at_rest = map.coords.iter().fold(0, |acc, line| acc + line.iter().filter(|&&cell| cell == '~').count());

    (not_at_rest + at_rest, at_rest)
}

fn get_limits_horizontally(coord: Point, map: &Map) -> Vec<Point> {
    let mut limits = vec!();
    let mut position = coord;
    loop {
        position += WEST;
        if map.is_clay(position).unwrap_or(true) || map.is_sand(position + SOUTH).unwrap_or(true) || map.is_falling_water(position + SOUTH).unwrap_or(true) {
            limits.push(position);
            break;
        }
    }
    let mut position = coord;
    loop {
        position += EAST;
        if map.is_clay(position).unwrap_or(true) || map.is_sand(position + SOUTH).unwrap_or(true)  || map.is_falling_water(position + SOUTH).unwrap_or(true) {
            limits.push(position);
            break;
        }
    }
    limits
}

struct Map {
    coords: Vec<Vec<char>>,
    min: Point,
    max: Point,
}

impl Map {
    fn get(&self, point: Point) -> Option<char> {
        if point.x < self.min.x || point.x > self.max.x+1 || point.y < self.min.y || point.y > self.max.y {
            return None;
        }
        Some(self.coords[(point.y - self.min.y) as usize][(point.x - self.min.x) as usize])
    }
    fn set(&mut self, point: Point, val: char) {
        self.coords[(point.y - self.min.y) as usize][(point.x - self.min.x) as usize] = val;
    }
    fn is_clay(&self, point: Point) -> Option<bool> {
        self.get(point).and_then(|t| Some(t == '#'))
    }
    fn is_sand(&self, point: Point) -> Option<bool> {
        self.get(point).and_then(|t| Some(t == '.'))
    }
    fn is_falling_water(&self, point: Point) -> Option<bool> {
        self.get(point).and_then(|t| Some(t == '|'))
    }
    fn display(&self) {
        for line in &self.coords {
            println!("{}", line.iter().fold("".to_owned(), |mut acc, cell| {
                acc.push(*cell);
                acc
            }));
        }
    }
}

#[allow(clippy::if_same_then_else)]
fn parse(input: &str) -> Map {
    let mut clays = vec!();
    let regex = Regex::new(r"^(x|y)=(\d+), (x|y)=(\d+)\.\.(\d+)").expect("bad regex format");
    for line in input.lines() {
        let capture = regex.captures(line).unwrap_or_else(|| panic!("regex error : line is not properly formated : {}", line));
        if &capture[1] == "x" {
            let x = capture[2].parse::<isize>().unwrap();
            for y in capture[4].parse::<isize>().unwrap()..=capture[5].parse::<isize>().unwrap() {
                clays.push(Point { x, y });
            }
        } else {
            let y = capture[2].parse::<isize>().unwrap();
            for x in capture[4].parse::<isize>().unwrap()..=capture[5].parse::<isize>().unwrap() {
                clays.push(Point { x, y });
            }
        }
    }
    let min_x = clays.iter().min_by_key(|clay| clay.x).expect("must have a min").x;
    let max_x = clays.iter().max_by_key(|clay| clay.x).expect("must have a max").x;
    let min_y = clays.iter().min_by_key(|clay| clay.y).expect("must have a min").y;
    let max_y = clays.iter().max_by_key(|clay| clay.y).expect("must have a max").y;

    let mut map = Map { coords: vec!(vec!('.'; (max_x - min_x + 2) as usize); (max_y - min_y + 1) as usize), min: Point { x: min_x, y: min_y }, max: Point { x: max_x, y: max_y } };
    for clay in clays {
        map.set(clay, '#');
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504";

    #[test]
    fn example_step1() {
        let mut map = parse(INPUT_EXAMPLE);
        assert_eq!(count_tiles_reached_by_water(&mut map), (57, 29));
    }

    #[test]
    fn input() {
        assert_eq!(run(), (27736, 22474));
    }
}
