use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct Firewall {
    scanners: Vec<(usize, usize)>,
}

impl Firewall {
    fn trip_severity(&self) -> usize {
        self.scanners
            .iter()
            .map(|&(depth, range)| {
                let cycle_length = (range - 1) * 2;
                if depth % cycle_length == 0 {
                    depth * range
                } else {
                    0
                }
            })
            .sum()
    }

    fn pico_delay(&self) -> usize {
        (0..usize::MAX)
            .find(|x| {
                !self.scanners.iter().any(|&(depth, range)| {
                    let cycle_length = (range - 1) * 2;
                    (depth + x) % cycle_length == 0
                })
            })
            .unwrap()
    }
}

impl FromStr for Firewall {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_scanner(s: &str) -> IResult<&str, (usize, usize)> {
            let (s, (depth, range)) =
                separated_pair(parse_usize, tag(": "), parse_usize).parse(s)?;

            Ok((s, (depth, range)))
        }

        let scanners: Vec<(usize, usize)> =
            s.lines().map(|l| parse_scanner(l).unwrap().1).collect();
        Ok(Firewall { scanners })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_13.txt").expect("Cannot open input file");
    let firewall: Firewall = s.parse().unwrap();

    println!("Part1: The trip severity is {}", firewall.trip_severity());
    println!(
        "Part2: We need to wait {} picoseconds before sending the packet",
        firewall.pico_delay()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0: 3
1: 2
4: 4
6: 4
";

    #[test]
    fn part_1() {
        let firewall: Firewall = EXAMPLE_1.parse().unwrap();
        assert_eq!(24, firewall.trip_severity());
    }

    #[test]
    fn part_2() {
        let firewall: Firewall = EXAMPLE_1.parse().unwrap();
        assert_eq!(10, firewall.pico_delay());
    }
}
