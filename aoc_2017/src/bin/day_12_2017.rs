use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct DigitalPlumbing {
    pipes: FxHashMap<usize, Vec<usize>>,
}

impl DigitalPlumbing {
    fn group(&self, id: usize) -> FxHashSet<usize> {
        let mut group_set: FxHashSet<usize> = FxHashSet::default();
        let mut queue: Vec<usize> = vec![id];

        group_set.insert(id);

        while let Some(current) = queue.pop() {
            let neighbours: &[usize] = self.pipes.get(&current).unwrap();

            for n in neighbours {
                if group_set.insert(*n) {
                    queue.push(*n);
                }
            }
        }

        group_set
    }

    fn group_zero(&self) -> usize {
        self.group(0).len()
    }

    fn nb_groups(&self) -> usize {
        let mut pipes: FxHashMap<usize, Vec<usize>> = self.pipes.clone();

        if let Some(&start_id) = pipes.keys().last() {
            let set = self.group(start_id);
            for id in set {
                pipes.remove(&id);
            }

            let sub_plumbing = DigitalPlumbing { pipes };
            sub_plumbing.nb_groups() + 1
        } else {
            0
        }
    }
}

impl FromStr for DigitalPlumbing {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pipe(s: &str) -> IResult<&str, (usize, Vec<usize>)> {
            let (s, source) = terminated(parse_usize, tag(" <-> ")).parse(s)?;
            let (s, targets) = separated_list1(tag(", "), parse_usize).parse(s)?;

            Ok((s, (source, targets)))
        }

        let pipes: FxHashMap<usize, Vec<usize>> =
            s.lines().map(|l| parse_pipe(l).unwrap().1).collect();
        Ok(DigitalPlumbing { pipes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_12.txt").expect("Cannot open input file");
    let plumbing: DigitalPlumbing = s.parse().unwrap();

    println!(
        "Part1: Program 0 is contained in a group of {} programs",
        plumbing.group_zero()
    );
    println!(
        "Part2: There are {} groups of program",
        plumbing.nb_groups()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0 <-> 2
1 <-> 1
2 <-> 0, 3, 4
3 <-> 2, 4
4 <-> 2, 3, 6
5 <-> 6
6 <-> 4, 5
";

    #[test]
    fn part_1() {
        let plumbing: DigitalPlumbing = EXAMPLE_1.parse().unwrap();
        assert_eq!(6, plumbing.group_zero());
    }

    #[test]
    fn part_2() {
        let plumbing: DigitalPlumbing = EXAMPLE_1.parse().unwrap();
        assert_eq!(2, plumbing.nb_groups());
    }
}
