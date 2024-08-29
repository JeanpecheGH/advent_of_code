use fxhash::FxHashMap;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;
use util::split_blocks;

#[derive(Debug, Clone)]
struct Polymerization {
    template: String,
    insertion_rules: FxHashMap<(char, char), char>,
}

impl Polymerization {
    fn pair_insertion(&self, times: usize) -> usize {
        let chars: Vec<char> = self.template.chars().collect();
        let first: char = chars.first().copied().unwrap();
        let last: char = chars.iter().last().copied().unwrap();

        //Create the pairs from the starting template
        let mut pair_map: FxHashMap<(char, char), usize> = FxHashMap::default();
        chars.windows(2).for_each(|pair| {
            let key: (char, char) = (pair[0], pair[1]);
            *pair_map.entry(key).or_default() += 1;
        });

        //Each pair is simply divided into 2 new pairs at each step
        for _ in 0..times {
            let mut new_pair_map: FxHashMap<(char, char), usize> = FxHashMap::default();
            for (&(c1, c2), &v) in &pair_map {
                let insert: char = self.insertion_rules.get(&(c1, c2)).copied().unwrap();

                *new_pair_map.entry((c1, insert)).or_default() += v;
                *new_pair_map.entry((insert, c2)).or_default() += v;
            }
            pair_map = new_pair_map;
        }

        //Split each pair into each char
        let mut char_map: FxHashMap<char, usize> = FxHashMap::default();
        for (&(c1, c2), &v) in &pair_map {
            *char_map.entry(c1).or_default() += v;
            *char_map.entry(c2).or_default() += v;
        }
        //Don't forget to add back the first and last char
        *char_map.entry(first).or_default() += 1;
        *char_map.entry(last).or_default() += 1;

        if let MinMax(min, max) = char_map.values().map(|&n| n / 2).minmax() {
            max - min
        } else {
            0
        }
    }
}

impl FromStr for Polymerization {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_insertion(s: &str) -> IResult<&str, ((char, char), char)> {
            let (s, a) = anychar(s)?;
            let (s, b) = anychar(s)?;
            let (s, t) = preceded(tag(" -> "), anychar)(s)?;
            Ok((s, ((a, b), t)))
        }

        let blocks = split_blocks(s);

        let template: String = blocks[0].lines().next().unwrap().to_string();
        let insertion_rules: FxHashMap<(char, char), char> = blocks[1]
            .lines()
            .map(|l| parse_insertion(l).unwrap().1)
            .collect();

        Ok(Polymerization {
            template,
            insertion_rules,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_14.txt").expect("Cannot open input file");
    let poly: Polymerization = s.parse().unwrap();
    println!(
        "Part1: After 10 steps, the difference between the most and least common elements is {}",
        poly.pair_insertion(10)
    );
    println!(
        "Part2: After 40 steps, it's now {}",
        poly.pair_insertion(40)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C
";

    #[test]
    fn part_1() {
        let poly: Polymerization = EXAMPLE_1.parse().unwrap();
        assert_eq!(1588, poly.pair_insertion(10));
    }

    #[test]
    fn part_2() {
        let poly: Polymerization = EXAMPLE_1.parse().unwrap();
        assert_eq!(2188189693529, poly.pair_insertion(40));
    }
}
