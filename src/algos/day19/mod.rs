pub mod model;

use self::model::Device;
use self::model::Instruction;
use self::model::OperationType::*;

use utils;

pub fn run() -> (usize, usize) {
    let input = &utils::read_file("inputs/day19.txt");
    (execute_program(input, 0), execute_program(input, 1))
}

fn execute_program(input: &str, start: usize) -> usize {
    let (ip, instructions) = parse(input);

    let mut device = Device { registers: [start, 0, 0, 0, 0, 0] };

    // initialisation ends when we reach line 1
    while device.registers[ip] != 1 {
        device = execute(device, &instructions[device.registers[ip]]);
        device.registers[ip] += 1;
    }

    // extract all factors
    let mut sum = 0;
    for factor in 1..=(device.registers[4] as f64).sqrt() as usize {
        if device.registers[4] % factor == 0 {
            sum += factor + device.registers[4] / factor;
        }
    }

    sum
}

fn execute(mut device: Device, instruction: &Instruction) -> Device {
    match instruction.op_type {
        ADDR => device.registers[instruction.c] = device.registers[instruction.a] + device.registers[instruction.b],
        ADDI => device.registers[instruction.c] = device.registers[instruction.a] + instruction.b,
        MULR => device.registers[instruction.c] = device.registers[instruction.a] * device.registers[instruction.b],
        MULI => device.registers[instruction.c] = device.registers[instruction.a] * instruction.b,
        BANR => device.registers[instruction.c] = device.registers[instruction.a] & device.registers[instruction.b],
        BANI => device.registers[instruction.c] = device.registers[instruction.a] & instruction.b,
        BORR => device.registers[instruction.c] = device.registers[instruction.a] | device.registers[instruction.b],
        BORI => device.registers[instruction.c] = device.registers[instruction.a] | instruction.b,
        SETR => device.registers[instruction.c] = device.registers[instruction.a],
        SETI => device.registers[instruction.c] = instruction.a,
        GTIR => device.registers[instruction.c] = if instruction.a > device.registers[instruction.b] { 1 } else { 0 },
        GTRI => device.registers[instruction.c] = if device.registers[instruction.a] > instruction.b { 1 } else { 0 },
        GTRR => device.registers[instruction.c] = if device.registers[instruction.a] > device.registers[instruction.b] { 1 } else { 0 },
        EQIR => device.registers[instruction.c] = if instruction.a == device.registers[instruction.b] { 1 } else { 0 },
        EQRI => device.registers[instruction.c] = if device.registers[instruction.a] == instruction.b { 1 } else { 0 },
        EQRR => device.registers[instruction.c] = if device.registers[instruction.a] == device.registers[instruction.b] { 1 } else { 0 }
    }
    device
}

fn parse(input: &str) -> (usize, Vec<Instruction>) {
    let mut lines = input.lines();

    let ip: usize = lines.next().expect("no lines in input")[4..].parse().expect("header has a bad format");

    (ip, lines.map(|line| Instruction::new(line)).collect())
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input() {
        assert_eq!(run(), (1248, 14952912));
    }
}
