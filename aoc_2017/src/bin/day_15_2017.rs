use nom::bytes::complete::take_till;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct DuelingGenerators {
    a: usize,
    b: usize,
}

impl DuelingGenerators {
    fn encounters(&self, nb_cycle: usize, only_multiples: bool) -> usize {
        let mut a: usize = self.a;
        let mut b: usize = self.b;

        let factor_a: usize = 16807;
        let factor_b: usize = 48271;
        let div: usize = 2147483647;
        let mask: usize = (u16::MAX as usize) + 1;

        let mut count: usize = 0;

        for _ in 0..nb_cycle {
            a = (a * factor_a) % div;
            b = (b * factor_b) % div;
            if only_multiples {
                while a % 4 != 0 {
                    a = (a * factor_a) % div;
                }
                while b % 8 != 0 {
                    b = (b * factor_b) % div;
                }
            }
            if a % mask == b % mask {
                count += 1;
            }
        }
        count
    }
}

impl FromStr for DuelingGenerators {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_seed(s: &str) -> IResult<&str, usize> {
            preceded(take_till(|c: char| c.is_ascii_digit()), parse_usize)(s)
        }

        let seeds: Vec<usize> = s.lines().map(|l| parse_seed(l).unwrap().1).collect();
        Ok(DuelingGenerators {
            a: seeds[0],
            b: seeds[1],
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_15.txt").expect("Cannot open input file");
    let duel: DuelingGenerators = s.parse().unwrap();

    println!(
        "Part1: The judge's count is {}",
        duel.encounters(40_000_000, false)
    );
    println!(
        "Part2: With the new rules, the judge's count is {}",
        duel.encounters(5_000_000, true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "Generator A starts with 65
Generator B starts with 8921
";

    #[test]
    fn part_1() {
        let duel: DuelingGenerators = EXAMPLE_1.parse().unwrap();
        assert_eq!(588, duel.encounters(40_000_000, false));
    }

    #[test]
    fn part_2() {
        let duel: DuelingGenerators = EXAMPLE_1.parse().unwrap();
        assert_eq!(309, duel.encounters(5_000_000, true));
    }
}
