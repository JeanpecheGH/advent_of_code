use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
struct Bridge {
    last: usize,
    strength: usize,
    length: usize,
}

impl Ord for Bridge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.length
            .cmp(&other.length)
            .then(self.strength.cmp(&other.strength))
    }
}

impl PartialOrd for Bridge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Bridge {
    fn extend(&self, (a, b): (usize, usize)) -> Bridge {
        let last: usize = if self.last == a { b } else { a };
        let strength: usize = self.strength + a + b;
        Bridge {
            last,
            strength,
            length: self.length + 1,
        }
    }
}

#[derive(Debug, Clone)]
struct ElectromagneticMoat {
    ports_pair: Vec<(usize, usize)>,
}

impl ElectromagneticMoat {
    fn best_bridge(bridge: Bridge, pairs: Vec<(usize, usize)>, longest: bool) -> Bridge {
        if !pairs.is_empty() {
            //Build all possible bridges from the current bridge, get the max strength among those
            let new_bridges: Vec<Bridge> = pairs
                .iter()
                .enumerate()
                .filter_map(|(i, &(a, b))| {
                    if bridge.last == a || bridge.last == b {
                        let mut new_pairs: Vec<(usize, usize)> = pairs.clone();
                        new_pairs.swap_remove(i);

                        Some(ElectromagneticMoat::best_bridge(
                            bridge.extend((a, b)),
                            new_pairs,
                            longest,
                        ))
                    } else {
                        None
                    }
                })
                .collect();

            let best_bridge: Option<Bridge> = if longest {
                new_bridges.iter().max().copied()
            } else {
                new_bridges
                    .iter()
                    .max_by(|a, b| a.strength.cmp(&b.strength))
                    .copied()
            };
            best_bridge.unwrap_or(bridge)
        } else {
            bridge
        }
    }
    fn strongest_bridge(&self) -> usize {
        ElectromagneticMoat::best_bridge(Bridge::default(), self.ports_pair.clone(), false).strength
    }

    fn longest_bridge(&self) -> usize {
        ElectromagneticMoat::best_bridge(Bridge::default(), self.ports_pair.clone(), true).strength
    }
}

impl FromStr for ElectromagneticMoat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (usize, usize)> {
            separated_pair(parse_usize, char('/'), parse_usize)(s)
        }

        let ports_pair: Vec<(usize, usize)> = s.lines().map(|l| parse_pair(l).unwrap().1).collect();

        Ok(ElectromagneticMoat { ports_pair })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_24.txt").expect("Cannot open input file");
    let moat: ElectromagneticMoat = s.parse().unwrap();

    println!(
        "Part1: The strongest bridge we can build has {} strength",
        moat.strongest_bridge()
    );
    println!(
        "Part2: The longest bridge we can build has {} strength",
        moat.longest_bridge()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0/2
2/2
2/3
3/4
3/5
0/1
10/1
9/10
";
    #[test]
    fn part_1() {
        let moat: ElectromagneticMoat = EXAMPLE_1.parse().unwrap();
        assert_eq!(31, moat.strongest_bridge());
    }
    #[test]
    fn part_2() {
        let moat: ElectromagneticMoat = EXAMPLE_1.parse().unwrap();
        assert_eq!(19, moat.longest_bridge());
    }
}
