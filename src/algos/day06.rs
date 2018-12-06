use std::cmp;
use utils;

pub fn run() -> (usize, usize) {
    let input = utils::read_file("inputs/day06.txt");
    let coordinates = parse(&input);

    (compute_max_distance(&coordinates), compute_region_size(&coordinates, 10000))
}

fn compute_max_distance(coordinates: &[Coordinate]) -> usize {
    let (min, max) = compute_boundaries(&coordinates);

    let areas = compute_areas(&coordinates, (&min, &max));

    // retrieve counts and endless_area
    let mut endless_area = vec!(1; coordinates.len());
    let mut counts = vec!(0; coordinates.len());
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let coord = areas[(x - min.x)][(y - min.y)];
            if coord.is_some() {
                counts[coord.unwrap()] += 1;
                if x == min.x || x == max.x || y == min.y || y == max.y {
                    endless_area[coord.unwrap()] = 0;
                }
            }
        }
    }

    let mut result = 0;
    for (idx, count) in counts.iter().enumerate() {
        result = cmp::max(result, (*count) * endless_area[idx]);
    }

    result
}

fn compute_areas(coordinates: &[Coordinate], (min, max): (&Coordinate, &Coordinate)) -> Vec<Vec<Option<usize>>> {
    let mut areas = vec!(vec!(None; max.y + 1 - min.y); max.x + 1 - min.x);

    // compute areas
    for x in min.x..=max.x {
        for y in min.y..=max.y {
            let mut min_dist_node = 0;
            let mut min_dist = 0;
            let mut double = false;

            for (idx, coord) in coordinates.iter().enumerate() {
                let dist = compute_distance_between(coord.x as i32, coord.y as i32, x as i32, y as i32);
                if idx == 0 || dist < min_dist {
                    min_dist_node = idx;
                    min_dist = dist;
                    double = false;
                } else if dist == min_dist {
                    double = true;
                }
            }
            if !double {
                areas[(x - min.x)][(y - min.y)] = Some(min_dist_node);
            }
        }
    }
    areas
}

fn compute_region_size(coordinates: &[Coordinate], max_region_size: usize) -> usize {
    let (min, max) = compute_boundaries(&coordinates);

    let max_dist_from_boundaries = max_region_size / coordinates.len();

    let mut count = 0;
    for x in (min.x as i32 - max_dist_from_boundaries as i32 - 1)..=(max.x as i32 + max_dist_from_boundaries as i32 + 1) {
        for y in (min.y as i32 - max_dist_from_boundaries as i32 - 1)..=(max.y as i32 + max_dist_from_boundaries as i32 + 1) {
            let mut sum: usize = coordinates
                .iter()
                .map(|coord| compute_distance_between(coord.x as i32, coord.y as i32, x, y))
                .sum();
            if sum < max_region_size {
                count += 1;
            }
        }
    }

    count
}

struct Coordinate {
    x: usize,
    y: usize,
}

fn parse(input: &str) -> Vec<Coordinate> {
    let mut coordinates = vec!();
    for line in input.lines() {
        let mut coord = line.split(", ");
        coordinates.push(Coordinate { x: coord.next().unwrap().parse::<usize>().unwrap(), y: coord.next().unwrap().parse::<usize>().unwrap() });
    }
    coordinates
}

fn compute_boundaries(coordinates: &[Coordinate]) -> (Coordinate, Coordinate) {
    let mut min = Coordinate { x: 9999, y: 9999 };
    let mut max = Coordinate { x: 0, y: 0 };

    for coord in coordinates {
        min.x = cmp::min(min.x, coord.x);
        max.x = cmp::max(max.x, coord.x);
        min.y = cmp::min(min.y, coord.y);
        max.y = cmp::max(max.y, coord.y);
    }

    (min, max)
}

fn compute_distance_between(xa: i32, ya: i32, xb: i32, yb: i32) -> usize {
    let result = if xa < xb { xb - xa } else { xa - xb } + if ya < yb { yb - ya } else { ya - yb };
    result as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_EXAMPLE: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9";

    #[test]
    fn example_step1() {
        assert_eq!(compute_max_distance(&parse(INPUT_EXAMPLE)), 17);
    }

    #[test]
    fn example_step2() {
        assert_eq!(compute_region_size(&parse(INPUT_EXAMPLE), 32), 16);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (3890, 40284));
    }
}
