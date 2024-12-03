use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::combinator::{map, opt, value};
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;

struct Memory {
    pairs: Vec<(bool, usize, usize)>,
}

impl Memory {
    fn multiply(&self, all: bool) -> usize {
        self.pairs
            .iter()
            .map(|(add, a, b)| a * b * (add | all) as usize)
            .sum()
    }
}

impl FromStr for Memory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (usize, usize)> {
            let (s, _) = tag("mul(")(s)?;
            let (s, a) = parse_usize(s)?;
            let (s, _) = char(',')(s)?;
            let (s, b) = parse_usize(s)?;
            let (s, _) = char(')')(s)?;
            Ok((s, (a, b)))
        }

        fn parse_do(s: &str) -> IResult<&str, bool> {
            let (s, b) = alt((value(true, tag("do()")), value(false, tag("don't()"))))(s)?;
            Ok((s, b))
        }

        fn parse_cond(s: &str, mut do_mul: bool) -> IResult<&str, (bool, usize, usize)> {
            let (s, b_opt) = opt(parse_do)(s)?;
            b_opt.iter().for_each(|&b| do_mul = b);

            let (s, (a, b)) = alt((map(parse_pair, |(a, b)| (a, b)), map(anychar, |_| (0, 0))))(s)?;

            Ok((s, (do_mul, a, b)))
        }

        fn parse_memory(mut s: &str) -> IResult<&str, Vec<(bool, usize, usize)>> {
            let mut do_mul: bool = true;
            let mut v: Vec<(bool, usize, usize)> = Vec::new();
            while !s.is_empty() {
                let (rest, triplet): (&str, (bool, usize, usize)) = parse_cond(s, do_mul)?;
                do_mul = triplet.0;
                v.push(triplet);
                s = rest;
            }
            Ok((s, v))
        }
        let pairs: Vec<(bool, usize, usize)> = parse_memory(s).unwrap().1;

        Ok(Memory { pairs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_03.txt").expect("Cannot open input file");
    let memory: Memory = s.parse().unwrap();

    println!(
        "Part1: The sum of all valid multiplications is {}",
        memory.multiply(true)
    );
    println!(
        "Part2: When disabling some multiplications, the sum is {}",
        memory.multiply(false)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str =
        "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";

    const EXAMPLE_2: &str =
        "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
    #[test]
    fn part_1() {
        let memory: Memory = EXAMPLE_1.parse().unwrap();
        assert_eq!(memory.multiply(true), 161);
    }
    #[test]
    fn part_2() {
        let memory: Memory = EXAMPLE_2.parse().unwrap();
        assert_eq!(memory.multiply(false), 48);
    }
}
