use algos::day19::model::Device;
use algos::day19::execute;
use algos::day19::parse;

use utils;

// skip coverage because it is too slow in debug mode
#[cfg_attr(tarpaulin, skip)]
pub fn run() -> (usize, usize) {
    let input = &utils::read_file("inputs/day21.txt");
    execute_program(input)
}

// skip coverage because it is too slow in debug mode
#[cfg_attr(tarpaulin, skip)]
fn execute_program(input: &str) -> (usize, usize) {
    let (ip, instructions) = parse(input);

    let mut step1 = None;
    let mut step2 = None;
    let mut seen = vec!();

    loop {
        let mut device = Device { registers: [0, 0, 0, 0, 0, 0] };
        while ip < device.registers.len() {
            if device.registers[ip] == 28 {
                if step1.is_none() {
                    step1 = Some(device.registers[4]);
                }
                if seen.contains(&device.registers[4]) {
                    step2 = Some(*seen.last().unwrap());
                    break;
                }
                seen.push(device.registers[4]);
            }
            device = execute(device, &instructions[device.registers[ip]]);
            device.registers[ip] += 1;
        }
        if step1.is_some() {
            break;
        }
    }

    (step1.unwrap(), step2.unwrap())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn input() {
        assert_eq!(run(), (9079325, 3715167));
    }
}
