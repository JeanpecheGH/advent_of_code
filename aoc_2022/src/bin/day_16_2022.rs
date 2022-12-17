use itertools::Itertools;
use std::cmp::max;
use std::collections::{HashMap, HashSet};
use std::str::FromStr;

const START: &str = "AA";

#[derive(Debug, Clone)]
struct Valve {
    name: String,
    flow_rate: u16,
    neighbours: Vec<String>,
}

impl FromStr for Valve {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&[',', ' ', ';', '=']).collect();
        let name: String = words[1].to_string();
        let flow_rate: u16 = words[5].parse().unwrap();
        let neighbours: Vec<String> = (11..words.len())
            .step_by(2)
            .map(|i| words[i].to_string())
            .collect();
        Ok(Self {
            name,
            flow_rate,
            neighbours,
        })
    }
}

#[derive(Debug)]
struct Tunnels {
    names: Vec<String>,
    flow_rates: Vec<u16>,
    dist_matrix: Vec<Vec<u8>>,
}

impl Tunnels {
    fn new(valves: HashMap<String, Valve>) -> Self {
        let mut names: Vec<String> = valves
            .iter()
            .filter_map(|(name, valve)| {
                if valve.flow_rate > 0 {
                    Some(name.to_string())
                } else {
                    None
                }
            })
            .collect();
        names.insert(0, START.to_string());
        let flow_rates: Vec<u16> = names
            .iter()
            .map(|name| valves.get(name).unwrap().flow_rate)
            .collect();

        let mut dist_matrix: Vec<Vec<u8>> = vec![vec![0; names.len()]; names.len()];

        for i in 0..names.len() {
            for j in i + 1..names.len() {
                let d: u8 = Self::dist(&valves, &names[i], &names[j]);
                dist_matrix[i][j] = d;
                dist_matrix[j][i] = d;
            }
        }

        Self {
            names,
            flow_rates,
            dist_matrix,
        }
    }

    fn max_pressure(&self, max_duration: u8) -> u16 {
        let remaining_valves: Vec<u8> = (1..self.names.len() as u8).collect();

        let start: Path = Path {
            pos: 0,
            duration: 0,
            released_pressure: 0,
            remaining_valves,
        };
        let mut current: Vec<Path> = vec![start];

        let mut max_released: u16 = 0;
        while !current.is_empty() {
            current = current
                .iter()
                .flat_map(|path| {
                    let ngbs: Vec<Path> = path
                        .remaining_valves
                        .iter()
                        .map(|&ngb| (ngb, self.dist_matrix[path.pos as usize][ngb as usize]))
                        .filter(|(_, d)| path.duration + *d + 1 < max_duration)
                        .map(|(ngb, d)| {
                            let new_duration = path.duration + d + 1;
                            let flow_rate: u16 = self.flow_rates[ngb as usize];
                            let released_pressure: u16 = path.released_pressure
                                + (max_duration - new_duration) as u16 * flow_rate;
                            let remaining_valves: Vec<u8> = path
                                .remaining_valves
                                .iter()
                                .filter(|&&n| ngb != n)
                                .cloned()
                                .collect();
                            Path {
                                pos: ngb,
                                duration: new_duration,
                                released_pressure,
                                remaining_valves,
                            }
                        })
                        .collect();
                    ngbs
                })
                .collect();
            max_released = max(
                max_released,
                current
                    .iter()
                    .map(|path| path.released_pressure)
                    .max()
                    .unwrap_or(0),
            );
        }
        max_released
    }
    fn max_pressure_duo(&self, max_duration: u8) -> u16 {
        let remaining_valves: Vec<u8> = (1..self.names.len() as u8).collect();

        let start: DuoPath = DuoPath {
            pos: 0,
            pos_eleph: 0,
            duration: 0,
            duration_eleph: 0,
            released_pressure: 0,
            remaining_valves,
        };
        let mut current: Vec<DuoPath> = vec![start];

        let mut max_released: u16 = 0;
        while !current.is_empty() {
            current = current
                .iter()
                .flat_map(|path| {
                    let ngbs: Vec<DuoPath> = path
                        .remaining_valves
                        .iter()
                        .permutations(2)
                        .map(|ngb_pair| {
                            let (ngb_1, ngb_2) = (*ngb_pair[0], *ngb_pair[1]);
                            (
                                (ngb_1, self.dist_matrix[path.pos as usize][ngb_1 as usize]),
                                (
                                    ngb_2,
                                    self.dist_matrix[path.pos_eleph as usize][ngb_2 as usize],
                                ),
                            )
                        })
                        .filter(|((_, d_1), (_, d_2))| {
                            path.duration + *d_1 + 1 < max_duration
                                || path.duration_eleph + *d_2 + 1 < max_duration
                        })
                        .map(|((ngb_1, d_1), (ngb_2, d_2))| {
                            let (new_pos_1, new_dura_1, pressure_1) =
                                if path.duration + d_1 + 1 < max_duration {
                                    let new_duration = path.duration + d_1 + 1;
                                    let flow_rate: u16 = self.flow_rates[ngb_1 as usize];
                                    let released_pressure: u16 =
                                        (max_duration - new_duration) as u16 * flow_rate;
                                    (ngb_1, new_duration, released_pressure)
                                } else {
                                    (path.pos, path.duration, 0)
                                };
                            let (new_pos_2, new_dura_2, pressure_2) =
                                if path.duration_eleph + d_2 + 1 < max_duration {
                                    let new_duration = path.duration_eleph + d_2 + 1;
                                    let flow_rate: u16 = self.flow_rates[ngb_2 as usize];
                                    let released_pressure: u16 =
                                        (max_duration - new_duration) as u16 * flow_rate;
                                    (ngb_2, new_duration, released_pressure)
                                } else {
                                    (path.pos_eleph, path.duration_eleph, 0)
                                };
                            let released_pressure =
                                path.released_pressure + pressure_1 + pressure_2;

                            let remaining_valves: Vec<u8> = path
                                .remaining_valves
                                .iter()
                                .filter(|n| **n != new_pos_1 && **n != new_pos_2)
                                .cloned()
                                .collect();
                            DuoPath {
                                pos: new_pos_1,
                                pos_eleph: new_pos_2,
                                duration: new_dura_1,
                                duration_eleph: new_dura_2,
                                released_pressure,
                                remaining_valves,
                            }
                        })
                        //We do not want solutions that are not improving the result we already have
                        .filter(|path| path.released_pressure > max_released)
                        .collect();
                    ngbs
                })
                .collect();
            max_released = max(
                max_released,
                current
                    .iter()
                    .map(|path| path.released_pressure)
                    .max()
                    .unwrap_or(0),
            );
        }
        max_released
    }

