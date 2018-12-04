use regex::*;
use std::collections::HashMap;
use std::cell::RefCell;
use utils;

pub fn run() -> (usize, usize) {
    // parse input
    let input = utils::read_file("inputs/day04.txt");

    let mut lines: Vec<&str> = input.lines().collect();
    lines.sort();

    run_from_input(&lines)
}

fn run_from_input(lines: &[&str]) -> (usize, usize) {
    let nights = parse_nights(&lines);
    let durations = compute_asleep_duration_per_guard(&nights);
    let (guard, max) = durations.iter().max_by(|(_, a), (_, b)|
        a.borrow().iter().sum::<usize>().cmp(&b.borrow().iter().sum())
    ).unwrap();

    let step1 = guard * max.borrow().iter().position(|val| val == max.borrow().iter().max().unwrap()).unwrap();

    (step1, 0)
}

fn compute_asleep_duration_per_guard(nights: &[Night]) -> HashMap<usize, RefCell<Vec<usize>>> {
    let mut minutes_by_guard: HashMap<usize, RefCell<Vec<usize>>> = HashMap::new();
    for night in nights {
        minutes_by_guard.entry(night.guard).or_insert_with(||RefCell::new(vec!(0; 60)));
        let mut minutes = &minutes_by_guard[&night.guard];

        let periods = night.periods.borrow();
        for (idx, period) in periods.iter().enumerate() {
            if period.status == Status::Asleep {
                let from: usize = get_accountable_minutes(period);
                let to: usize = if idx < periods.len() { get_accountable_minutes(&periods[idx + 1]) } else { 59 };
                for minute in minutes.borrow_mut().iter_mut().take(to).skip(from) {
                    *minute += 1;
                }
            }
        }
    }
    minutes_by_guard
}

fn get_accountable_minutes(period: &Period) -> usize {
    if period.hour == 0 {
        period.minute
    } else {
        0
    }
}

#[derive(PartialEq, Debug)]
enum Status {
    Awake,
    Asleep,
}

#[derive(Debug)]
struct Period {
    hour: usize,
    minute: usize,
    status: Status,
}

#[derive(Debug)]
struct Night {
    guard: usize,
    periods: RefCell<Vec<Period>>,
}

#[cfg_attr(tarpaulin, skip)]
fn parse(value: &str) -> Captures {
    lazy_static! {
		static ref regex: Regex = Regex::new(r"^\[.*? (\d\d):(\d\d)\] (Guard #(\d+) begins shift|wakes up|falls asleep)$").unwrap();
	}
    regex.captures(value).unwrap()
}

#[cfg_attr(tarpaulin, skip)]
fn parse_nights(values: &[&str]) -> Vec<Night> {
    let mut nights = RefCell::new(vec!());

    for value in values {
        let parsed = parse(value);

        let hour = parsed[1].parse::<usize>().unwrap();
        let minute = parsed[2].parse::<usize>().unwrap();
        let action = &parsed[3];

        if action.starts_with("Guard") {
            nights.borrow_mut().push(Night {
                guard: parsed[4].parse::<usize>().unwrap(),
                periods: RefCell::new(vec!(Period { hour: 0, minute: 0, status: Status::Awake })),
            });
        } else {
            let period = Period { hour, minute, status: if action == "wakes up" { Status::Awake } else { Status::Asleep } };
            nights.get_mut().last().unwrap().periods.borrow_mut().push(period);
        }
    }

    nights.into_inner()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse() {
        let input_test: Vec<&str> = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up".lines().collect();
        assert_eq!(run_from_input(&input_test), (240, 0));
    }

    #[test]
    fn example() {}

    #[test]
    fn input() {
        assert_eq!(run(), (109659, 0));
    }
}