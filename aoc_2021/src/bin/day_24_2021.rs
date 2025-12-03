use nom::branch::alt;
use nom::character::complete::{alpha1, anychar, space1};
use nom::combinator::{map, opt, rest};
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_isize;

#[derive(Debug, Copy, Clone)]
enum LogicValue {
    Reg(char),
    Val(isize),
}

impl LogicValue {
    fn val(&self, regs: &[isize]) -> isize {
        match self {
            LogicValue::Reg(c) => {
                let reg_id: usize = (*c as u8 - b'w') as usize;
                regs[reg_id]
            }
            LogicValue::Val(v) => *v,
        }
    }

    fn set(&self, regs: &mut [isize], value: isize) {
        if let LogicValue::Reg(c) = self {
            let reg_id: usize = (*c as u8 - b'w') as usize;
            regs[reg_id] = value;
        }
    }
}

impl FromStr for LogicValue {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_value(s: &str) -> IResult<&str, LogicValue> {
            alt((
                map(parse_isize, LogicValue::Val),
                map(anychar, LogicValue::Reg),
            ))
            .parse(s)
        }
        Ok(parse_value(s).unwrap().1)
    }
}

#[derive(Debug, Copy, Clone)]
enum LogicInstruction {
    Inp(LogicValue),
    Add(LogicValue, LogicValue),
    Mul(LogicValue, LogicValue),
    Div(LogicValue, LogicValue),
    Mod(LogicValue, LogicValue),
    Eql(LogicValue, LogicValue),
}

impl FromStr for LogicInstruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instruction(s: &str) -> IResult<&str, LogicInstruction> {
            let (s, name) = alpha1(s)?;
            let (s, val1) = preceded(space1, alpha1).parse(s)?;
            let (s, val2) = opt(preceded(space1, rest)).parse(s)?;

