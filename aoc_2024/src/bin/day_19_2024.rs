use fxhash::FxHashMap;
use nom::bytes::complete::tag;
use nom::character::complete::alpha1;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::split_blocks;

struct LinenLayout {
    towels: Vec<String>,
    designs: Vec<String>,
}
impl LinenLayout {
    fn nb_possible(&self, design: &str, cache: &mut FxHashMap<String, usize>) -> usize {
        if let Some(&b) = cache.get(design) {
            return b;
        }
        let mut nb_possible = 0;

        for t in &self.towels {
            if design.starts_with(t) {
                nb_possible += self.nb_possible(&design[t.len()..], cache);
            }
        }
        cache.insert(design.to_string(), nb_possible);
        nb_possible
    }

    fn solve(&self) -> (usize, usize) {
        let mut cache: FxHashMap<String, usize> = FxHashMap::default();
        //An empty design is valid
        cache.insert("".to_string(), 1);
        let valid_designs: Vec<usize> = self
            .designs
            .iter()
            .map(|d| self.nb_possible(d, &mut cache))
            .filter(|&nb| nb > 0)
            .collect();
        (valid_designs.len(), valid_designs.iter().sum())
    }
}

impl FromStr for LinenLayout {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_towels(s: &str) -> IResult<&str, Vec<&str>> {
            let (s, towels) = separated_list1(tag(", "), alpha1).parse(s)?;
            Ok((s, towels))
        }

        let blocks: Vec<&str> = split_blocks(s);
        let towels: Vec<String> = parse_towels(blocks[0])
            .unwrap()
            .1
            .iter()
            .map(|s| s.to_string())
            .collect();
        let designs: Vec<String> = blocks[1].lines().map(|l| l.to_string()).collect();

        Ok(LinenLayout { towels, designs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_19.txt").expect("Cannot open input file");
    let layout: LinenLayout = s.parse().unwrap();
    let (nb_valid_designs, nb_total_designs): (usize, usize) = layout.solve();
    println!("Part1: {} designs are possible", nb_valid_designs);
    println!(
        "Part2: The sum of the ways you can arrange each design is {}",
        nb_total_designs
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb
";

    #[test]
    fn test_1() {
        let layout: LinenLayout = EXAMPLE_1.parse().unwrap();
        assert_eq!(layout.solve(), (6, 16));
    }
}
