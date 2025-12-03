use itertools::Itertools;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::cmp::min;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct CrabSwarm {
    crabs: Vec<usize>,
}

impl CrabSwarm {
    fn minimum_fuel(&self) -> (usize, usize) {
        fn distance_to(target: usize, from: usize, nb_from: usize, quadratic: bool) -> usize {
            let dist: usize = target.abs_diff(from);
            if quadratic {
                nb_from * dist * (dist + 1) / 2
            } else {
                nb_from * dist
            }
        }

        let size = self.crabs.len();
        let counts: Vec<(usize, usize)> = self
            .crabs
            .iter()
            .copied()
            .counts()
            .into_iter()
            .sorted_by(|(k1, _), (k2, _)| k1.cmp(k2))
            .collect();

        let mut median: usize = 0;
        let mut mean: usize = 0;
        let mut nb_values: usize = 0;

        for &(v, n) in &counts {
            nb_values += n;
            if median == 0 && nb_values >= size / 2 {
                median = v;
            }

            mean += v * n;
        }
        mean /= size;

        let (linear, quad_low, quad_high) = counts.into_iter().fold(
            (0, 0, 0),
            |(mut linear, mut quad_low, mut quad_high), (v, n)| {
                linear += distance_to(median, v, n, false);
                quad_low += distance_to(mean, v, n, true);
                quad_high += distance_to(mean + 1, v, n, true);
                (linear, quad_low, quad_high)
            },
        );

        (linear, min(quad_low, quad_high))
    }
}

impl FromStr for CrabSwarm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(s: &str) -> IResult<&str, Vec<usize>> {
            separated_list1(char(','), parse_usize).parse(s)
        }

        let crabs: Vec<usize> = s.lines().next().map(|l| parse_line(l).unwrap().1).unwrap();
        Ok(CrabSwarm { crabs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_07.txt").expect("Cannot open input file");
    let swarm: CrabSwarm = s.parse().unwrap();

    let (linear, quadratic) = swarm.minimum_fuel();
    println!("Part1: We need {linear} fuel for crab to align");
    println!("Part2: With a quadratic fuel consumption, aligning the crabs costs {quadratic} fuel");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "16,1,2,0,4,2,7,1,2,14";

    #[test]
    fn part_1() {
        let swarm: CrabSwarm = EXAMPLE_1.parse().unwrap();
        assert_eq!((37, 168), swarm.minimum_fuel());
    }
}
