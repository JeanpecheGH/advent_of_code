use fxhash::FxHashMap;
use std::{collections::hash_map::Entry, str::FromStr};
use util::coord::Pos;

struct RaceTrack {
    grid: Vec<Vec<bool>>,
    start: Pos,
    end: Pos,
}
impl RaceTrack {
    fn race(&self) -> FxHashMap<Pos, usize> {
        let mut distances: FxHashMap<Pos, usize> = FxHashMap::default();
        distances.insert(self.start, 0);
        let mut to_visit: Vec<Pos> = vec![self.start];

        while let Some(p) = to_visit.pop() {
            if p == self.end {
                break;
            }
            let d = distances[&p];
            p.neighbours()
                .iter()
                .filter(|&&Pos(x, y)| self.grid[y][x])
                .for_each(|&ngb| {
                    if let Entry::Vacant(e) = distances.entry(ngb) {
                        e.insert(d + 1);
                        to_visit.push(ngb);
                    }
                });
        }
        distances
    }

    fn horizontal_cheats(
        &self,
        distances: &FxHashMap<Pos, usize>,
        min_cheat: usize,
    ) -> Vec<(Pos, usize)> {
        let mut cheats: Vec<(Pos, usize)> = Vec::new();
        let width: usize = self.grid[0].len();
        for y in 1..self.grid.len() - 1 {
            for x in 2..width - 2 {
                if !self.grid[y][x] && self.grid[y][x - 1] && self.grid[y][x + 1] {
                    let d_left: usize = distances[&Pos(x - 1, y)];
                    let d_right: usize = distances[&Pos(x + 1, y)];
                    let diff: usize = d_left.abs_diff(d_right);
                    if diff >= 2 + min_cheat {
                        cheats.push((Pos(x, y), diff - 2));
                    }
                }
            }
        }
        cheats
    }

    fn vertical_cheats(
        &self,
        distances: &FxHashMap<Pos, usize>,
        min_cheat: usize,
    ) -> Vec<(Pos, usize)> {
        let mut cheats: Vec<(Pos, usize)> = Vec::new();
        let width: usize = self.grid[0].len();
        for y in 2..self.grid.len() - 2 {
            for x in 1..width - 1 {
                if !self.grid[y][x] && self.grid[y - 1][x] && self.grid[y + 1][x] {
                    let d_up: usize = distances[&Pos(x, y - 1)];
                    let d_down: usize = distances[&Pos(x, y + 1)];
                    let diff: usize = d_up.abs_diff(d_down);
                    if diff >= 2 + min_cheat {
                        cheats.push((Pos(x, y), diff - 2));
                    }
                }
            }
        }
        cheats
    }

    fn global_cheats(
        &self,
        distances: &FxHashMap<Pos, usize>,
        min_cheat: usize,
    ) -> Vec<(Pos, Pos, usize)> {
        let mut cheats: Vec<(Pos, Pos, usize)> = Vec::new();
        let height: usize = self.grid.len();
        let width: usize = self.grid[0].len();
        for y in 1..height - 1 {
            for x in 1..width - 1 {
                if self.grid[y][x] {
                    for j in y..height - 1 {
                        let min_x: usize = if j == y { x + 1 } else { 1 };
                        for i in min_x..width - 1 {
                            if self.grid[j][i] {
                                let dist: usize = Pos(x, y).distance(Pos(i, j));
                                if dist <= 20 {
                                    let d1: usize = distances[&Pos(x, y)];
                                    let d2: usize = distances[&Pos(i, j)];
                                    let diff: usize = d1.abs_diff(d2);
                                    if diff >= dist + min_cheat {
                                        cheats.push((Pos(x, y), Pos(i, j), diff - dist));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
        cheats
    }

    fn nb_cheats(&self, min_cheat: usize) -> (usize, usize) {
        let distances: FxHashMap<Pos, usize> = self.race();
        let horizontal_cheats: Vec<(Pos, usize)> = self.horizontal_cheats(&distances, min_cheat);
        let vertical_cheats: Vec<(Pos, usize)> = self.vertical_cheats(&distances, min_cheat);
        let short: usize = horizontal_cheats.len() + vertical_cheats.len();

        let global_cheats: Vec<(Pos, Pos, usize)> = self.global_cheats(&distances, min_cheat);
        (short, global_cheats.len())
    }
}

impl FromStr for RaceTrack {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Pos = Pos(0, 0);
        let mut end: Pos = Pos(0, 0);
        let grid: Vec<Vec<bool>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| match c {
                        '#' => false,
                        'S' => {
                            start = Pos(x, y);
                            true
                        }
                        'E' => {
                            end = Pos(x, y);
                            true
                        }
                        _ => true,
                    })
                    .collect()
            })
            .collect();

        Ok(RaceTrack { grid, start, end })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_20.txt").expect("Cannot open input file");
    let track: RaceTrack = s.parse().unwrap();
    let (short, long) = track.nb_cheats(100);
    println!(
        "Part1: There are {short} cheats during 2 picoseconds and saving at least 100 picoseconds"
    );
    println!("Part2: If the cheats can last up to 20 picosondes, {long} are saving at least 100 picoseconds");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

    #[test]
    fn part_1() {
        let track: RaceTrack = EXAMPLE_1.parse().unwrap();
        assert_eq!(track.nb_cheats(20).0, 5);
        assert_eq!(track.nb_cheats(4).0, 30);
        assert_eq!(track.nb_cheats(2).0, 44);
    }

    #[test]
    fn part_2() {
        let track: RaceTrack = EXAMPLE_1.parse().unwrap();
        assert_eq!(track.nb_cheats(72).1, 29);
        assert_eq!(track.nb_cheats(74).1, 7);
        assert_eq!(track.nb_cheats(76).1, 3);
    }
}
