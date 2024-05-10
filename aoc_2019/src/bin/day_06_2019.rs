use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;

struct OrbitMap {
    orbits: VecDeque<(String, String)>,
    depth_map: HashMap<String, usize>,
}

impl OrbitMap {
    fn orbital_transfers(&mut self) -> usize {
        let mut santa_orbits: HashSet<String> = HashSet::new();
        let mut santa_current: String = "SAN".to_string();
        let mut you_orbits: HashSet<String> = HashSet::new();
        let mut you_current: String = "YOU".to_string();

        while santa_current != you_current {
            let (centre, sat) = self.orbits.pop_front().unwrap();
            if sat == santa_current {
                santa_orbits.insert(centre.clone());
                santa_current.clone_from(&centre);
            }
            if sat == you_current {
                you_orbits.insert(centre.clone());
                you_current.clone_from(&centre);
            }
            self.orbits.push_back((centre, sat));
        }
        let diff: HashSet<&String> = santa_orbits.symmetric_difference(&you_orbits).collect();

        diff.len()
    }

    fn fill_map(&mut self) {
        while !self.orbits.is_empty() {
            let (centre, sat) = self.orbits.pop_front().unwrap();
            if let Some(d) = self.depth_map.get(&centre) {
                self.depth_map.insert(sat, d + 1);
            } else {
                self.orbits.push_back((centre, sat));
            }
        }
    }

    fn nb_orbits(&self) -> usize {
        self.depth_map.values().sum()
    }
}

impl FromStr for OrbitMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let orbits: VecDeque<(String, String)> = s
            .lines()
            .map(|l| {
                let words: Vec<&str> = l.split(')').collect();
                (words[0].to_string(), words[1].to_string())
            })
            .collect();

        let mut depth_map: HashMap<String, usize> = HashMap::new();
        depth_map.insert("COM".to_string(), 0);

        Ok(Self { orbits, depth_map })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_06.txt").expect("Cannot open input file");
    let mut orbits_map: OrbitMap = s.parse().unwrap();
    let orbital_transfers = orbits_map.orbital_transfers();
    orbits_map.fill_map();
    println!(
        "Part1: There are {} direct or indirect orbits",
        orbits_map.nb_orbits()
    );
    println!("Part2; We need to make {orbital_transfers} to join santa's orbit");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L";

    const INPUT_2: &str = "COM)B
B)C
C)D
D)E
E)F
B)G
G)H
D)I
E)J
J)K
K)L
K)YOU
I)SAN";

    #[test]
    fn test_part_1() {
        let mut orbits_map: OrbitMap = INPUT.parse().unwrap();
        orbits_map.fill_map();
        assert_eq!(orbits_map.nb_orbits(), 42);
    }

    #[test]
    fn test_part_2() {
        let mut orbits_map: OrbitMap = INPUT_2.parse().unwrap();
        assert_eq!(orbits_map.orbital_transfers(), 4);
    }
}
