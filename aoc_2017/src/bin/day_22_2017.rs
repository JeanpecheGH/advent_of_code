use fxhash::{FxHashMap, FxHashSet};
use std::str::FromStr;
use util::coord::PosI;
use util::orientation::Dir;

enum NodeState {
    Weakened,
    Infected,
    Flagged,
    Cleaned,
}

#[derive(Debug, Clone)]
struct Virus {
    infected_nodes: FxHashSet<PosI>,
}

impl Virus {
    fn one_burst(pos: &mut PosI, dir: &mut Dir, nodes: &mut FxHashSet<PosI>) -> bool {
        let infected = if nodes.remove(pos) {
            *dir = dir.turn_right();
            false
        } else {
            nodes.insert(*pos);
            *dir = dir.turn_left();
            true
        };

        match *dir {
            Dir::North => pos.1 -= 1,
            Dir::East => pos.0 += 1,
            Dir::South => pos.1 += 1,
            Dir::West => pos.0 -= 1,
        }

        infected
    }
    fn bursts(&self, times: usize) -> usize {
        let mut nodes: FxHashSet<PosI> = self.infected_nodes.clone();
        let mut pos: PosI = PosI(0, 0);
        let mut dir: Dir = Dir::North;

        let mut nb_infected: usize = 0;

        for _ in 0..times {
            if Virus::one_burst(&mut pos, &mut dir, &mut nodes) {
                nb_infected += 1;
            }
        }
        nb_infected
    }
    fn evolved_one_burst(
        pos: &mut PosI,
        dir: &mut Dir,
        nodes: &mut FxHashMap<PosI, NodeState>,
    ) -> bool {
        let state: &mut NodeState = nodes.entry(*pos).or_insert(NodeState::Cleaned);

        let infected: bool = match *state {
            NodeState::Weakened => {
                *state = NodeState::Infected;
                true
            }
            NodeState::Infected => {
                *state = NodeState::Flagged;
                *dir = dir.turn_right();
                false
            }
            NodeState::Flagged => {
                nodes.remove(pos);
                *dir = dir.half_turn();
                false
            }
            NodeState::Cleaned => {
                *state = NodeState::Weakened;
                *dir = dir.turn_left();
                false
            }
        };

        match *dir {
            Dir::North => pos.1 -= 1,
            Dir::East => pos.0 += 1,
            Dir::South => pos.1 += 1,
            Dir::West => pos.0 -= 1,
        }

        infected
    }

    fn evolved_bursts(&self, times: usize) -> usize {
        let mut nodes: FxHashMap<PosI, NodeState> = self
            .infected_nodes
            .iter()
            .map(|&n| (n, NodeState::Infected))
            .collect();
        let mut pos: PosI = PosI(0, 0);
        let mut dir: Dir = Dir::North;

        let mut nb_infected: usize = 0;

        for _ in 0..times {
            if Virus::evolved_one_burst(&mut pos, &mut dir, &mut nodes) {
                nb_infected += 1;
            }
        }
        nb_infected
    }
}

impl FromStr for Virus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        //Size is odd
        let offset = lines.len() as isize / 2;

        let infected_nodes: FxHashSet<PosI> = lines
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(x, c)| {
                        if c == '#' {
                            Some(PosI(x as isize - offset, y as isize - offset))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<PosI>>()
            })
            .collect();

        Ok(Virus { infected_nodes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_22.txt").expect("Cannot open input file");
    let virus: Virus = s.parse().unwrap();

    println!(
        "Part1: After 10_000 bursts, the virus caused {} nodes to become infected",
        virus.bursts(10_000)
    );
    println!(
        "Part2: After evolving, 10_000_000 bursts caused {} nodes to become infected",
        virus.evolved_bursts(10_000_000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "..#
#..
...
";
    #[test]
    fn part_1_test_1() {
        let virus: Virus = EXAMPLE_1.parse().unwrap();
        assert_eq!(5, virus.bursts(7));
    }
    #[test]
    fn part_1_test_2() {
        let virus: Virus = EXAMPLE_1.parse().unwrap();
        assert_eq!(41, virus.bursts(70));
    }
    #[test]
    fn part_1_test_3() {
        let virus: Virus = EXAMPLE_1.parse().unwrap();
        assert_eq!(5587, virus.bursts(10_000));
    }
    #[test]
    fn part_2_test_1() {
        let virus: Virus = EXAMPLE_1.parse().unwrap();
        assert_eq!(26, virus.evolved_bursts(100));
    }
    #[test]
    fn part_2_test_2() {
        let virus: Virus = EXAMPLE_1.parse().unwrap();
        //Should be 2511944 for 10_000_000 bursts, but it's a bit slow for a test
        assert_eq!(2608, virus.evolved_bursts(10_000));
    }
}
