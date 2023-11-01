use itertools::Itertools;
use std::collections::hash_map::Entry::Vacant;
use std::collections::HashMap;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

const MARGIN: usize = 55;

struct Grove {
    grid: Vec<Vec<bool>>,
    order: Vec<Dir>,
}

impl Grove {
    fn move_around(&mut self, rounds: usize) {
        for _ in 0..rounds {
            self.round();
        }
    }

    fn round(&mut self) {
        let mut target_map: HashMap<Pos, Pos> = HashMap::new();
        //Find target for each elves
        for (j, row) in self.grid.iter().enumerate() {
            for (i, &b) in row.iter().enumerate() {
                if b {
                    let pos = Pos(i, j);
                    if let Some(target) = self.target(&pos) {
                        if let Vacant(e) = target_map.entry(target) {
                            e.insert(pos);
                        } else {
                            target_map.remove(&target);
                        }
                    }
                }
            }
        }
        //Apply not-conflicting targets
        target_map.into_iter().for_each(|(target, pos)| {
            self.grid[pos.1][pos.0] = false;
            self.grid[target.1][target.0] = true;
        });

        //Rotate order of directions
        self.order.rotate_left(1);
    }

    fn neighbours(&self, &Pos(x, y): &Pos) -> Vec<bool> {
        (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|(i, j)| *i != 0 || *j != 0)
            .map(|(i, j)| (((x as isize + i) as usize), (y as isize + j) as usize))
            .map(|(i, j)| self.grid[j][i])
            .collect()
    }

    fn target(&self, pos @ &Pos(x, y): &Pos) -> Option<Pos> {
        let ngbs: Vec<bool> = self.neighbours(pos);
        if ngbs.iter().all(|b| !*b) {
            return None;
        }
        /*
        Neighbours :
          035
          1.6
          247
         */
        self.order.iter().find_map(|dir| match dir {
            Dir::North => {
                if !ngbs[0] && !ngbs[3] && !ngbs[5] {
                    Some(Pos(x, y - 1))
                } else {
                    None
                }
            }
            Dir::South => {
                if !ngbs[2] && !ngbs[4] && !ngbs[7] {
                    Some(Pos(x, y + 1))
                } else {
                    None
                }
            }
            Dir::West => {
                if !ngbs[0] && !ngbs[1] && !ngbs[2] {
                    Some(Pos(x - 1, y))
                } else {
                    None
                }
            }
            Dir::East => {
                if !ngbs[5] && !ngbs[6] && !ngbs[7] {
                    Some(Pos(x + 1, y))
                } else {
                    None
                }
            }
        })
    }

    fn rectangle_limits(&self) -> (Pos, Pos) {
        let top_y: usize = self
            .grid
            .iter()
            .position(|row| row.iter().any(|b| *b))
            .unwrap();
        let bot_y: usize = self
            .grid
            .iter()
            .rposition(|row| row.iter().any(|b| *b))
            .unwrap();

        let mut top_x: usize = 0;
        'outer: loop {
            for j in top_y..=bot_y {
                if self.grid[j][top_x] {
                    break 'outer;
                }
            }
            top_x += 1;
        }

        let mut bot_x: usize = self.grid.len() - 1;
        'outer: loop {
            for j in top_y..=bot_y {
                if self.grid[j][bot_x] {
                    break 'outer;
                }
            }
            bot_x -= 1;
        }
        (Pos(top_x, top_y), Pos(bot_x, bot_y))
    }

    fn score(&self) -> usize {
        let (top_left, bot_right): (Pos, Pos) = self.rectangle_limits();
        self.grid[top_left.1..=bot_right.1]
            .iter()
            .map(|row| {
                row[top_left.0..=bot_right.0]
                    .iter()
                    .filter(|b| !**b)
                    .count()
            })
            .sum()
    }

    fn equal(&self, other: &[Vec<bool>]) -> bool {
        self.grid
            .iter()
            .enumerate()
            .all(|(j, row)| row.iter().enumerate().all(|(i, b)| other[j][i] == *b))
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Printing Grove:");
        for row in self.grid.iter() {
            for &b in row.iter() {
                let c = if b { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Grove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let size = lines.len() + 2 * MARGIN;

        let mut grid: Vec<Vec<bool>> = vec![vec![false; size]; size];
        for (j, row) in lines.into_iter().enumerate() {
            for (i, c) in row.chars().enumerate() {
                if c == '#' {
                    grid[j + MARGIN][i + MARGIN] = true;
                }
            }
        }

        let order: Vec<Dir> = vec![Dir::North, Dir::South, Dir::West, Dir::East];
        Ok(Grove { grid, order })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_23.txt").expect("Cannot open input file");
    let mut grove: Grove = s.parse().unwrap();
    grove.move_around(10);
    println!(
        "Part1: There are {} empty spaces in the rectangle occupied by the elves",
        grove.score()
    );

    let mut grid: Vec<Vec<bool>> = vec![vec![false; grove.grid.len()]; grove.grid.len()];
    let mut nb_round = 10;
    while !grove.equal(&grid) {
        grid = grove.grid.clone();
        grove.round();
        nb_round += 1;
    }
    println!("Part2: After round {nb_round}, the elves are not moving anymore");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..";

    #[test]
    fn part_1() {
        let mut grove: Grove = INPUT.parse().unwrap();
        grove.move_around(10);
        assert_eq!(grove.score(), 110);
    }

    #[test]
    fn part_2() {
        let mut grove: Grove = INPUT.parse().unwrap();
        let mut grid: Vec<Vec<bool>> = vec![vec![false; grove.grid.len()]; grove.grid.len()];
        let mut nb_round = 0;
        while !grove.equal(&grid) {
            grid = grove.grid.clone();
            grove.round();
            nb_round += 1;
        }
        assert_eq!(nb_round, 20);
    }
}
