use utils::read_file;
use utils::Point;

pub fn run() -> (usize, usize) {
    let input = &read_file("inputs/day18.txt");
    (compute_result_after_iterations(input, 10), compute_result_after_iterations(input, 1_000_000_000))
}

fn compute_result_after_iterations(input: &str, iterations: usize) -> usize {
    let mut map = Map::new(input);
    let mut seen = vec!();

    let mut index_break = None;
    for i in 1..=iterations {
        seen.push(map.clone());
        let next = map.next_minute();
        if seen.contains(&next) {
            index_break = Some(i);
            map = next;
            break;
        }
        map = next;
    }

    if index_break.is_some() {
        let seen_index = index_break.unwrap();
        let previous_seen_index = seen.iter().enumerate().find(|(_, m)| **m == map).unwrap().0;
        let cycle = seen_index - previous_seen_index;
        map = seen[previous_seen_index + (iterations - previous_seen_index) % cycle].clone();
    }

    map.display();
    map.count('|') * map.count('#')
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Map {
    acres: Vec<Vec<char>>,
}

impl Map {
    fn new(input: &str) -> Map {
        Map { acres: input.lines().map(|line| line.chars().collect()).collect() }
    }
    fn get(&self, point: Point) -> Option<char> {
        if point.x < 0 || point.x >= self.acres[0].len() as isize || point.y < 0 || point.y >= self.acres[0].len() as isize {
            return None;
        }
        Some(self.acres[(point.y) as usize][(point.x) as usize])
    }
    fn set(&mut self, point: Point, val: char) {
        self.acres[point.y as usize][point.x as usize] = val;
    }
    fn count(&self, value: char) -> usize {
        self.acres.iter().fold(0, |acc, line| acc + line.iter().filter(|&&acre| acre == value).count())
    }
    fn count_adjacent_with<F>(&self, point: Point, predicate: F) -> usize where F: Fn(char) -> bool {
        let mut result = 0;
        for x in point.x - 1..=point.x + 1 {
            for y in point.y - 1..=point.y + 1 {
                if (x != point.x || y != point.y)
                    && self.get(Point { x, y }).is_some()
                    && predicate(self.get(Point { x, y }).unwrap()) {
                    result += 1;
                }
            }
        }
        result
    }
    fn next_minute(&self) -> Self {
        let mut next = self.clone();
        for y in 0isize..next.acres.len() as isize {
            for x in 0isize..next.acres.len() as isize {
                let current = Point { x, y };
                let value = self.get(current).unwrap();
                if value == '.' {
                    if self.count_adjacent_with(current, |acre| acre == '|') > 2 {
                        next.set(current, '|');
                    }
                } else if value == '|' {
                    if self.count_adjacent_with(current, |acre| acre == '#') > 2 {
                        next.set(current, '#');
                    }
                } else { // if value == '#'
                    if self.count_adjacent_with(current, |acre| acre == '#') > 0
                        && self.count_adjacent_with(current, |acre| acre == '|') > 0 {
                        next.set(current, '#');
                    } else {
                        next.set(current, '.');
                    }
                }
            }
        }
        next
    }
    fn display(&self) {
        for line in &self.acres {
            println!("{}", line.iter().fold("".to_owned(), |mut acc, cell| {
                acc.push(*cell);
                acc
            }));
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn example_step1() {
        assert_eq!(compute_result_after_iterations(INPUT_EXAMPLE, 10), 1147);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (560091, 202301));
    }
}
