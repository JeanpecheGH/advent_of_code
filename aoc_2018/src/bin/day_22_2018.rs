use fxhash::FxHashMap;
use nom::character::complete::{char, line_ending};
use nom::sequence::{preceded, separated_pair, terminated};
use nom::IResult;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::basic_parser::{parse_usize, title};
use util::coord::Pos;

const MARGIN: usize = 7;

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
enum Equipment {
    Torch,
    ClimbingGear,
    Nothing,
}

impl Equipment {
    fn valid_for(&self, risk_level: usize) -> bool {
        matches!(
            (self, risk_level),
            (Equipment::Torch, 0 | 2)
                | (Equipment::ClimbingGear, 0 | 1)
                | (Equipment::Nothing, 1 | 2)
        )
    }

    fn switch_for(&self, risk_level: usize) -> Equipment {
        match (self, risk_level) {
            (Equipment::Torch, 0) => Equipment::ClimbingGear,
            (Equipment::Torch, 2) => Equipment::Nothing,
            (Equipment::ClimbingGear, 0) => Equipment::Torch,
            (Equipment::ClimbingGear, 1) => Equipment::Nothing,
            (Equipment::Nothing, 1) => Equipment::ClimbingGear,
            (Equipment::Nothing, 2) => Equipment::Torch,
            _ => *self,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash)]
struct RescueNode {
    pos: Pos,
    equipment: Equipment,
    minutes: usize,
    heuristic: usize,
}

impl RescueNode {
    fn min_time(&self) -> usize {
        self.minutes + self.heuristic
    }
}

impl Ord for RescueNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .min_time()
            .cmp(&self.min_time())
            .then(self.minutes.cmp(&other.minutes))
    }
}

impl PartialOrd for RescueNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct CaveMaze {
    target: Pos,
    grid: Vec<Vec<usize>>,
}

impl CaveMaze {
    fn risk_level(&self) -> usize {
        self.grid
            .iter()
            .take(self.target.1 + 1)
            .map(|row| {
                row.iter()
                    .take(self.target.0 + 1)
                    .map(|erosion| erosion % 3)
                    .sum::<usize>()
            })
            .sum()
    }

    fn risk_at(&self, pos: &Pos) -> usize {
        self.grid[pos.1][pos.0] % 3
    }

    fn equipment_allowed_at(&self, equipment: Equipment, pos: &Pos) -> bool {
        equipment.valid_for(self.risk_at(pos))
    }

    fn switch_equipment(&self, node: &RescueNode) -> RescueNode {
        let eq: Equipment = node.equipment.switch_for(self.risk_at(&node.pos));
        RescueNode {
            equipment: eq,
            minutes: node.minutes + 7,
            ..*node
        }
    }

    fn neighbours(&self, node: &RescueNode) -> Vec<RescueNode> {
        let mut nodes: Vec<RescueNode> = node
            .pos
            .neighbours_safe(self.target.0 + MARGIN + 1, self.target.1 + MARGIN + 1)
            .into_iter()
            .filter(|p| self.equipment_allowed_at(node.equipment, p))
            .map(|pos| RescueNode {
                pos,
                equipment: node.equipment,
                minutes: node.minutes + 1,
                heuristic: pos.distance(self.target),
            })
            .collect();

        nodes.push(self.switch_equipment(node));
        nodes
    }

    fn a_star(&self, &pos: &Pos) -> usize {
        let start: RescueNode = RescueNode {
            pos,
            equipment: Equipment::Torch,
            minutes: 0,
            heuristic: pos.distance(self.target),
        };

        let mut heap: BinaryHeap<RescueNode> = BinaryHeap::new();
        heap.push(start);

        let mut visited_pos: FxHashMap<(Pos, Equipment), usize> = FxHashMap::default();
        visited_pos.insert((pos, Equipment::Torch), 0);

        loop {
            if let Some(best_node) = heap.pop() {
                if let Some(&m) = visited_pos.get(&(best_node.pos, best_node.equipment)) {
                    if m == best_node.minutes {
                        if best_node.pos == self.target && best_node.equipment == Equipment::Torch {
                            return best_node.minutes;
                        }
                        let new_nodes: Vec<RescueNode> = self.neighbours(&best_node);

                        new_nodes
                            .into_iter()
                            //Filter already visited positions
                            .filter(|node| {
                                let key: (Pos, Equipment) = (node.pos, node.equipment);
                                let e = visited_pos.entry(key).or_insert(usize::MAX);
                                if node.minutes < *e {
                                    *e = node.minutes;
                                    true
                                } else {
                                    false
                                }
                            })
                            .for_each(|node| {
                                heap.push(node);
                            })
                    }
                }
            } else {
                //The node cannot reach the end
                return usize::MAX;
            }
        }
    }

    fn rescue_time(&self) -> usize {
        self.a_star(&Pos(0, 0))
    }
}

impl FromStr for CaveMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_depth_and_target(s: &str) -> IResult<&str, (usize, Pos)> {
            let (s, depth) = preceded(title, terminated(parse_usize, line_ending))(s)?;
            let (s, (x, y)) =
                preceded(title, separated_pair(parse_usize, char(','), parse_usize))(s)?;
            Ok((s, (depth, Pos(x, y))))
        }

        let (depth, target) = parse_depth_and_target(s).unwrap().1;
        let mut grid: Vec<Vec<usize>> = vec![vec![0; target.0 + MARGIN + 1]; target.1 + MARGIN + 1];

        for y in 0..=(target.1 + MARGIN) {
            for x in 0..=(target.0 + MARGIN) {
                let geo_index: usize = match (x, y) {
                    (0, _) => y * 48271,
                    (_, 0) => x * 16807,
                    (i, j) if i == target.0 && j == target.1 => 0,
                    _ => grid[y - 1][x] * grid[y][x - 1],
                };
                grid[y][x] = (geo_index + depth) % 20183;
            }
        }

        Ok(CaveMaze { target, grid })
    }
}

fn main() {
    let now = std::time::Instant::now();

    let s = util::file_as_string("aoc_2018/input/day_22.txt").expect("Cannot open input file");
    let maze: CaveMaze = s.parse().unwrap();

    println!(
        "Part1: The total risk level for the area is {}",
        maze.risk_level()
    );
    println!(
        "Part2: The target can be rescued in {} minutes",
        maze.rescue_time()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "depth: 510
target: 10,10";

    #[test]
    fn part_1() {
        let maze: CaveMaze = EXAMPLE_1.parse().unwrap();
        assert_eq!(114, maze.risk_level());
    }

    #[test]
    fn part_2() {
        let maze: CaveMaze = EXAMPLE_1.parse().unwrap();
        assert_eq!(45, maze.rescue_time())
    }
}
