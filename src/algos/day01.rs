use std::collections::HashSet;
use utils;

pub fn run() -> (i32, i32) {
    // parse input
    let input = utils::read_file("inputs/day01.txt");
    let deltas = input.lines().map(|s| s.replace("+", "").parse::<i32>().unwrap()).collect::<Vec<i32>>();

    // compute results
    (compute_final_frequency(&deltas), compute_first_frequency_reached_twice(&deltas))
}

fn compute_final_frequency(deltas: &[i32]) -> i32 {
    deltas.iter().sum()
}

fn compute_first_frequency_reached_twice(deltas: &[i32]) -> i32 {
    let mut reached = HashSet::new();

    let mut acc = 0;
    reached.insert(0);

    for delta in deltas.iter().cycle() {
        acc += delta;
        if reached.contains(&acc) {
            break;
        }
        reached.insert(acc);
    }

    acc
}

#[cfg(test)]
mod tests_step1 {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(compute_final_frequency(&vec!(1, -2, 3, 1)), 3);
    }

    #[test]
    fn all_positive() {
        assert_eq!(compute_final_frequency(&vec!(1, 1, 1)), 3);
    }

    #[test]
    fn last_negative() {
        assert_eq!(compute_final_frequency(&vec!(1, 1, -2)), 0);
    }

    #[test]
    fn all_negative() {
        assert_eq!(compute_final_frequency(&vec!(-1, -2, -3)), -6);
    }
}

#[cfg(test)]
mod tests_step2 {
    use super::*;

    #[test]
    fn first_example() {
        assert_eq!(compute_first_frequency_reached_twice(&vec!(1, -2, 3, 1)), 2);
    }

    #[test]
    fn other_examples() {
        assert_eq!(compute_first_frequency_reached_twice(&vec!(1, -1)), 0);
        assert_eq!(compute_first_frequency_reached_twice(&vec!(3, 3, 4, -2, -4)), 10);
        assert_eq!(compute_first_frequency_reached_twice(&vec!(-6, 3, 8, 5, -6)), 5);
        assert_eq!(compute_first_frequency_reached_twice(&vec!(7, 7, -2, -7, -4)), 14);
    }

}

#[cfg(test)]
mod tests_input {
    use super::*;

    #[test]
    fn input() {
        let (step1, step2) = run();
        assert_eq!(step1, 437);
        assert_eq!(step2, 655);
    }
}