use fxhash::{FxHashMap, FxHasher};
use std::hash::{Hash, Hasher};
use std::str::FromStr;
use util::coord::PosI;

#[derive(Debug, Copy, Clone, Hash, Eq, PartialEq)]
enum Tile {
    OpenGround,
    Tree,
    LumberYard,
}

impl Tile {
    fn from_char(c: char) -> Self {
        match c {
            '|' => Self::Tree,
            '#' => Self::LumberYard,
            _ => Self::OpenGround,
        }
    }

    fn as_char(&self) -> char {
        match self {
            Tile::OpenGround => '.',
            Tile::Tree => '|',
            Tile::LumberYard => '#',
        }
    }
}

#[derive(Debug, Clone)]
struct Forest {
    grid: Vec<Vec<Tile>>,
    minute: usize,
}

impl Forest {
    fn minutes_until_loop(&mut self, n: usize) -> usize {
        let mut score_map: FxHashMap<u64, (usize, usize)> = FxHashMap::default();

        let mut hash = FxHasher::default();
        self.grid.hash(&mut hash);
        while !score_map.contains_key(&hash.finish()) {
            score_map.insert(hash.finish(), (self.minute, self.score()));
            self.minute();

            hash = FxHasher::default();
            self.grid.hash(&mut hash);
        }
        let last: usize = self.minute;
        let first: usize = score_map.get(&hash.finish()).copied().unwrap().0;
        let target: usize = (n - first) % (last - first) + first;

        score_map
            .into_iter()
            .find_map(|(_, (m, s))| if m == target { Some(s) } else { None })
            .unwrap()
    }

    fn minutes(&mut self, n: usize) -> usize {
        for _ in 0..n {
            self.minute();
        }
        self.score()
    }

    fn minute(&mut self) {
        let mut new_grid: Vec<Vec<Tile>> = Vec::new();

        let height: isize = self.grid.len() as isize;
        let width: isize = self.grid[0].len() as isize;

        for y in 0..height {
            let mut row: Vec<Tile> = Vec::new();
            for x in 0..width {
                let pos: PosI = PosI(x, y);
                let ngbs: Vec<PosI> = pos.neighbours_diag_limit(width, height);

                let new_tile: Tile = match self.grid[y as usize][x as usize] {
                    Tile::OpenGround => {
                        let nb_trees: usize = ngbs
                            .iter()
                            .map(|&PosI(x, y)| self.grid[y as usize][x as usize])
                            .filter(|&t| t == Tile::Tree)
                            .count();
                        if nb_trees >= 3 {
                            Tile::Tree
                        } else {
                            Tile::OpenGround
                        }
                    }
                    Tile::Tree => {
                        let nb_yards: usize = ngbs
                            .iter()
                            .map(|&PosI(x, y)| self.grid[y as usize][x as usize])
                            .filter(|&t| t == Tile::LumberYard)
                            .count();
                        if nb_yards >= 3 {
                            Tile::LumberYard
                        } else {
                            Tile::Tree
                        }
                    }
                    Tile::LumberYard => {
                        let nb_yards: usize = ngbs
                            .iter()
                            .map(|&PosI(x, y)| self.grid[y as usize][x as usize])
                            .filter(|&t| t == Tile::LumberYard)
                            .count();
                        let nb_trees: usize = ngbs
                            .iter()
                            .map(|&PosI(x, y)| self.grid[y as usize][x as usize])
                            .filter(|&t| t == Tile::Tree)
                            .count();
                        if nb_yards >= 1 && nb_trees >= 1 {
                            Tile::LumberYard
                        } else {
                            Tile::OpenGround
                        }
                    }
                };
                row.push(new_tile);
            }
            new_grid.push(row);
        }

        self.grid = new_grid;
        self.minute += 1;
    }

    fn score(&self) -> usize {
        let wooded: usize = self
            .grid
            .iter()
            .map(|row| row.iter().filter(|&&t| t == Tile::Tree).count())
            .sum();
        let yards: usize = self
            .grid
            .iter()
            .map(|row| row.iter().filter(|&&t| t == Tile::LumberYard).count())
            .sum();
        wooded * yards
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for t in row {
                print!("{}", t.as_char());
            }

            println!();
        }
    }
}

impl FromStr for Forest {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect();

        Ok(Forest { grid, minute: 0 })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_18.txt").expect("Cannot open input file");
    let mut forest: Forest = s.parse().unwrap();

    println!(
        "Part1: After 10 minutes, the resource value is {}",
        forest.minutes(10)
    );
    println!(
        "Part2: After 1_000_000_000 minutes, the resource value is {}",
        forest.minutes_until_loop(1_000_000_000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = ".#.#...|#.
.....#|##|
.|..|...#.
..|#.....#
#.#|||#|#|
...#.||...
.|....|...
||...#|.#|
|.||||..|.
...#.|..|.";

    #[test]
    fn part_1() {
        let mut forest: Forest = EXAMPLE_1.parse().unwrap();

        assert_eq!(forest.minutes(10), 1147);
    }
}
