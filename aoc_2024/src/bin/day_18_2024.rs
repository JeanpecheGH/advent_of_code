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

#[derive(Clone, Debug)]
struct ByteBlock {
    bytes: FxHashSet<Pos>,
    top_right: bool,
    bottom_left: bool,
}

impl ByteBlock {
    fn contains(&self, pos: &Pos) -> bool {
        self.bytes.contains(pos)
    }
    fn merge(&mut self, other: ByteBlock) {
        self.bytes.extend(other.bytes);
        self.top_right = self.top_right || other.top_right;
        self.bottom_left = self.bottom_left || other.bottom_left;
    }

    fn is_wall(&self) -> bool {
        self.top_right && self.bottom_left
    }
}

struct BlockCache {
    blocks: Vec<ByteBlock>,
    size: usize,
}

impl BlockCache {
    fn from_size(size: usize) -> BlockCache {
        BlockCache {
            blocks: Vec::new(),
            size,
        }
    }
    fn is_blocking(&self) -> bool {
        self.blocks.iter().any(|b| b.is_wall())
    }

    fn add_byte(&mut self, pos: Pos) -> bool {
        //Create a new block with the falling byte
        let mut bytes: FxHashSet<Pos> = FxHashSet::default();
        bytes.insert(pos);
        let top_right: bool = pos.0 == self.size || pos.1 == 0;
        let bottom_left: bool = pos.0 == 0 || pos.1 == self.size;
        let mut new_block = ByteBlock {
            bytes,
            top_right,
            bottom_left,
        };
        //Merge the blocks touching this new block with it
        //Keep the others in the vec
        let ngbs: Vec<Pos> = pos.neighbours_diag_safe(self.size, self.size);
        self.blocks.retain(|b| {
            let is_near: bool = ngbs.iter().any(|n| b.contains(n));
            if is_near {
                new_block.merge(b.clone());
            }
            !is_near
        });
        //Add the newly created block
        self.blocks.push(new_block);
        self.is_blocking()
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

    fn blocking_byte(&self, size: usize) -> Pos {
        let mut cache: BlockCache = BlockCache::from_size(size);
        self.falling_bytes
            .iter()
            .find(|&&pos| cache.add_byte(pos))
            .copied()
            .unwrap()
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
    let block: Pos = ram.blocking_byte(70);
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
        assert_eq!(ram.blocking_byte(6), Pos(6, 1));
    }
}
