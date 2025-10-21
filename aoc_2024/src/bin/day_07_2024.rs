use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;
use rayon::prelude::*;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct Equation {
    total: usize,
    values: Vec<usize>,
}

impl Equation {
    // Solving from right to left is faster
    fn is_solvable(&self, with_concat: bool) -> bool {
        fn mul(
            rest: &[usize],
            target: usize,
            current: usize,
            last: usize,
            with_concat: bool,
        ) -> bool {
            if current.is_multiple_of(last) {
                inner(rest, target, current / last, with_concat)
            } else {
                false
            }
        }

        fn concat(
            rest: &[usize],
            target: usize,
            current: usize,
            last: usize,
            with_concat: bool,
        ) -> bool {
            let d: usize = 10usize.pow(last.ilog10() + 1);
            if current % d == last {
                inner(rest, target, current / d, with_concat)
            } else {
                false
            }
        }

        fn inner(rest: &[usize], target: usize, current: usize, with_concat: bool) -> bool {
            if current < target {
                return false;
            }
            if rest.is_empty() {
                target == current
            } else {
                let len = rest.len() - 1;
                let new_rest = &rest[..len];
                let last: usize = rest[len];
                inner(new_rest, target, current - last, with_concat)
                    || mul(new_rest, target, current, last, with_concat)
                    || (with_concat && concat(new_rest, target, current, last, with_concat))
            }
        }
        inner(&self.values[1..], self.values[0], self.total, with_concat)
    }
}

impl FromStr for Equation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_equation(s: &str) -> IResult<&str, Equation> {
            let (s, total) = terminated(parse_usize, tag(": "))(s)?;
            let (s, values) = separated_list1(char(' '), parse_usize)(s)?;

            Ok((s, Equation { total, values }))
        }

        Ok(parse_equation(s).unwrap().1)
    }
}

struct BridgeRepair {
    equations: Vec<Equation>,
}

impl BridgeRepair {
    fn calibration(&self) -> (usize, usize) {
        let (solvable, rest): (Vec<Equation>, Vec<Equation>) = self
            .equations
            .clone()
            .into_par_iter()
            .partition(|eq| eq.is_solvable(false));
        let calibration: usize = solvable.iter().map(|eq| eq.total).sum();
        let calibration_with_concat: usize = rest
            .par_iter()
            .filter(|eq| eq.is_solvable(true))
            .map(|eq| eq.total)
            .sum();
        (calibration, calibration + calibration_with_concat)
    }
}

impl FromStr for BridgeRepair {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let equations: Vec<Equation> = s.lines().map(|l| l.parse().unwrap()).collect();

        Ok(BridgeRepair { equations })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_07.txt").expect("Cannot open input file");
    let bridge: BridgeRepair = s.parse().unwrap();
    let (calibration, calibration_with_concat) = bridge.calibration();
    println!("Part1: The total calibration result is {}", calibration);
    println!(
        "Part2: When adding the concatenation operation, the total is now {}",
        calibration_with_concat
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20
";
    #[test]
    fn test() {
        let bridge: BridgeRepair = EXAMPLE_1.parse().unwrap();
        assert_eq!(bridge.calibration(), (3749, 11387));
    }
}
