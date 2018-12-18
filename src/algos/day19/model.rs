use std::str::FromStr;
use std::fmt::Error;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Device {
    pub registers: [usize; 6],
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub op_type: OperationType,
    pub a: usize,
    pub b: usize,
    pub c: usize,
}

impl Instruction {
    pub fn new(input: &str) -> Self {
        let buffer: Vec<&str> = input.split(' ').collect();
        Instruction { op_type: OperationType::from_str(buffer[0]).expect("unknown operation type"), a: buffer[1].parse().unwrap(), b: buffer[2].parse().unwrap(), c: buffer[3].parse().unwrap() }
    }
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


impl FromStr for OperationType {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        match input {
            "addr" => Ok(OperationType::ADDR),
            "addi" => Ok(OperationType::ADDI),
            "mulr" => Ok(OperationType::MULR),
            "muli" => Ok(OperationType::MULI),
            "banr" => Ok(OperationType::BANR),
            "bani" => Ok(OperationType::BANI),
            "borr" => Ok(OperationType::BORR),
            "bori" => Ok(OperationType::BORI),
            "setr" => Ok(OperationType::SETR),
            "seti" => Ok(OperationType::SETI),
            "gtir" => Ok(OperationType::GTIR),
            "gtri" => Ok(OperationType::GTRI),
            "gtrr" => Ok(OperationType::GTRR),
            "eqir" => Ok(OperationType::EQIR),
            "eqri" => Ok(OperationType::EQRI),
            "eqrr" => Ok(OperationType::EQRR),
            _ => Err(Error {})
        }
    }
}