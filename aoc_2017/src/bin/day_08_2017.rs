use fxhash::FxHashMap;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::{alpha1, space1};
use nom::combinator::map;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_isize;

#[derive(Debug, Copy, Clone)]
enum RegisterAction {
    Increase,
    Decrease,
}

impl FromStr for RegisterAction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "inc" => Ok(RegisterAction::Increase),
            "dec" => Ok(RegisterAction::Decrease),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum RegisterCondition {
    Less,
    More,
    LessOrEqual,
    MoreOrEqual,
    Equal,
    NotEqual,
}

impl FromStr for RegisterCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "<" => Ok(RegisterCondition::Less),
            ">" => Ok(RegisterCondition::More),
            "<=" => Ok(RegisterCondition::LessOrEqual),
            ">=" => Ok(RegisterCondition::MoreOrEqual),
            "==" => Ok(RegisterCondition::Equal),
            "!=" => Ok(RegisterCondition::NotEqual),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Instruction {
    mod_reg: String,
    mod_action: RegisterAction,
    mod_value: isize,
    cond_reg: String,
    cond_op: RegisterCondition,
    cond_value: isize,
}

impl FromStr for Instruction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_instr(s: &str) -> IResult<&str, Instruction> {
            let (s, mod_reg) = map(terminated(alpha1, space1), |w: &str| w.to_string()).parse(s)?;
            let (s, mod_action): (&str, &str) = terminated(alpha1, space1).parse(s)?;
            let mod_action: RegisterAction = mod_action.parse().unwrap();
            let (s, mod_value) = terminated(parse_isize, tag(" if ")).parse(s)?;
            let (s, cond_reg) =
                map(terminated(alpha1, space1), |w: &str| w.to_string()).parse(s)?;
            let (s, cond_op): (&str, &str) =
                terminated(take_till(|c| c == ' '), space1).parse(s)?;
            let cond_op: RegisterCondition = cond_op.parse().unwrap();
            let (s, cond_value) = parse_isize(s)?;

            Ok((
                s,
                Instruction {
                    mod_reg,
                    mod_action,
                    mod_value,
                    cond_reg,
                    cond_op,
                    cond_value,
                },
            ))
        }
        Ok(parse_instr(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct ILikeRegisters {
    instructions: Vec<Instruction>,
    registers: FxHashMap<String, isize>,
}

impl ILikeRegisters {
    fn apply_instructions(&mut self) -> (isize, isize) {
        let mut largest_during: isize = 0;

        for i in self.instructions.iter() {
            let r_cond: &mut isize = self.registers.entry(i.cond_reg.clone()).or_insert(0);
            let modify: bool = match (i.cond_op, i.cond_value) {
                (RegisterCondition::Less, v) if *r_cond < v => true,
                (RegisterCondition::More, v) if *r_cond > v => true,
                (RegisterCondition::LessOrEqual, v) if *r_cond <= v => true,
                (RegisterCondition::MoreOrEqual, v) if *r_cond >= v => true,
                (RegisterCondition::Equal, v) if *r_cond == v => true,
                (RegisterCondition::NotEqual, v) if *r_cond != v => true,
                _ => false,
            };
            if modify {
                let r_mod: &mut isize = self.registers.entry(i.mod_reg.clone()).or_insert(0);
                match i.mod_action {
                    RegisterAction::Increase => *r_mod += i.mod_value,
                    RegisterAction::Decrease => *r_mod -= i.mod_value,
                }
            }
            let l: isize = self.largest_value();
            if l > largest_during {
                largest_during = l;
            }
        }
        (self.largest_value(), largest_during)
    }

    fn largest_value(&self) -> isize {
        self.registers.values().max().copied().unwrap_or(0)
    }
}

impl FromStr for ILikeRegisters {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<Instruction> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(ILikeRegisters {
            instructions,
            registers: FxHashMap::default(),
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_08.txt").expect("Cannot open input file");
    let mut reg: ILikeRegisters = s.parse().unwrap();
    let (largest_at_end, largest_during): (isize, isize) = reg.apply_instructions();
    println!("Part1: After applying the instructions, the largest value of any register is {largest_at_end}");
    println!("Part2: The largest value of any register during the process was {largest_during}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "b inc 5 if a > 1
a inc 1 if b < 5
c dec -10 if a >= 1
c inc -20 if c == 10
";

    #[test]
    fn part_1() {
        let mut reg: ILikeRegisters = EXAMPLE_1.parse().unwrap();
        assert_eq!((1, 10), reg.apply_instructions());
    }
}
