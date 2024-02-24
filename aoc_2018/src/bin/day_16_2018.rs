use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::character::complete::{char, space1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::{parse_usize, title};
use util::split_blocks;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
enum Opcode {
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
    fn all() -> Vec<Opcode> {
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

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instruction(s: &str) -> IResult<&str, Instruction> {
            let (s, v) = separated_list1(space1, parse_usize)(s)?;
            Ok((
                s,
                Instruction {
                    opcode: v[0],
                    a: v[1],
                    b: v[2],
                    c: v[3],
                },
            ))
        }
        Ok(parse_instruction(s).unwrap().1)
    }
}

#[derive(Debug, Copy, Clone)]
struct Sample {
    before: [usize; 4],
    instruction: Instruction,
    after: [usize; 4],
}

impl FromStr for Sample {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_state(s: &str) -> IResult<&str, [usize; 4]> {
            let (s, _) = title(s)?;
            let (s, v) = delimited(
                char('['),
                separated_list1(tag(", "), parse_usize),
                char(']'),
            )(s)?;
            Ok((s, [v[0], v[1], v[2], v[3]]))
        }
        let lines: Vec<&str> = s.lines().collect();
        let before = parse_state(lines[0]).unwrap().1;
        let instruction: Instruction = lines[1].parse().unwrap();
        let after = parse_state(lines[2]).unwrap().1;
        Ok(Sample {
            before,
            instruction,
            after,
        })
    }
}

#[derive(Debug, Clone)]
struct WristDevice {
    registers: [usize; 4],
    samples: Vec<Sample>,
    instructions: Vec<Instruction>,
}

impl WristDevice {
    fn samples_behave_like(&mut self) -> (usize, FxHashMap<usize, FxHashSet<Opcode>>) {
        let mut op_map: FxHashMap<usize, FxHashSet<Opcode>> = FxHashMap::default();
        let samples = self.samples.clone();
        let nb_behave: usize = samples
            .iter()
            .filter(|s| {
                let v = self.test_sample(s);
                let e = op_map.entry(s.instruction.opcode).or_default();
                v.iter().for_each(|&op| {
                    let _ = e.insert(op);
                });
                v.len() >= 3
            })
            .count();

        (nb_behave, op_map)
    }

    fn sample_and_execute(&mut self) -> (usize, usize) {
        let (nb_behave, mut op_map) = self.samples_behave_like();

        //Reduce the op_map to associate all code to a unique Opcode
        let mut final_op_map: FxHashMap<usize, Opcode> = FxHashMap::default();
        while !op_map.is_empty() {
            if let Some((&op, v)) = op_map.iter().find(|(_, v)| v.len() == 1) {
                let real_op: Opcode = v.iter().last().copied().unwrap();
                final_op_map.insert(op, real_op);
                op_map.remove(&op);
                op_map.iter_mut().for_each(|(_, v)| {
                    let _ = v.remove(&real_op);
                });
            }
        }

        self.registers = [0; 4];
        for i in 0..self.instructions.len() {
            let instr = self.instructions[i];
            let op: Opcode = final_op_map.get(&instr.opcode).copied().unwrap();
            self.apply_instruction(op, &instr);
        }

        (nb_behave, self.registers[0])
    }
    fn apply_instruction(&mut self, op: Opcode, instruction: &Instruction) {
        let (a, b, c) = (instruction.a, instruction.b, instruction.c);
        match op {
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
    fn test_sample(&mut self, sample: &Sample) -> Vec<Opcode> {
        Opcode::all()
            .into_iter()
            .filter(|&op| {
                self.registers = sample.before;
                self.apply_instruction(op, &sample.instruction);
                self.registers == sample.after
            })
            .collect()
    }
}

impl FromStr for WristDevice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks: Vec<&str> = split_blocks(s);

        let instructions: Vec<Instruction> = blocks
            .pop()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        //Remove empty block
        blocks.pop();

        let samples: Vec<Sample> = blocks.into_iter().map(|b| b.parse().unwrap()).collect();

        Ok(Self {
            registers: [0; 4],
            samples,
            instructions,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_16.txt").expect("Cannot open input file");
    let mut device: WristDevice = s.parse().unwrap();

    let (nb_behave, reg_zero) = device.sample_and_execute();

    println!(
        "Part1: {} samples are behaving like 3 opcodes or more",
        nb_behave
    );
    println!(
        "Part2: After running the test program, register 0 contains the value {}",
        reg_zero
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "Before: [3, 2, 1, 1]
9 2 1 2
After:  [3, 2, 2, 1]";

    #[test]
    fn part_1() {
        let sample: Sample = EXAMPLE_1.parse().unwrap();
        let mut device: WristDevice = WristDevice {
            registers: [0; 4],
            samples: vec![sample],
            instructions: Vec::new(),
        };

        assert_eq!(device.samples_behave_like().0, 1);
    }
}
