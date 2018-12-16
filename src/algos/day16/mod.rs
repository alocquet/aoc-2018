pub mod model;

use self::model::*;
use self::model::Device;
use self::model::OperationType;
use self::model::Instruction;
use self::model::Operation;
use self::model::OperationType::*;

use utils;

pub fn run() -> (usize, usize) {
    let input = &utils::read_file("inputs/day16-s1.txt");
    let operations = parse(input);
    let ope_matches = match_operations(&operations);

    let mappings = compute_operations_mappings(&ope_matches);

    let input = &utils::read_file("inputs/day16-s2.txt");
    let mut device = Device { registers: [0, 0, 0, 0] };
    for line in input.lines() {
        let instruction = Instruction::new(line);
        device = execute(device, &instruction, &mappings[instruction.op_code]);
    }

    (ope_matches.iter().filter(|&matches| matches.operation_type.len() >= 3).count(), device.registers[0])
}


fn execute(mut device: Device, instruction: &Instruction, op_type: &OperationType) -> Device {
    match op_type {
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

fn match_operations(operations: &[Operation]) -> Vec<OperationMatch> {
    let mut matches = vec!();
    for operation in operations {
        let mut operation_types: Vec<OperationType> = vec!();
        for operation_type in &[ADDR, ADDI, MULR, MULI, BANR, BANI, BORR, BORI, SETR, SETI, GTIR, GTRI, GTRR, EQIR, EQRI, EQRR] {
            if execute(operation.from, &operation.instruction, operation_type) == operation.to {
                operation_types.push((*operation_type).clone());
            }
        }
        matches.push(OperationMatch { op_code: operation.instruction.op_code, operation_type: operation_types });
    }
    matches
}

fn compute_operations_mappings(operation_matches: &[OperationMatch]) -> Vec<OperationType> {
    let mut mappings = vec!(None; 16);
    for operation_match in operation_matches {
        let mapping = &mut mappings[operation_match.op_code];
        if mapping.is_none() {
            *mapping = Some(operation_match.operation_type.clone());
        } else {
            *mapping = Some(mapping.clone().unwrap().iter().filter(|op_type| operation_match.operation_type.contains(op_type)).cloned().collect());
        }
    }

    let mut mappings: Vec<Vec<OperationType>> = mappings.iter().map(|map| map.clone().unwrap()).collect();
    loop {
        for idx in 0..16 {
            if mappings[idx].len() == 1 {
                let op_type = mappings[idx].first().unwrap().clone();
                for (other_idx, mapping) in mappings.iter_mut().enumerate() {
                    if other_idx != idx {
                        mapping.retain(|op| *op != op_type);
                    }
                }
            }
        }
        if mappings.iter().find(|mapping| mapping.len() > 1).is_none() {
            break;
        }
    }
    mappings.iter().map(|mapping| mapping.first().unwrap().clone()).collect()
}

fn parse(input: &str) -> Vec<Operation> {
    let mut operations = vec!();

    let mut lines = input.lines();
    loop {
        let line = lines.next();
        if line.is_some() {
            let from = Device::new(line.unwrap());
            let instruction = Instruction::new(lines.next().unwrap());
            let to = Device::new(lines.next().unwrap());
            operations.push(Operation { from, instruction, to });
            lines.next();
        } else {
            break;
        }
    }
    operations
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_step1() {
        let operations = parse("Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]

");
        assert_eq!(match_operations(&operations).iter().filter(|&matches| matches.operation_type.len() >= 3).count(), 1);
    }

    #[test]
    fn input() {
        assert_eq!(run(), (493, 445));
    }
}
