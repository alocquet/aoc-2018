use std::collections::HashMap;
use utils::Point;
use utils::EAST;
use utils::WEST;
use utils::NORTH;
use utils::SOUTH;
use std::collections::VecDeque;
use std::collections::HashSet;
use std::str::Chars;
use utils::read_file;

pub fn run() -> (usize, usize) {
    let input = &read_file("inputs/day20.txt");
    let mut cells = HashMap::new();
    cells.insert(Point { x: 0, y: 0 }, 'X');
    fill_map(&mut input.chars(), Point { x: 0, y: 0 }, &mut cells);
    find_all_path(&cells)
}

fn fill_map(path_iter: &mut Chars, start: Point, cells: &mut HashMap<Point, char>) -> Vec<Point> {
    let mut ends = vec!();
    let mut positions = HashSet::new();
    positions.insert(start);

    while let Some(car) = path_iter.next() {
        let mut next_positions = HashSet::new();
        if car == ')' {
            break;
        }
        for position in positions {
            match car {
                '(' => {
                    // recursive call && add next positions
                    fill_map(path_iter, position, cells)
                        .iter().for_each(|next| { next_positions.insert(*next); });
                }
                '|' => {
                    ends.push(position);
                    // restart from positions
                    next_positions = HashSet::new();
                    next_positions.insert(start);
                }
                'E' | 'W' | 'S' | 'N' => {
                    let mut next = get_position(position, car);
                    if car == 'S' || car == 'N' {
                        cells.insert(next, '-');
                    } else {
                        cells.insert(next, '|');
                    }
                    next = get_position(next, car);
                    cells.insert(next, '.');
                    next_positions.insert(next);
                }
                _ => panic!("unrecognized caracter")
            }
        }
        positions = next_positions;
    }
    for p in positions {
        ends.push(p);
    }
    ends
}

fn get_position(point: Point, dir: char) -> Point {
    match dir {
        'E' => point + EAST,
        'W' => point + WEST,
        'N' => point + NORTH,
        'S' => point + SOUTH,
        _ => panic!("unrecognized direction")
    }
}

fn find_all_path(cells: &HashMap<Point, char>) -> (usize, usize) {
    let mut visited = vec!();
    let mut queue = VecDeque::new();
    queue.push_back(vec!(Point { x: 0, y: 0 }));
    let mut step1 = 0;
    let mut step2 = 0;
    while !queue.is_empty() {
        let current = queue.pop_front().unwrap();
        visited.push(*current.last().unwrap());

        for direction in &[NORTH, SOUTH, EAST, WEST] {
            let next = *current.last().unwrap() + *direction;
            let next_val = cells.get(&next).unwrap_or(&'#');
            if *next_val == '|' || *next_val == '-' {
                let next = next + *direction;
                if !visited.contains(&next) {
                    let mut next_path = current.clone();
                    next_path.push(next);
                    if next_path.len() > step1 {
                        step1 = next_path.len();
                    }
                    if next_path.len() > 1000 {
                        step2 += 1;
                    }
                    queue.push_back(next_path);
                }
            }
        }
    }

    (step1 - 1, step2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(run(), (4155, 8434));
    }
}
