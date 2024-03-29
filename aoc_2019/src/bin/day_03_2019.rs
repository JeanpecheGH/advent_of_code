use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use util::coord::PosI;
use util::orientation::Dir;

#[derive(Debug, Copy, Clone)]
struct Move {
    len: usize,
    dir: Dir,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c: char = s.chars().next().unwrap();
        let len: usize = s[1..].parse().unwrap();
        let dir = match c {
            'U' => Dir::North,
            'D' => Dir::South,
            'L' => Dir::West,
            'R' => Dir::East,
            _ => return Err(()),
        };

        Ok(Move { len, dir })
    }
}

struct Wires {
    wire_1: Vec<Move>,
    wire_2: Vec<Move>,
}

impl Wires {
    fn paths(&self) -> (HashMap<PosI, usize>, HashMap<PosI, usize>) {
        fn to_path(moves: &[Move]) -> HashMap<PosI, usize> {
            let mut map: HashMap<PosI, usize> = HashMap::new();
            let mut current_pos: PosI = PosI(0, 0);
            let mut dist: usize = 0;
            map.insert(current_pos, dist);
            moves.iter().for_each(|m| {
                let dir: PosI = match m.dir {
                    Dir::North => PosI(0, -1),
                    Dir::South => PosI(0, 1),
                    Dir::West => PosI(-1, 0),
                    Dir::East => PosI(1, 0),
                };
                for _ in 0..m.len {
                    current_pos = PosI(current_pos.0 + dir.0, current_pos.1 + dir.1);
                    dist += 1;
                    map.insert(current_pos, dist);
                }
            });
            map
        }

        (to_path(&self.wire_1), to_path(&self.wire_2))
    }

    fn best_crossings(&self) -> (usize, usize) {
        let (wire_1, wire_2) = self.paths();
        let wire_1_pos: HashSet<PosI> = wire_1.keys().copied().collect();
        let wire_2_pos: HashSet<PosI> = wire_2.keys().copied().collect();
        let mut crossings: HashSet<PosI> = wire_1_pos.intersection(&wire_2_pos).copied().collect();
        crossings.remove(&PosI(0, 0));
        let nearest_crossing = crossings
            .iter()
            .map(|&PosI(x, y)| x.unsigned_abs() + y.unsigned_abs())
            .min()
            .unwrap();

        let fastest_crossing: usize = crossings
            .iter()
            .map(|pos| wire_1.get(pos).unwrap() + wire_2.get(pos).unwrap())
            .min()
            .unwrap();

        (nearest_crossing, fastest_crossing)
    }
}

impl FromStr for Wires {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let wires: Vec<Vec<Move>> = s
            .lines()
            .map(|l| {
                let words: Vec<&str> = l.split(',').collect();
                words.into_iter().map(|w| w.parse().unwrap()).collect()
            })
            .collect();

        Ok(Self {
            wire_1: wires[0].clone(),
            wire_2: wires[1].clone(),
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_03.txt").expect("Cannot open input file");
    let wires: Wires = s.parse().unwrap();
    let (part_1, part_2) = wires.best_crossings();
    println!("Part1: The distance to the nearest wire crossing is {part_1}");
    println!("Part1: The distance to the fastest wire crossing is {part_2}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "R8,U5,L5,D3\nU7,R6,D4,L4";
    const INPUT_2: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83";
    const INPUT_3: &str =
        "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    #[test]
    fn test_1_part_1() {
        let wires: Wires = INPUT_1.parse().unwrap();
        assert_eq!(wires.best_crossings(), (6, 30));
    }
    #[test]
    fn test_2_part_1() {
        let wires: Wires = INPUT_2.parse().unwrap();
        assert_eq!(wires.best_crossings(), (159, 610));
    }
    #[test]
    fn test_3_part_1() {
        let wires: Wires = INPUT_3.parse().unwrap();
        assert_eq!(wires.best_crossings(), (135, 410));
    }
}
