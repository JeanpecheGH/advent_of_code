use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::HashSet;
use std::str::FromStr;
use util::basic_parser::parse_isize;

struct Calibration {
    changes: Vec<isize>,
}

impl Calibration {
    fn frequency(&self) -> isize {
        self.changes.iter().sum()
    }

    fn repeat_frequency(&self) -> isize {
        let mut freq_set: HashSet<isize> = HashSet::new();
        let mut curr: isize = 0;
        freq_set.insert(curr);

        loop {
            for c in self.changes.iter() {
                curr += c;

                if !freq_set.insert(curr) {
                    return curr;
                }
            }
        }
    }
}

impl FromStr for Calibration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_change(s: &str) -> IResult<&str, isize> {
            let (s, change) = preceded(opt(char('+')), parse_isize)(s)?;

            Ok((s, change))
        }

        let changes: Vec<isize> = s.lines().map(|l| parse_change(l).unwrap().1).collect();

        Ok(Calibration { changes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_01.txt").expect("Cannot open input file");
    let cal: Calibration = s.parse().unwrap();

    println!("Part1: The resulting frequency is {}", cal.frequency());
    println!(
        "Part2: The first repeated frequency is {}",
        cal.repeat_frequency()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "+1
-2
+3
+1";

    #[test]
    fn part_1() {
        let cal: Calibration = EXAMPLE_1.parse().unwrap();
        assert_eq!(cal.frequency(), 3);
    }
    #[test]
    fn part_2() {
        let cal: Calibration = EXAMPLE_1.parse().unwrap();
        assert_eq!(cal.repeat_frequency(), 2);
    }
}
