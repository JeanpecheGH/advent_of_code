use itertools::Itertools;
use nom::character::complete::char;
use nom::combinator::opt;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use std::collections::HashSet;
use std::str::FromStr;
use util::basic_parser::parse_isize;

struct Calibration {
    changes: Vec<isize>,
}

impl Calibration {
    fn frequencies(&self) -> (Vec<isize>, Option<isize>) {
        let mut n: isize = 0;
        let mut repeat: Option<isize> = None;
        let mut set: HashSet<isize> = HashSet::new();
        set.insert(0);

        let mut v: Vec<isize> = self
            .changes
            .iter()
            .map(|c| {
                n += c;
                if !set.insert(n) && repeat.is_none() {
                    repeat = Some(n);
                }
                n
            })
            .collect();

        let mut zero: Vec<isize> = vec![0];
        zero.append(&mut v);

        (zero, repeat)
    }

    fn repeat_frequency(&self) -> (isize, Option<isize>) {
        let (mut v, repeat): (Vec<isize>, Option<isize>) = self.frequencies();
        let incr: isize = v.pop().unwrap();

        if repeat.is_some() {
            return (incr, repeat);
        }
        let repeat: Option<isize> = v
            .into_iter()
            .enumerate()
            .into_group_map_by(|(_, n)| ((n % incr) + incr) % incr)
            .into_iter()
            .filter(|(_, pairs)| pairs.len() > 1)
            .map(|(_, mut pairs)| {
                if incr > 0 {
                    pairs.sort_by(|(_, a), (_, b)| a.cmp(b))
                } else {
                    pairs.sort_by(|(_, a), (_, b)| b.cmp(a))
                }
                let triplet: (isize, usize, isize) = pairs.windows(2).fold(
                    (isize::MAX, usize::MAX, 0),
                    |(cycle, idx, val), pair| {
                        let (i, a) = pair[0];
                        let b = pair[1].1;
                        let div: isize = (b - a) / incr;
                        if cycle > div || (cycle == div && idx > i) {
                            (div, i, b)
                        } else {
                            (cycle, idx, val)
                        }
                    },
                );
                triplet
            })
            .min_by(|a, b| a.0.cmp(&b.0).then(a.1.cmp(&b.1)))
            .map(|t| t.2);
        (incr, repeat)
    }
}

impl FromStr for Calibration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_change(s: &str) -> IResult<&str, isize> {
            let (s, change) = preceded(opt(char('+')), parse_isize).parse(s)?;

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
    let (freq, repeat) = cal.repeat_frequency();

    println!("Part1: The resulting frequency is {}", freq);
    println!("Part2: The first repeated frequency is {}", repeat.unwrap());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "+1
-2
+3
+1";
    const EXAMPLE_2: &str = "+100000000
-99999999";

    #[test]
    fn test_1() {
        let cal: Calibration = EXAMPLE_1.parse().unwrap();
        assert_eq!(cal.repeat_frequency(), (3, Some(2)));
    }
    #[test]
    fn test_2() {
        let cal: Calibration = EXAMPLE_2.parse().unwrap();
        assert_eq!(cal.repeat_frequency(), (1, Some(100000000)));
    }
}
