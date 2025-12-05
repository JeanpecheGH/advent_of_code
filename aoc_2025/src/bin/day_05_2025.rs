use nom::IResult;
use nom::Parser;
use nom::character::char;
use nom::sequence::separated_pair;
use std::cmp::{max, min};
use std::ops::RangeInclusive;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

struct Cafeteria {
    fresh_ranges: Vec<RangeInclusive<usize>>,
    ingredients: Vec<usize>,
}

impl Cafeteria {
    fn nb_fresh_ingredients(&self) -> usize {
        self.ingredients
            .iter()
            .filter(|i| self.fresh_ranges.iter().any(|r| r.contains(i)))
            .count()
    }

    fn nb_fresh_ids(&self) -> usize {
        let mut final_ranges: Vec<RangeInclusive<usize>> = Vec::new();
        for r in &self.fresh_ranges {
            let mut merged_ranges: Vec<RangeInclusive<usize>> = Vec::new();
            let mut current: RangeInclusive<usize> = r.clone();

            while let Some(other) = final_ranges.pop() {
                // The range are disjoint, store the other range
                if current.start() > other.end() || current.end() < other.start() {
                    merged_ranges.push(other);
                } else {
                    // Merging the ranges
                    current =
                        min(*current.start(), *other.start())..=max(*current.end(), *other.end());
                }
            }
            //At the end, put your new built range in the new ranges
            merged_ranges.push(current);

            final_ranges = merged_ranges;
        }
        final_ranges.iter().map(|r| r.end() - r.start() + 1).sum()
    }
}

impl FromStr for Cafeteria {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_range(s: &str) -> IResult<&str, RangeInclusive<usize>> {
            let (s, (l, r)) = separated_pair(parse_usize, char('-'), parse_usize).parse(s)?;
            Ok((s, l..=r))
        }

        let blocks = split_blocks(s);
        let fresh_ranges: Vec<RangeInclusive<usize>> = blocks[0]
            .lines()
            .map(|l| parse_range(l).unwrap().1)
            .collect();
        let ingredients: Vec<usize> = blocks[1].lines().map(|l| l.parse().unwrap()).collect();

        Ok(Cafeteria {
            fresh_ranges,
            ingredients,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_05.txt").expect("Cannot open input file");
    let cafeteria: Cafeteria = s.parse().unwrap();

    println!(
        "Part1: There are {} fresh ingredients",
        cafeteria.nb_fresh_ingredients()
    );
    println!(
        "Part2: There are {} available fresh IDs",
        cafeteria.nb_fresh_ids()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32
";
    #[test]
    fn test_part_1() {
        let cafeteria: Cafeteria = EXAMPLE_1.parse().unwrap();
        assert_eq!(cafeteria.nb_fresh_ingredients(), 3);
    }

    #[test]
    fn test_part_2() {
        let cafeteria: Cafeteria = EXAMPLE_1.parse().unwrap();
        assert_eq!(cafeteria.nb_fresh_ids(), 14);
    }
}
