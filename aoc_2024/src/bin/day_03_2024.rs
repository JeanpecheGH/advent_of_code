use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::combinator::{map, value};
use nom::multi::fold_many1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

type MemoryValue = (bool, usize);

struct Memory {
    values: Vec<MemoryValue>,
}

impl Memory {
    fn multiply(&self, all: bool) -> usize {
        self.values
            .iter()
            .map(|(add, v)| v * (add | all) as usize)
            .sum()
    }
}

enum ParseToken {
    Switch(bool),
    Value(usize),
    Nothing,
}

impl FromStr for Memory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_mul(s: &str) -> IResult<&str, ParseToken> {
            let (s, _) = tag("mul(")(s)?;
            let (s, (a, b)) = separated_pair(parse_usize, char(','), parse_usize).parse(s)?;
            let (s, _) = char(')')(s)?;
            Ok((s, ParseToken::Value(a * b)))
        }

        fn parse_do(s: &str) -> IResult<&str, ParseToken> {
            let (s, b) = alt((value(true, tag("do()")), value(false, tag("don't()")))).parse(s)?;
            Ok((s, ParseToken::Switch(b)))
        }

        fn parse_token(s: &str) -> IResult<&str, ParseToken> {
            let (s, token) =
                alt((parse_do, parse_mul, map(anychar, |_| ParseToken::Nothing))).parse(s)?;

            Ok((s, token))
        }

        fn parse_memory(s: &str) -> IResult<&str, Vec<MemoryValue>> {
            let (s, (_, v)): (&str, (bool, Vec<MemoryValue>)) = fold_many1(
                parse_token,
                || (true, Vec::new()),
                |(mut enabled, mut acc): (bool, Vec<MemoryValue>), token: ParseToken| {
                    match token {
                        ParseToken::Switch(b) => enabled = b,
                        ParseToken::Value(v) => acc.push((enabled, v)),
                        ParseToken::Nothing => (),
                    };
                    (enabled, acc)
                },
            )
            .parse(s)?;
            Ok((s, v))
        }
        let values: Vec<MemoryValue> = parse_memory(s).unwrap().1;

        Ok(Memory { values })
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
