use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::character::complete::{char, space1};
use nom::multi::separated_list1;
use nom::sequence::delimited;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::{parse_usize, title};
use util::split_blocks;
use util::wrist_device::{Instruction, Opcode, WristDevice};

#[derive(Debug, Copy, Clone)]
struct HiddenInstruction {
    opcode: usize,
    a: usize,
    b: usize,
    c: usize,
}

impl HiddenInstruction {
    fn as_instr_with_op(&self, op: Opcode) -> Instruction {
        Instruction::from_op(op, self.a, self.b, self.c)
    }
}

impl FromStr for HiddenInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instruction(s: &str) -> IResult<&str, HiddenInstruction> {
            let (s, v) = separated_list1(space1, parse_usize)(s)?;
            Ok((
                s,
                HiddenInstruction {
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

#[derive(Debug, Clone)]
struct Sample {
    before: Vec<usize>,
    instruction: HiddenInstruction,
    after: Vec<usize>,
}

impl FromStr for Sample {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_state(s: &str) -> IResult<&str, Vec<usize>> {
            let (s, _) = title(s)?;
            let (s, v) = delimited(
                char('['),
                separated_list1(tag(", "), parse_usize),
                char(']'),
            )(s)?;
            Ok((s, v))
        }
        let lines: Vec<&str> = s.lines().collect();
        let before = parse_state(lines[0]).unwrap().1;
        let instruction: HiddenInstruction = lines[1].parse().unwrap();
        let after = parse_state(lines[2]).unwrap().1;
        Ok(Sample {
            before,
            instruction,
            after,
        })
    }
}

#[derive(Debug, Clone)]
struct ClassificationDevice {
    samples: Vec<Sample>,
    instructions: Vec<HiddenInstruction>,
}

impl ClassificationDevice {
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

        let new_instrs: Vec<Instruction> = self
            .instructions
            .iter()
            .map(|i| {
                let op: Opcode = final_op_map.get(&i.opcode).copied().unwrap();
                i.as_instr_with_op(op)
            })
            .collect();

        let mut device: WristDevice = WristDevice::from_size_and_instructions(4, new_instrs);
        device.apply_all();

        (nb_behave, device.reg_value(0))
    }

    fn test_sample(&mut self, sample: &Sample) -> Vec<Opcode> {
        Opcode::all()
            .into_iter()
            .filter(|&op| {
                let mut device: WristDevice = WristDevice::from_registers(&sample.before);
                let instr: Instruction = sample.instruction.as_instr_with_op(op);
                device.apply_instruction(&instr);
                device.has_state(&sample.after)
            })
            .collect()
    }
}

impl FromStr for ClassificationDevice {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks: Vec<&str> = split_blocks(s);

        let instructions: Vec<HiddenInstruction> = blocks
            .pop()
            .unwrap()
            .lines()
            .map(|l| l.parse().unwrap())
            .collect();
        //Remove empty block
        blocks.pop();

        let samples: Vec<Sample> = blocks.into_iter().map(|b| b.parse().unwrap()).collect();

        Ok(Self {
            samples,
            instructions,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_16.txt").expect("Cannot open input file");
    let mut device: ClassificationDevice = s.parse().unwrap();

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
        let mut device: ClassificationDevice = ClassificationDevice {
            samples: vec![sample],
            instructions: Vec::new(),
        };

        assert_eq!(device.samples_behave_like().0, 1);
    }
}
