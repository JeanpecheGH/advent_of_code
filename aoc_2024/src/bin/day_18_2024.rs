use fxhash::{FxHashMap, FxHashSet};
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct RamNode {
    pos: Pos,
    score: usize,
    h: usize,
}

impl RamNode {
    fn heuristic(&self) -> usize {
        self.score + self.h
    }
}

impl Ord for RamNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heuristic()
            .cmp(&self.heuristic())
            .then(other.score.cmp(&self.score))
    }
}

impl PartialOrd for RamNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct RamRun {
    falling_bytes: Vec<Pos>,
}
impl RamRun {
    fn shortest_path(&self, size: usize, nb_fallen: usize) -> usize {
        let fallen_bytes: FxHashSet<Pos> =
            self.falling_bytes.iter().take(nb_fallen).copied().collect();
        let end: Pos = Pos(size, size);
        let starting_node: RamNode = RamNode {
            pos: Pos(0, 0),
            score: 0,
            h: Pos(0, 0).distance(end),
        };
        let mut cache: FxHashMap<Pos, usize> = FxHashMap::default();
        cache.insert(Pos(0, 0), 0);
        let mut priority_queue: BinaryHeap<RamNode> = BinaryHeap::default();
        priority_queue.push(starting_node);

        while let Some(node) = priority_queue.pop() {
            if node.pos == end {
                return node.score;
            }
            node.pos
                .neighbours_safe(size + 1, size + 1)
                .into_iter()
                .filter(|pos| !fallen_bytes.contains(pos))
                .map(|pos| RamNode {
                    pos,
                    score: node.score + 1,
                    h: pos.distance(end),
                })
                .for_each(|n| {
                    let min = cache.entry(n.pos).or_insert(usize::MAX);
                    if n.score < *min {
                        *min = n.score;
                        priority_queue.push(n);
                    }
                });
        }

        0
    }

    fn blocking_byte(&self, size: usize, min_fallen: usize) -> Pos {
        fn forms_wall(fallen_bytes: &FxHashSet<Pos>, size: usize) -> bool {
            let mut horizontal_bytes: FxHashSet<Pos> = FxHashSet::default();
            let mut vertical_bytes: FxHashSet<Pos> = FxHashSet::default();
            let mut current_h_bytes: Vec<Pos> = fallen_bytes
                .iter()
                .filter(|Pos(x, _)| *x == 0)
                .copied()
                .collect();
            let mut current_v_bytes: Vec<Pos> = fallen_bytes
                .iter()
                .filter(|Pos(_, y)| *y == 0)
                .copied()
                .collect();
            horizontal_bytes.extend(current_h_bytes.clone());
            vertical_bytes.extend(current_v_bytes.clone());

            while !current_v_bytes.is_empty() || !current_h_bytes.is_empty() {
                current_h_bytes = current_h_bytes
                    .into_iter()
                    .flat_map(|p| p.neighbours_diag_safe(size + 1, size + 1))
                    .filter(|p| fallen_bytes.contains(p))
                    .filter(|&p| horizontal_bytes.insert(p))
                    .collect();
                current_v_bytes = current_v_bytes
                    .into_iter()
                    .flat_map(|p| p.neighbours_diag_safe(size + 1, size + 1))
                    .filter(|p| fallen_bytes.contains(p))
                    .filter(|&p| vertical_bytes.insert(p))
                    .collect();
            }

            horizontal_bytes.iter().any(|&Pos(x, _)| x == size)
                || vertical_bytes.iter().any(|&Pos(_, y)| y == size)
        }

        let mut i: usize = min_fallen;

        while i < self.falling_bytes.len() {
            let fallen_bytes: FxHashSet<Pos> = self.falling_bytes.iter().take(i).copied().collect();
            if forms_wall(&fallen_bytes, size) {
                return self.falling_bytes[i - 1];
            }
            i += 1;
        }

        Pos(0, 0)
    }
}

impl FromStr for RamRun {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> IResult<&str, Pos> {
            let (s, (x, y)) = separated_pair(parse_usize, char(','), parse_usize)(s)?;
            Ok((s, Pos(x, y)))
        }

        let falling_bytes: Vec<Pos> = s.lines().map(|l| parse_pos(l).unwrap().1).collect();

        Ok(RamRun { falling_bytes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_18.txt").expect("Cannot open input file");
    let ram: RamRun = s.parse().unwrap();
    println!(
        "Part1: After 1024 bytes have fallen, we need {} steps to reach the exit",
        ram.shortest_path(70, 1024)
    );
    let block: Pos = ram.blocking_byte(70, 1024);
    println!(
        "Part2: The byte falling at coordinates {},{} will block the exit",
        block.0, block.1
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0
";

    #[test]
    fn part_1() {
        let ram: RamRun = EXAMPLE_1.parse().unwrap();
        assert_eq!(ram.shortest_path(6, 12), 22);
    }
    #[test]
    fn part_2() {
        let ram: RamRun = EXAMPLE_1.parse().unwrap();
        assert_eq!(ram.blocking_byte(6, 12), Pos(6, 1));
    }
}
