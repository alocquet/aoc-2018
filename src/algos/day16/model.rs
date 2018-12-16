use regex::Regex;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Device {
    pub registers: [usize; 4],
}

impl Device {
    pub fn new(input: &str) -> Self {
        lazy_static! {
            static ref regex: Regex = Regex::new(r"^(?:Before|After):\s*\[(.*), (.*), (.*), (.*)\]$").expect("Malformed regex");
        }
        let buffer = regex.captures(input).expect("bad format");
        Device { registers: [buffer[1].parse().unwrap(), buffer[2].parse().unwrap(), buffer[3].parse().unwrap(), buffer[4].parse().unwrap()] }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op_code: usize,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        let buffer: Vec<&str> = input.split(' ').collect();
        Instruction { op_code: buffer[0].parse().unwrap(), a: buffer[1].parse().unwrap(), b: buffer[2].parse().unwrap(), c: buffer[3].parse().unwrap() }
    }
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub from: Device,
    pub instruction: Instruction,
    pub to : Device,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum OperationType {
    ADDR,
    ADDI,
    MULR,
    MULI,
    BANR,
    BANI,
    BORR,
    BORI,
    SETR,
    SETI,
    GTIR,
    GTRI,
    GTRR,
    EQIR,
    EQRI,
    EQRR,
}

pub struct OperationMatch {
    pub op_code: usize,
    pub operation_type: Vec<OperationType>,
}