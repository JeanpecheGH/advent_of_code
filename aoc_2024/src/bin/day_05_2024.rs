use fxhash::FxHashMap;
use itertools::Itertools;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Clone)]
struct OrderingRules {
    rules: FxHashMap<usize, Vec<usize>>,
}

impl OrderingRules {
    fn cmp(&self, a: usize, b: usize) -> Ordering {
        if let Some(v) = self.rules.get(&a) {
            if v.contains(&b) {
                return Ordering::Less;
            }
        }
        Ordering::Greater
    }
}

impl FromStr for OrderingRules {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (usize, usize)> {
            separated_pair(parse_usize, char('|'), parse_usize).parse(s)
        }
        let mut rules: FxHashMap<usize, Vec<usize>> = FxHashMap::default();
        s.lines().for_each(|l| {
            let (left, right) = parse_pair(l).unwrap().1;
            rules.entry(left).or_default().push(right);
        });
        Ok(OrderingRules { rules })
    }
}

#[derive(Clone)]
struct Update {
    pages: Vec<usize>,
}

impl Update {
    fn middle(&self) -> usize {
        self.pages[self.pages.len() / 2]
    }

    fn well_ordered(&self, rules: &OrderingRules) -> bool {
        self.pages
            .is_sorted_by(|&a, &b| rules.cmp(a, b) == Ordering::Less)
    }

    fn sorted(&self, rules: &OrderingRules) -> Self {
        let sorted_pages = self
            .pages
            .iter()
            .sorted_unstable_by(|&&a, &&b| rules.cmp(a, b))
            .copied()
            .collect();

        Update {
            pages: sorted_pages,
        }
    }
}

impl FromStr for Update {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_list(s: &str) -> IResult<&str, Vec<usize>> {
            separated_list1(char(','), parse_usize).parse(s)
        }
        let pages = parse_list(s).unwrap().1;
        Ok(Update { pages })
    }
}

struct SafetyManual {
    rules: OrderingRules,
    updates: Vec<Update>,
}

impl SafetyManual {
    fn solve(&self) -> (usize, usize) {
        self.updates
            .iter()
            .fold((0, 0), |(sorted, not_sorted), update| {
                if update.well_ordered(&self.rules) {
                    (sorted + update.middle(), not_sorted)
                } else {
                    (sorted, not_sorted + update.sorted(&self.rules).middle())
                }
            })
    }
}

impl FromStr for SafetyManual {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = split_blocks(s);
        let rules: OrderingRules = blocks[0].parse()?;
        let updates: Vec<Update> = blocks[1].lines().map(|l| l.parse().unwrap()).collect();

        Ok(SafetyManual { rules, updates })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_05.txt").expect("Cannot open input file");
    let manual: SafetyManual = s.parse().unwrap();

    let (part_1, part_2) = manual.solve();
    println!(
        "Part1: The sum of the middle page number of the correctly-sorted updates is {part_1}"
    );
    println!(
        "Part2: After sorting the other updates, the sum of their middle page number is {part_2}"
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47
";

    #[test]
    fn test() {
        let manual: SafetyManual = EXAMPLE_1.parse().unwrap();
        assert_eq!(manual.solve(), (143, 123));
    }
}
