use crate::basic_parser::parse_usize;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space1};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum Opcode {
    Addr,
    Addi,
    Mulr,
    Muli,
    Banr,
    Bani,
    Borr,
    Bori,
    Setr,
    Seti,
    Gtir,
    Gtri,
    Gtrr,
    Eqir,
    Eqri,
    Eqrr,
}

impl Opcode {
    pub fn all() -> Vec<Opcode> {
        vec![
            Opcode::Addr,
            Opcode::Addi,
            Opcode::Mulr,
            Opcode::Muli,
            Opcode::Banr,
            Opcode::Bani,
            Opcode::Borr,
            Opcode::Bori,
            Opcode::Setr,
            Opcode::Seti,
            Opcode::Gtir,
            Opcode::Gtri,
            Opcode::Gtrr,
            Opcode::Eqir,
            Opcode::Eqri,
            Opcode::Eqrr,
        ]
    }
}

impl FromStr for Opcode {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "addr" => Ok(Self::Addr),
            "addi" => Ok(Self::Addi),
            "mulr" => Ok(Self::Mulr),
            "muli" => Ok(Self::Muli),
            "banr" => Ok(Self::Banr),
            "bani" => Ok(Self::Bani),
            "borr" => Ok(Self::Borr),
            "bori" => Ok(Self::Bori),
            "setr" => Ok(Self::Setr),
            "seti" => Ok(Self::Seti),
            "gtir" => Ok(Self::Gtir),
            "gtri" => Ok(Self::Gtri),
            "gtrr" => Ok(Self::Gtrr),
            "eqir" => Ok(Self::Eqir),
            "eqri" => Ok(Self::Eqri),
            "eqrr" => Ok(Self::Eqrr),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Instruction {
    opcode: Opcode,
    a: usize,
    b: usize,
    c: usize,
}

impl Instruction {
    pub fn from_op(opcode: Opcode, a: usize, b: usize, c: usize) -> Self {
        Instruction { opcode, a, b, c }
    }
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
            let (s, op) = map(terminated(alpha1, space1), |op_name: &str| {
                op_name.parse::<Opcode>().unwrap()
            })
            .parse(s)?;
            let (s, v) = separated_list1(space1, parse_usize).parse(s)?;
            Ok((
                s,
                Instruction {
                    opcode: op,
                    a: v[0],
                    b: v[1],
                    c: v[2],
                },
            ))
        }
        Ok(parse_instruction(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
pub struct WristDevice {
    reg_pointer: Option<usize>,
    registers: Vec<usize>,
    instructions: Vec<Instruction>,
}

impl WristDevice {
    pub fn from_registers(registers: &[usize]) -> Self {
        WristDevice {
            reg_pointer: None,
            registers: registers.to_vec(),
            instructions: Vec::new(),
        }
    }

    pub fn from_size_and_instructions(size: usize, instructions: Vec<Instruction>) -> Self {
        WristDevice {
            reg_pointer: None,
            registers: vec![0; size],
            instructions,
        }
    }

    pub fn reset(&mut self) {
        for r in self.registers.iter_mut() {
            *r = 0;
        }
    }

    pub fn apply_instruction(&mut self, instruction: &Instruction) {
        let (a, b, c) = (instruction.a, instruction.b, instruction.c);
        match instruction.opcode {
            Opcode::Addr => self.registers[c] = self.registers[a] + self.registers[b],
            Opcode::Addi => self.registers[c] = self.registers[a] + b,
            Opcode::Mulr => self.registers[c] = self.registers[a] * self.registers[b],
            Opcode::Muli => self.registers[c] = self.registers[a] * b,
            Opcode::Banr => self.registers[c] = self.registers[a] & self.registers[b],
            Opcode::Bani => self.registers[c] = self.registers[a] & b,
            Opcode::Borr => self.registers[c] = self.registers[a] | self.registers[b],
            Opcode::Bori => self.registers[c] = self.registers[a] | b,
            Opcode::Setr => self.registers[c] = self.registers[a],
            Opcode::Seti => self.registers[c] = a,
            Opcode::Gtir => self.registers[c] = (a > self.registers[b]) as usize,
            Opcode::Gtri => self.registers[c] = (self.registers[a] > b) as usize,
            Opcode::Gtrr => self.registers[c] = (self.registers[a] > self.registers[b]) as usize,
            Opcode::Eqir => self.registers[c] = (a == self.registers[b]) as usize,
            Opcode::Eqri => self.registers[c] = (self.registers[a] == b) as usize,
            Opcode::Eqrr => self.registers[c] = (self.registers[a] == self.registers[b]) as usize,
        }
    }

    pub fn apply_all(&mut self) {
        for i in 0..self.instructions.len() {
            let instr = self.instructions[i];
            self.apply_instruction(&instr);
        }
    }

    pub fn apply_all_with_pointer(&mut self, stop_after_init: bool) {
        let pointer: usize = self.reg_pointer.unwrap();
        while self.registers[pointer] < self.instructions.len() {
            let instr_idx: usize = self.registers[pointer];
            let instr: Instruction = self.instructions[instr_idx];
            self.apply_instruction(&instr);
            self.registers[pointer] += 1;
            //Stopping after the first "init" loop
            if instr_idx >= self.registers[pointer] && stop_after_init {
                break;
            }
        }
    }

    pub fn has_state(&self, registers: &[usize]) -> bool {
        self.registers == registers
    }

    pub fn reg_value(&self, reg_index: usize) -> usize {
        self.registers[reg_index]
    }

    pub fn set_reg_value(&mut self, reg_index: usize, value: usize) {
        self.registers[reg_index] = value;
    }
}

impl FromStr for WristDevice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pointer(s: &str) -> IResult<&str, usize> {
            preceded(tag("#ip "), parse_usize).parse(s)
        }

        let mut lines = s.lines();
        //Parse instruction pointer register
        let reg_pointer: Option<usize> = parse_pointer(lines.next().unwrap()).map(|r| r.1).ok();
        //Parse instructions
        let instructions: Vec<Instruction> = lines.map(|l| l.parse().unwrap()).collect();

        Ok(WristDevice {
            reg_pointer,
            registers: vec![0; 6],
            instructions,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_instruction() {
        let instruction: Instruction = "addi 1 2 3".parse().unwrap();
        assert_eq!(
            instruction,
            Instruction {
                opcode: Opcode::Addi,
                a: 1,
                b: 2,
                c: 3
            }
        )
    }
}