    fn dist(valves: &HashMap<String, Valve>, start_valve: &str, end_valve: &str) -> u8 {
        let start: String = valves.get(start_valve).map(|v| v.name.to_string()).unwrap();
        let mut visited: HashSet<String> = HashSet::new();
        visited.insert(start.clone());
        let mut current: Vec<String> = vec![start];
        let mut dist: u8 = 0;
        loop {
            current = current
                .iter()
                .flat_map(|v| valves.get(v).unwrap().neighbours.clone())
                .filter(|ngb| visited.insert(ngb.clone()))
                .collect();
            dist += 1;
            if current.contains(&end_valve.to_string()) {
                return dist;
            }
        }
    }
}

impl FromStr for Tunnels {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let valves: HashMap<String, Valve> = s
            .lines()
            .map(|l| {
                let valve: Valve = l.parse().unwrap();
                (valve.name.clone(), valve)
            })
            .collect();
        Ok(Self::new(valves))
    }
}

struct Path {
    pos: u8,
    duration: u8,
    released_pressure: u16,
    remaining_valves: Vec<u8>,
}

struct DuoPath {
    pos: u8,
    pos_eleph: u8,
    duration: u8,
    duration_eleph: u8,
    released_pressure: u16,
    remaining_valves: Vec<u8>,
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_16.txt").expect("Cannot open input file");
    let tunnels: Tunnels = s.parse().unwrap();

    println!(
        "Part1: During 30 minutes, we can at most release {} pressure",
        tunnels.max_pressure(30)
    );
    println!(
        "Part2: During 26 minutes, we can at most release {} pressure with the help of a smart Elephant",
        tunnels.max_pressure_duo(26)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II";

    #[test]
    fn part_1() {
        let tunnels: Tunnels = INPUT.parse().unwrap();

        assert_eq!(tunnels.max_pressure(30), 1651);
    }

    #[test]
    fn part_2() {
        let tunnels: Tunnels = INPUT.parse().unwrap();

        assert_eq!(tunnels.max_pressure_duo(26), 1707);
    }
}
