use utils::UPoint;
use std::cmp::max;
use std::collections::HashMap;
use utils::Direction::DOWN;
use utils::Direction::LEFT;
use utils::Direction::RIGHT;
use utils::Direction::UP;

#[cfg_attr(tarpaulin, skip)]
pub fn run() -> (usize, usize) {
    run_from_input(5355, UPoint { x: 14, y: 796 })
}

pub fn run_from_input(depth: usize, target: UPoint) -> (usize, usize) {
    let mut erosion_levels = compute_erosion_levels(depth, target);
    let risk_levels = compute_risk_levels(&erosion_levels);
    (risk_levels.iter().map(|line| line.iter().sum::<usize>()).sum(), compute_path(&mut erosion_levels, depth, target))
}

fn compute_geologic_index(point: UPoint, erosion_levels: &mut Vec<Vec<Option<usize>>>, target: UPoint) -> usize {
    if (point.x == 0 && point.y == 0) || point == target {
        0
    } else if point.y == 0 {
        point.x * 16807
    } else if point.x == 0 {
        point.y * 48271
    } else {
        erosion_levels[(point.y - 1)][(point.x)].expect("previous erosion not computed") * erosion_levels[(point.y)][(point.x - 1)].expect("previous erosion not computed")
    }
}

fn compute_erosion_levels(depth: usize, target: UPoint) -> Vec<Vec<Option<usize>>> {
    let mut erosion_levels = vec!(vec!(None; 1000 + target.x); 100 + target.y);

    for idx in 0..=target.y {
        fill_erosion_level(&mut erosion_levels, idx, depth, target);
    }
    erosion_levels
}

fn fill_erosion_level(erosion_levels: &mut Vec<Vec<Option<usize>>>, idx: usize, depth: usize, target: UPoint) {
    for x in 0..idx {
        fill_one_erosion_level(erosion_levels, x, idx, depth, target);
    }
    for y in 0..idx {
        fill_one_erosion_level(erosion_levels, idx, y, depth, target);
    }
    fill_one_erosion_level(erosion_levels, idx, idx, depth, target);
}

fn fill_one_erosion_level(erosion_levels: &mut Vec<Vec<Option<usize>>>, x: usize, y: usize, depth: usize, target: UPoint) {
    let geologic_index = compute_geologic_index(UPoint { x, y }, erosion_levels, target);
    erosion_levels[y][x] = Some((depth + geologic_index) % 20183);
}

fn compute_risk_levels(erosion_levels: &[Vec<Option<usize>>]) -> Vec<Vec<usize>> {
    erosion_levels.iter().map(|line| line.iter().map(|cell| if cell.is_none() { 0 } else { cell.unwrap() % 3 }).collect()).collect()
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
enum Tool {
    Torch,
    ClimbingGear,
    Neither,
}

impl Tool {
    fn is_valid(&self, risk: usize) -> bool {
        (risk == 0 && (*self == Tool::Torch || *self == Tool::ClimbingGear))
            || (risk == 1 && (*self == Tool::Neither || *self == Tool::ClimbingGear))
            || (risk == 2 && (*self == Tool::Torch || *self == Tool::Neither))
    }
}

#[derive(Debug)]
struct State {
    point: UPoint,
    tool: Tool,
    time: usize,
}

fn compute_path(erosion_levels: &mut Vec<Vec<Option<usize>>>, depth: usize, target: UPoint) -> usize {
    let mut queue = vec!(State { point: UPoint { x: 0, y: 0 }, time: 0, tool: Tool::Torch });
    let mut seen: HashMap<(UPoint, Tool), usize> = HashMap::new();
    while !queue.is_empty() {
        let current = queue.pop().expect("queue should be non empty");

        // println!("{:?}", current);
        if current.point == target {
            return if current.tool == Tool::Torch { current.time } else { current.time + 7 };
        }
        for dir in &[UP, LEFT, RIGHT, DOWN] {
            let next = current.point + *dir;
            if next.is_none() {
                continue;
            }
            let next = next.unwrap();
            let mut next_erosion_level = erosion_levels[next.y][next.x];
            if next_erosion_level.is_none() {
                fill_erosion_level(erosion_levels, max(next.x, next.y), depth, target);
                next_erosion_level = erosion_levels[next.y][next.x];
            }
            let current_risk = erosion_levels[current.point.y][current.point.x].unwrap() % 3;
            let next_risk = next_erosion_level.expect("next erosion level not computed") % 3;
            for next_tool in &[Tool::Neither, Tool::ClimbingGear, Tool::Torch] {
                let next_time = current.time + if *next_tool == current.tool { 1 } else { 8 };
                if (!seen.contains_key(&(next, *next_tool)) || seen[&(next, *next_tool)] > next_time) && next_tool.is_valid(next_risk) && next_tool.is_valid(current_risk) {
                    queue.push(State { point: next, tool: *next_tool, time: next_time });
                    seen.insert((next, *next_tool), next_time);
                }
            }
        }
        queue.sort_by_key(|node| node.time);
        queue.reverse();
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        assert_eq!(run_from_input(510, UPoint { x: 10, y: 10 }), (114, 45));
    }
}
