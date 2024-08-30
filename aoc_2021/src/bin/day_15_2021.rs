use fxhash::FxHashSet;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::coord::Pos;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct ChitonNode {
    pos: Pos,
    risk_level: usize,
}

impl Ord for ChitonNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.risk_level.cmp(&self.risk_level)
    }
}

impl PartialOrd for ChitonNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct ChitonCave {
    grid: Vec<Vec<usize>>,
    width: usize,
    height: usize,
}

impl ChitonCave {
    fn risk_at(&self, Pos(x, y): Pos) -> usize {
        let d_x: usize = x / self.width;
        let rest_x: usize = x % self.width;
        let d_y: usize = y / self.height;
        let rest_y: usize = y % self.height;

        (self.grid[rest_y][rest_x] - 1 + d_x + d_y) % 9 + 1
    }

    fn risk_level(&self, size_factor: usize) -> usize {
        let max_x: usize = self.width * size_factor;
        let max_y: usize = self.height * size_factor;
        let end: Pos = Pos(max_x - 1, max_y - 1);
        let start: ChitonNode = ChitonNode {
            pos: Pos(0, 0),
            risk_level: 0,
        };

        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        visited.insert(start.pos);
        let mut priority_queue: BinaryHeap<ChitonNode> = BinaryHeap::default();
        priority_queue.push(start);

        while let Some(best_node) = priority_queue.pop() {
            if best_node.pos == end {
                return best_node.risk_level;
            }
            best_node
                .pos
                .neighbours_safe(max_x, max_y)
                .into_iter()
                .filter(|&pos| visited.insert(pos))
                .for_each(|pos| {
                    let risk_level: usize = best_node.risk_level + self.risk_at(pos);
                    priority_queue.push(ChitonNode { pos, risk_level })
                })
        }
        //Should not happen
        0
    }
}

impl FromStr for ChitonCave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let width: usize = grid[0].len();
        let height: usize = grid.len();

        Ok(ChitonCave {
            grid,
            width,
            height,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_15.txt").expect("Cannot open input file");
    let cave: ChitonCave = s.parse().unwrap();
    println!(
        "Part1: The lowest risk level to traverse the cave is {}",
        cave.risk_level(1)
    );
    println!(
        "Part2: In the actual size cave, the lowest risk level is {}",
        cave.risk_level(5)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581
";

    #[test]
    fn part_1() {
        let cave: ChitonCave = EXAMPLE_1.parse().unwrap();
        assert_eq!(40, cave.risk_level(1));
    }

    #[test]
    fn part_2() {
        let cave: ChitonCave = EXAMPLE_1.parse().unwrap();
        assert_eq!(315, cave.risk_level(5));
    }
}
