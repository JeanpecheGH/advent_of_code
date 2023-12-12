use nom::bytes::complete::{tag, take};
use nom::sequence::{preceded, tuple};
use nom::IResult;
use std::collections::HashMap;
use std::str::FromStr;
use util::orientation::Dir;
use util::{lcm, split_blocks};

struct Wasteland {
    dirs: Vec<Dir>,
    nodes: HashMap<String, (String, String)>,
}

impl Wasteland {
    fn steps(&self) -> usize {
        let mut steps = 0;
        let mut curr_node: &str = "AAA";

        while curr_node != "ZZZ" {
            let dir_step: usize = steps % self.dirs.len();
            let (left, right) = self.nodes.get(curr_node).unwrap();
            curr_node = match self.dirs[dir_step] {
                Dir::West => left,
                Dir::East => right,
                _ => panic!("Invalid direction"),
            };

            steps += 1
        }
        steps
    }

    fn multi_steps(&self) -> usize {
        let mut steps: usize = 0;
        let mut curr_nodes: Vec<&str> = self
            .nodes
            .keys()
            .filter_map(|k| {
                if k.ends_with('A') {
                    Some(k.as_str())
                } else {
                    None
                }
            })
            .collect();

        let mut dist_to_z: Vec<usize> = vec![0; curr_nodes.len()];

        while dist_to_z.iter().any(|&n| n == 0) {
            let dir_step: usize = steps % self.dirs.len();
            curr_nodes = curr_nodes
                .into_iter()
                .enumerate()
                .map(|(i, key)| {
                    let (left, right) = self.nodes.get(key).unwrap();
                    let new_key = match self.dirs[dir_step] {
                        Dir::West => left,
                        Dir::East => right,
                        _ => panic!("Invalid direction"),
                    };
                    if new_key.ends_with('Z') {
                        dist_to_z[i] = steps + 1
                    }
                    new_key.as_str()
                })
                .collect();

            steps += 1;
        }
        //Each cycle is equal to the first distance, so we can apply LCM immediately
        dist_to_z.iter().fold(1, |acc, &n| lcm(acc, n as isize)) as usize
    }
}

impl FromStr for Wasteland {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_dict(s: &str) -> IResult<&str, (String, (String, String))> {
            let (s, (k, l, r)) = tuple((
                take(3usize),
                preceded(tag(" = ("), take(3usize)),
                preceded(tag(", "), take(3usize)),
            ))(s)?;

            Ok((s, (k.to_string(), (l.to_string(), r.to_string()))))
        }

        let blocks: Vec<&str> = split_blocks(s);
        let dirs: Vec<Dir> = blocks[0]
            .chars()
            .map(|c| match c {
                'L' => Dir::West,
                'R' => Dir::East,
                _ => Dir::North,
            })
            .collect();

        let nodes: HashMap<String, (String, String)> = blocks[1]
            .lines()
            .map(|line| parse_dict(line).unwrap().1)
            .collect();

        Ok(Wasteland { dirs, nodes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_08.txt").expect("Cannot open input file");
    let wasteland: Wasteland = s.parse().unwrap();

    println!(
        "Part1: {} steps are required to reach ZZZ from AAA",
        wasteland.steps()
    );
    println!("Part2: It takes {} steps to end on nodes ending with Z when starting on all the nodes ending with A", wasteland.multi_steps());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";

    const EXAMPLE_2: &str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

    #[test]
    fn part_1() {
        let wasteland: Wasteland = EXAMPLE_1.parse().unwrap();
        assert_eq!(wasteland.steps(), 6);
    }
    #[test]
    fn part_2() {
        let wasteland: Wasteland = EXAMPLE_2.parse().unwrap();
        assert_eq!(wasteland.multi_steps(), 6);
    }
}
