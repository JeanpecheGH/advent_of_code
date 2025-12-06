use nom::IResult;
use nom::Parser;
use nom::character::anychar;
use nom::character::complete::{space0, space1};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use std::str::FromStr;
use util::basic_parser::usize_list;

enum Operation {
    Mul,
    Add,
}

impl Operation {
    fn from_char(c: char) -> Operation {
        match c {
            '*' => Operation::Mul,
            '+' => Operation::Add,
            _ => panic!("Unknown operation"),
        }
    }

    fn parse_ops(s: &str) -> IResult<&str, Vec<Operation>> {
        let (s, ops) = separated_list1(space1, anychar).parse(s)?;
        let ops: Vec<Operation> = ops.into_iter().map(Operation::from_char).collect();
        Ok((s, ops))
    }
}

struct MathProblems {
    numbers: Vec<Vec<usize>>,
    ops: Vec<Operation>,
}

impl MathProblems {
    fn add_results(&self) -> usize {
        self.ops
            .iter()
            .enumerate()
            .map(|(i, op)| match op {
                Operation::Mul => self.numbers.iter().map(|v| v[i]).product::<usize>(),
                Operation::Add => self.numbers.iter().map(|v| v[i]).sum::<usize>(),
            })
            .sum()
    }
}

impl FromStr for MathProblems {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_numbers(s: &str) -> IResult<&str, Vec<usize>> {
            let (s, numbers) = preceded(space0, usize_list).parse(s)?;
            Ok((s, numbers))
        }

        let mut lines: Vec<&str> = s.lines().collect();
        let ops: Vec<Operation> = Operation::parse_ops(lines.pop().unwrap()).unwrap().1;
        let numbers: Vec<Vec<usize>> = lines.iter().map(|l| parse_numbers(l).unwrap().1).collect();

        Ok(MathProblems { numbers, ops })
    }
}

struct ColumnMathProblems {
    numbers: Vec<Vec<usize>>,
    ops: Vec<Operation>,
}

impl ColumnMathProblems {
    fn add_results(&self) -> usize {
        self.ops
            .iter()
            .enumerate()
            .map(|(i, op)| match op {
                Operation::Mul => self.numbers[i].iter().product::<usize>(),
                Operation::Add => self.numbers[i].iter().sum::<usize>(),
            })
            .sum()
    }
}

impl FromStr for ColumnMathProblems {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<&str> = s.lines().collect();
        let ops: Vec<Operation> = Operation::parse_ops(lines.pop().unwrap()).unwrap().1;

        // Split each line by digit, replace spaces by 0
        let digits: Vec<Vec<usize>> = lines
            .iter()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap_or(0) as usize)
                    .collect()
            })
            .collect();
        // Write numbers by column, create a new problem if a column is empty (==0)
        let nb_digits = digits.iter().map(|d| d.len()).max().unwrap();
        let mut numbers: Vec<Vec<usize>> = Vec::new();
        let mut current_problem: Vec<usize> = Vec::new();
        for i in 0..nb_digits {
            let column_digits: Vec<usize> = digits
                .iter()
                .map(|d| d.get(i).copied().unwrap_or(0))
                .collect();
            let mut current: usize = 0;
            for d in column_digits {
                if d != 0 {
                    current *= 10;
                    current += d;
                }
            }
            if current == 0 {
                numbers.push(current_problem);
                current_problem = Vec::new();
            } else {
                current_problem.push(current);
            }
        }
        numbers.push(current_problem);

        Ok(ColumnMathProblems { numbers, ops })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_06.txt").expect("Cannot open input file");
    let problems: MathProblems = s.parse().unwrap();

    println!(
        "Part1: The sum of the answers is {}",
        problems.add_results()
    );
    let problems: ColumnMathProblems = s.parse().unwrap();
    println!(
        "Part2: With vertical numbers, the sum of the answers is {}",
        problems.add_results()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
";
    #[test]
    fn test_part_1() {
        let problems: MathProblems = EXAMPLE_1.parse().unwrap();
        assert_eq!(problems.add_results(), 4277556);
    }

    #[test]
    fn test_part_2() {
        let problems: ColumnMathProblems = EXAMPLE_1.parse().unwrap();
        assert_eq!(problems.add_results(), 3263827);
    }
}
