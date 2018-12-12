use regex::*;
use utils;
use std::collections::HashSet;
use std::collections::HashMap;

pub fn run() -> (i64, i64) {
    (run_from_file("inputs/day12.txt", 20), run_from_file("inputs/day12.txt", 50_000_000_000))
}

fn run_from_file(file: &str, nb_iterations: i64) -> i64{
    let input = utils::read_file(file);
    let (initial, rules) = parse(&input);

    let mut current_state = initial.clone();

    let mut seen_nodes = HashMap::new();

    let mut first_node_idx: i64 = 0;

    for stream_idx in 0..nb_iterations as i64 {
        // compute slice
        let (i, _) = current_state.chars().enumerate().find(|(_, c)| *c == '#').unwrap();
        let variation = i as i64 - 4;
        first_node_idx += variation;

        // add .... before first_node_idx # and after last #
        let mut tmp = "....".to_owned();
        tmp.push_str(current_state.trim_left_matches('.').trim_right_matches('.'));
        tmp.push_str("....");
        current_state = tmp;

        if seen_nodes.contains_key(&current_state) {
            first_node_idx += variation * (nb_iterations - stream_idx);
            break;
        }
        seen_nodes.insert(current_state.clone(), (stream_idx, first_node_idx));

        let mut next = "..".to_owned();
        for i in 0..current_state.len() - 5 {
            if rules.contains(&current_state[i..i + 5]) {
                next.push('#');
            } else {
                next.push('.');
            }
        }
        current_state = next;
    }

    current_state.chars().enumerate().map(|(idx, val)| if val == '#' { (idx as i64) + first_node_idx } else { 0 }).sum()
}

fn parse(input: &str) -> (String, HashSet<String>) {
    let reg_initial: Regex = Regex::new(r"^initial state: (.*)$").unwrap();
    let reg_rule: Regex = Regex::new(r"^(.*?) => (.)$").unwrap();

    let mut lines = input.lines();
    let initial = reg_initial.captures(lines.next().unwrap()).unwrap()[1].to_owned();
    lines.next();

    let mut rules = HashSet::new();
    for line in lines {
        let capture = reg_rule.captures(line).unwrap();
        if &capture[2] == "#" {
            rules.insert(capture[1].to_owned());
        }
    }

    (initial, rules)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn compute_example_power_levels() {
        assert_eq!(run_from_file("inputs/day12-example.txt", 20), 325);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (2045, 2100000000428));
    }
}