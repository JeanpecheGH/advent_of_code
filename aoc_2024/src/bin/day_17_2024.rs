use itertools::Itertools;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::{parse_usize, title};

#[derive(Clone)]
struct ChronoComputer {
    a: usize,
    b: usize,
    c: usize,
    ptr: usize,
    opcodes: Vec<usize>,
    output: Vec<usize>,
}
impl ChronoComputer {
    fn combo(&self, operand: usize) -> usize {
        match operand {
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => operand,
        }
    }
    fn apply_instructions(&mut self, opcode: usize, operand: usize) {
        match opcode {
            0 => {
                self.a /= 2usize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            1 => {
                self.b ^= operand;
                self.ptr += 2;
            }
            2 => {
                self.b = self.combo(operand) % 8;
                self.ptr += 2;
            }
            3 => {
                if self.a == 0 {
                    self.ptr += 2;
                } else {
                    self.ptr = operand;
                }
            }
            4 => {
                self.b ^= self.c;
                self.ptr += 2;
            }
            5 => {
                self.output.push(self.combo(operand) % 8);
                self.ptr += 2;
            }
            6 => {
                self.b = self.a / 2usize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            7 => {
                self.c = self.a / 2usize.pow(self.combo(operand) as u32);
                self.ptr += 2;
            }
            _ => (),
        }
    }

    fn output(&mut self) -> Vec<usize> {
        while self.ptr <= self.opcodes.len() - 2 {
            self.apply_instructions(self.opcodes[self.ptr], self.opcodes[self.ptr + 1]);
        }

        self.output.clone()
    }

    fn reverse_linear(&self) -> usize {
        fn inner_reverse(opcodes: &[usize], a: usize) -> Vec<usize> {
            if opcodes.is_empty() {
                vec![a]
            } else {
                let len: usize = opcodes.len();
                let target: usize = opcodes[len - 1];
                let mut solutions: Vec<usize> = Vec::new();
                for rest in 0..8 {
                    let local_a = a * 8 + rest;
                    let mut b: usize = rest ^ 1;
                    let c: usize = local_a / (2usize.pow(b as u32));
                    b ^= c;
                    b ^= 6;
                    if b % 8 == target {
                        solutions.push(local_a);
                    }
                }
                solutions
                    .into_iter()
                    .flat_map(|new_a| inner_reverse(&opcodes[..len - 1], new_a))
                    .collect()
            }
        }
        let sols: Vec<usize> = inner_reverse(&self.opcodes, 0);
        sols.into_iter().min().unwrap()
    }

    #[allow(dead_code)]
    fn reverse_bf(&self) -> usize {
        let mut a: usize = 8usize.pow((self.opcodes.len() - 1) as u32);
        loop {
            let mut computer: ChronoComputer = self.clone();
            computer.a = a;
            let output: Vec<usize> = computer.output();
            if output == self.opcodes {
                return a;
            }
            a += 1;
        }
    }
}

impl FromStr for ChronoComputer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_register(s: &str) -> IResult<&str, usize> {
            let (s, reg) = preceded(title, parse_usize)(s)?;
            Ok((s, reg))
        }

        fn parse_opcodes(s: &str) -> IResult<&str, Vec<usize>> {
            let (s, opcodes) = preceded(title, separated_list1(char(','), parse_usize))(s)?;
            Ok((s, opcodes))
        }

        let lines: Vec<&str> = s.lines().collect();
        let a: usize = parse_register(lines[0]).unwrap().1;
        let b: usize = parse_register(lines[1]).unwrap().1;
        let c: usize = parse_register(lines[2]).unwrap().1;
        let opcodes: Vec<usize> = parse_opcodes(lines[4]).unwrap().1;

        Ok(ChronoComputer {
            a,
            b,
            c,
            ptr: 0,
            opcodes,
            output: Vec::new(),
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_17.txt").expect("Cannot open input file");
    let mut computer: ChronoComputer = s.parse().unwrap();
    let computer_2: ChronoComputer = computer.clone();

    let out: Vec<usize> = computer.output();
    println!(
        "Part1: The output of the program is {}",
        out.into_iter().map(|n| n.to_string()).join(",")
    );
    println!(
        "Part2: {} is the smallest initial value that causes the program to copy itself",
        computer_2.reverse_linear()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0
";

    const EXAMPLE_2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0
";

    #[test]
    fn part_1() {
        let mut computer: ChronoComputer = EXAMPLE_1.parse().unwrap();
        assert_eq!(computer.output(), vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0]);
    }

    #[test]
    fn part_2() {
        let computer: ChronoComputer = EXAMPLE_2.parse().unwrap();
        assert_eq!(computer.reverse_bf(), 117440);
    }
}
