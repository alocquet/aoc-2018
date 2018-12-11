use std::cmp;

// skip coverage because it is too slow in debug mode
#[cfg_attr(tarpaulin, skip)]
pub fn run() -> ((usize, usize), ((usize, usize), usize)) {
    (compute_max_total_power(8561), compute_max_total_power_all_square_size(8561))
}

// skip coverage because it is too slow in debug mode
#[cfg_attr(tarpaulin, skip)]
fn compute_max_total_power_all_square_size(grid_sn: usize) -> ((usize, usize), usize) {
    let levels = compute_power_levels_matrix(grid_sn);
    let mut max = 0;
    let mut max_coord = (0, 0);
    let mut max_square_size = 0;
    for x in 0..299 {
        for y in 0..299 {
            let square_size = 300 - cmp::max(x, y);
            let mut sum = 0;
            for i in 0..square_size {
                for xi in 0..i {
                    sum += levels[x + xi][y + i];
                }
                for yi in 0..i {
                    sum += levels[x + i][y + yi];
                }
                sum += levels[x + i][y + i];
                if sum > max {
                    max = sum;
                    max_coord = (x + 1, y + 1);
                    max_square_size = i + 1;
                }
            }
        }
    }

    (max_coord, max_square_size)
}

fn compute_max_total_power(grid_sn: usize) -> (usize, usize) {
    let levels = compute_power_levels_matrix(grid_sn);
    let mut max = 0;
    let mut max_coord = (0, 0);
    for x in 0..297 {
        for y in 0..297 {
            let mut sum = 0;
            for xi in 0..3 {
                for yi in 0..3 {
                    sum += levels[x + xi][y + yi];
                }
            }
            if sum > max {
                max = sum;
                max_coord = (x + 1, y + 1);
            }
        }
    }
    max_coord
}

fn compute_power_levels_matrix(grid_sn: usize) -> Vec<Vec<isize>> {
    (0..300).map(|x| (0..300).map(|y|compute_power_level((x + 1, y + 1), grid_sn)).collect()).collect()
}

fn compute_power_level((x, y): (usize, usize), grid_sn: usize) -> isize {
    let rack_id = x + 10;
    let power_level = (rack_id * y + grid_sn) * rack_id;
    let power_level = (power_level / 100) % 10;
    power_level as isize - 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_example_power_levels() {
        assert_eq!(compute_power_level((3, 5), 8), 4);
        assert_eq!(compute_power_level((122, 79), 57), -5);
        assert_eq!(compute_power_level((217, 196), 39), 0);
        assert_eq!(compute_power_level((101, 153), 71), 4);
    }

    #[test]
    fn compute_example_max_total_power() {
        assert_eq!(compute_max_total_power(18), (33, 45));
        assert_eq!(compute_max_total_power(42), (21, 61));
    }

    #[test]
    #[ignore]
    fn compute_example_max_total_power_for_all_square_size() {
        assert_eq!(compute_max_total_power_all_square_size(18), ((90, 269), 16));
        assert_eq!(compute_max_total_power_all_square_size(42), ((232, 251), 12));
    }
}