            let val1: LogicValue = val1.parse().unwrap();
            let instr: LogicInstruction = if let Some(val2) = val2 {
                let val2: LogicValue = val2.parse().unwrap();
                match name {
                    "add" => LogicInstruction::Add(val1, val2),
                    "mul" => LogicInstruction::Mul(val1, val2),
                    "div" => LogicInstruction::Div(val1, val2),
                    "mod" => LogicInstruction::Mod(val1, val2),
                    _ => LogicInstruction::Eql(val1, val2),
                }
            } else {
                LogicInstruction::Inp(val1)
            };
            Ok((s, instr))
        }
        Ok(parse_instruction(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct LogicUnit {
    instructions: Vec<LogicInstruction>,
}

impl LogicUnit {
    fn run(&self, mut input: Vec<isize>) -> [isize; 4] {
        let mut regs: [isize; 4] = [0, 0, 0, 0];
        for instr in &self.instructions {
            match instr {
                LogicInstruction::Inp(v1) => v1.set(&mut regs, input.pop().unwrap()),
                LogicInstruction::Add(v1, v2) => {
                    let old = v1.val(&regs);
                    let new = old + v2.val(&regs);
                    v1.set(&mut regs, new);
                }
                LogicInstruction::Mul(v1, v2) => {
                    let old = v1.val(&regs);
                    let new = old * v2.val(&regs);
                    v1.set(&mut regs, new);
                }
                LogicInstruction::Div(v1, v2) => {
                    let old = v1.val(&regs);
                    let new = old / v2.val(&regs);
                    v1.set(&mut regs, new);
                }
                LogicInstruction::Mod(v1, v2) => {
                    let old = v1.val(&regs);
                    let new = old % v2.val(&regs);
                    v1.set(&mut regs, new);
                }
                LogicInstruction::Eql(v1, v2) => {
                    let old = v1.val(&regs);
                    let new = (old == v2.val(&regs)) as isize;
                    v1.set(&mut regs, new);
                }
            }
        }

        regs
    }

    fn min_max_model_number(&self) -> (Vec<isize>, Vec<isize>) {
        fn two_values(diff: isize, max: bool) -> (isize, isize) {
            match (diff, max) {
                (_, true) if diff > 0 => (9 - diff, 9),
                (_, true) => (9, 9 + diff),
                (_, false) if diff > 0 => (1, 1 + diff),
                (_, false) => (1 - diff, 1),
            }
        }

        let (pair_difference, _) = self
            .instructions
            //Split the instruction for each of the 14 inputs
            .chunks(18)
            //Get the 2 offset variables
            .map(|chunk| {
                let a: isize = if let LogicInstruction::Add(_, LogicValue::Val(v)) = chunk[5] {
                    v
                } else {
                    0
                };
                let b: isize = if let LogicInstruction::Add(_, LogicValue::Val(v)) = chunk[15] {
                    v
                } else {
                    0
                };
                (a, b)
            })
            //We want the index of each input number
            .enumerate()
            //Compute the difference between each input pairs
            .fold(
                (Vec::new(), Vec::new()),
                |(mut acc, mut cache), (i, (a, b))| {
                    if a > 0 {
                        //This is a mul 26 step
                        //Store the current index and added offset for further use
                        cache.push((i, b));
                    } else {
                        //This is a div 26 step
                        let (index, offset) = cache.pop().unwrap();
                        acc.push(((index, i), offset + a));
                    }
                    (acc, cache)
                },
            );

        let mut min_model_number: [isize; 14] = [0; 14];
        let mut max_model_number: [isize; 14] = [0; 14];

        for ((i, j), diff) in pair_difference {
            (min_model_number[i], min_model_number[j]) = two_values(diff, false);
            (max_model_number[i], max_model_number[j]) = two_values(diff, true);
        }

        (min_model_number.to_vec(), max_model_number.to_vec())
    }

    fn model_numbers(&self) -> (isize, isize) {
        let (mut min_vec, mut max_vec) = self.min_max_model_number();
        let min: isize = min_vec.iter().fold(0, |acc, i| acc * 10 + *i);
        let max: isize = max_vec.iter().fold(0, |acc, i| acc * 10 + *i);

        //Check the computed values in the ALU
        min_vec.reverse();
        max_vec.reverse();
        if self.run(min_vec)[3] == 0 && self.run(max_vec)[3] == 0 {
            (min, max)
        } else {
            (0, 0)
        }
    }
}

impl FromStr for LogicUnit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<LogicInstruction> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(LogicUnit { instructions })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_24.txt").expect("Cannot open input file");
    let unit: LogicUnit = s.parse().unwrap();
    let (min, max) = unit.model_numbers();
    println!("Part1: The largest accepted model number is {max}");
    println!("Part2: The smallest accepted model number is {min}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "inp x
mul x -1";
    const EXAMPLE_2: &str = "inp z
inp x
mul z 3
eql z x
";
    const EXAMPLE_3: &str = "inp w
add z w
mod z 2
div w 2
add y w
mod y 2
div w 2
add x w
mod x 2
div w 2
mod w 2
";

    #[test]
    fn part_1_test_1() {
        let unit: LogicUnit = EXAMPLE_1.parse().unwrap();
        assert_eq!([0, -8, 0, 0], unit.run(vec![8]));
    }
    #[test]
    fn part_1_test_2() {
        let unit: LogicUnit = EXAMPLE_2.parse().unwrap();
        assert_eq!([0, 9, 0, 1], unit.run(vec![9, 3]));
    }
    #[test]
    fn part_1_test_3() {
        let unit: LogicUnit = EXAMPLE_2.parse().unwrap();
        assert_eq!([0, 5, 0, 0], unit.run(vec![5, 3]));
    }

    #[test]
    fn part_1_test_4() {
        let unit: LogicUnit = EXAMPLE_3.parse().unwrap();
        assert_eq!([1, 1, 1, 1], unit.run(vec![15]));
    }

    #[test]
    fn part_1_test_5() {
        let unit: LogicUnit = EXAMPLE_3.parse().unwrap();
        assert_eq!([0, 0, 0, 0], unit.run(vec![16]));
    }
}
