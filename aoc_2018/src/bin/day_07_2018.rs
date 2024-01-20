use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::str::FromStr;

struct Instructions {
    steps: Vec<(char, char)>,
}

impl Instructions {
    fn steps_and_blockers(&self) -> (FxHashSet<char>, FxHashMap<char, FxHashSet<char>>) {
        self.steps.iter().fold(
            (FxHashSet::default(), FxHashMap::default()),
            |(mut set, mut map), &(from, to)| {
                set.insert(from);
                set.insert(to);
                let e = map.entry(to).or_insert(FxHashSet::default());
                e.insert(from);

                (set, map)
            },
        )
    }

    fn execution_order(&self) -> String {
        //Build a map of "blockers"
        let (mut step_set, mut blockers): (FxHashSet<char>, FxHashMap<char, FxHashSet<char>>) =
            self.steps_and_blockers();

        let mut min_queue: BinaryHeap<Reverse<char>> = BinaryHeap::new();
        let mut result: Vec<char> = Vec::new();
        while !step_set.is_empty() {
            //Add nodes in set but not in map in the queue
            let free: Vec<char> = step_set
                .iter()
                .filter(|s| !blockers.keys().contains(s))
                .copied()
                .collect();

            //Remove those nodes from set
            for f in free {
                step_set.remove(&f);
                min_queue.push(Reverse(f));
            }

            //Apply the min element from the queue
            let min = min_queue.pop().unwrap().0;
            result.push(min);

            //Remove applied element from blockers and clean empty blockers
            for (_, v) in blockers.iter_mut() {
                v.remove(&min);
            }
            blockers.retain(|_, v| !v.is_empty());
        }

        result.into_iter().collect()
    }

    fn execution_order_slow(&self, nb_workers: usize, added_time: usize) -> (String, usize) {
        let (mut step_set, mut blockers): (FxHashSet<char>, FxHashMap<char, FxHashSet<char>>) =
            self.steps_and_blockers();

        let mut min_queue: BinaryHeap<Reverse<char>> = BinaryHeap::new();
        let mut result: Vec<char> = Vec::new();
        let mut workers: Vec<Option<(char, usize)>> = vec![None; nb_workers];
        let mut time: usize = 0;
        while !step_set.is_empty() || workers.iter().any(|w| w.is_some()) {
            //Add nodes in set but not in map in the queue
            let free: Vec<char> = step_set
                .iter()
                .filter(|s| !blockers.keys().contains(s))
                .copied()
                .collect();

            //Remove those nodes from set
            for f in free {
                step_set.remove(&f);
                min_queue.push(Reverse(f));
            }

            for w in workers.iter_mut() {
                if w.is_none() {
                    //Give available tasks to workers
                    *w = min_queue.pop().map(|rc| {
                        let c = rc.0;
                        let n = c as usize - b'A' as usize + 1 + added_time;
                        (c, n)
                    });
                }
                if let Some((c, t)) = w {
                    if *t > 0 {
                        //Advance tasks
                        *t -= 1;
                    }
                    if *t == 0 {
                        //End tasks
                        result.push(*c);

                        //Remove applied element from blockers and clean empty blockers
                        for (_, v) in blockers.iter_mut() {
                            v.remove(c);
                        }
                        *w = None;
                    }
                }
            }
            blockers.retain(|_, v| !v.is_empty());
            time += 1;
        }

        (result.into_iter().collect(), time)
    }
}

impl FromStr for Instructions {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_step(s: &str) -> IResult<&str, (char, char)> {
            let (s, (from, to)) = preceded(
                tag("Step "),
                separated_pair(anychar, tag(" must be finished before step "), anychar),
            )(s)?;
            Ok((s, (from, to)))
        }

        let steps: Vec<(char, char)> = s.lines().map(|l| parse_step(l).unwrap().1).collect();

        Ok(Instructions { steps })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_07.txt").expect("Cannot open input file");
    let instructions: Instructions = s.parse().unwrap();

    println!(
        "Part1: The instructions will be executed in order {}",
        instructions.execution_order()
    );
    let (order, time) = instructions.execution_order_slow(5, 60);
    println!("Part2: The instructions will be executed in order {order} in {time} seconds");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "Step C must be finished before step A can begin.
Step C must be finished before step F can begin.
Step A must be finished before step B can begin.
Step A must be finished before step D can begin.
Step B must be finished before step E can begin.
Step D must be finished before step E can begin.
Step F must be finished before step E can begin.
";

    #[test]
    fn part_1() {
        let instructions: Instructions = EXAMPLE_1.parse().unwrap();
        assert_eq!(instructions.execution_order(), "CABDFE");
    }
    #[test]
    fn part_2() {
        let instructions: Instructions = EXAMPLE_1.parse().unwrap();
        assert_eq!(
            instructions.execution_order_slow(2, 0),
            ("CABFDE".to_string(), 15)
        );
    }
}
