use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use util::coord::Pos;

const SIZE: usize = 5;
#[derive(Clone)]
struct Bugs {
    grid: Vec<Vec<bool>>,
}

impl Bugs {
    fn empty() -> Bugs {
        let row: Vec<bool> = vec![false; SIZE];
        let grid: Vec<Vec<bool>> = (0..SIZE).map(|_| row.clone()).collect();
        Bugs { grid }
    }
    fn next(&self) -> Bugs {
        let mut new_grid: Vec<Vec<bool>> = self.grid.clone();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                let p: Pos = Pos(x, y);
                let nb_bugs: usize = p
                    .neighbours_safe(SIZE, SIZE)
                    .into_iter()
                    .filter(|&Pos(i, j)| self.grid[j][i])
                    .count();

                new_grid[y][x] = matches!((b, nb_bugs), (_, 1) | (false, 2));
            }
        }
        Bugs { grid: new_grid }
    }

    fn score(&self) -> usize {
        let mut pow: usize = 1;
        let mut score: usize = 0;

        for row in self.grid.iter() {
            for bug in row.iter() {
                if *bug {
                    score += pow;
                }
                pow *= 2;
            }
        }
        score
    }

    fn count(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn bug_at(&self, Pos(x, y): Pos) -> bool {
        if x > SIZE || y > SIZE {
            false
        } else {
            self.grid[y][x]
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for y in 0..SIZE {
            for x in 0..SIZE {
                let c: char = if self.grid[y][x] { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Bugs {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<bool>> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        Ok(Bugs { grid })
    }
}

struct Eris {
    bugs: Bugs,
    scores: HashSet<usize>,
}

impl Eris {
    fn first_double_state(&mut self) -> usize {
        loop {
            self.bugs = self.bugs.next();
            let new_score: usize = self.bugs.score();
            if !self.scores.insert(new_score) {
                return new_score;
            }
        }
    }
}

impl FromStr for Eris {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bugs: Bugs = s.parse().unwrap();
        let scores: HashSet<usize> = HashSet::from([bugs.score()]);
        Ok(Eris { bugs, scores })
    }
}

#[derive(Copy, Clone)]
struct LevelPos {
    level: usize,
    pos: Pos,
}

impl LevelPos {
    fn new(level: usize, pos: Pos) -> LevelPos {
        LevelPos { level, pos }
    }
}

struct RecursiveEris {
    levels: VecDeque<Bugs>,
}

impl RecursiveEris {
    fn nb_bugs(&self) -> usize {
        self.levels.iter().map(|b| b.count()).sum()
    }

    fn after_time(&mut self, time: usize) -> usize {
        for _ in 0..time {
            self.one_minute();
        }

        self.nb_bugs()
    }

    fn one_minute(&mut self) {
        //First, if we have any bug in the first or last layer, we add a layer in that direction
        if let Some(b) = self.levels.front() {
            if b.count() > 0 {
                self.levels.push_front(Bugs::empty());
            }
        }
        if let Some(b) = self.levels.back() {
            if b.count() > 0 {
                self.levels.push_back(Bugs::empty());
            }
        }

        //Clone the levels and compute each level based on the previous ones
        let mut new_levels: VecDeque<Bugs> = self.levels.clone();
        for (n, level) in self.levels.iter().enumerate() {
            for (y, row) in level.grid.iter().enumerate() {
                for (x, &b) in row.iter().enumerate() {
                    //We never modify the middle tile
                    if x == 2 && y == 2 {
                        continue;
                    }
                    let p: LevelPos = LevelPos {
                        level: n,
                        pos: Pos(x, y),
                    };
                    let ngbs: Vec<LevelPos> = self.neighbours(p);
                    let nb_bugs: usize = ngbs.into_iter().filter(|&p| self.bug_at(p)).count();
                    new_levels[n].grid[y][x] = matches!((b, nb_bugs), (_, 1) | (false, 2));
                }
            }
        }
        self.levels = new_levels;
    }

    fn neighbours(&self, p: LevelPos) -> Vec<LevelPos> {
        match (p.level, p.pos.0, p.pos.1) {
            //Simple case, natural neighbours
            (l, 1 | 3, 1 | 3) => p
                .pos
                .neighbours()
                .into_iter()
                .map(|pos| LevelPos::new(l, pos))
                .collect(),
            //4 inner cases at last level
            (l, 2, 1) if l == self.levels.len() - 1 => {
                vec![
                    LevelPos::new(l, Pos(2, 0)),
                    LevelPos::new(l, Pos(1, 1)),
                    LevelPos::new(l, Pos(3, 1)),
                ]
            }
            (l, 2, 3) if l == self.levels.len() - 1 => {
                vec![
                    LevelPos::new(l, Pos(2, 4)),
                    LevelPos::new(l, Pos(1, 3)),
                    LevelPos::new(l, Pos(3, 3)),
                ]
            }
            (l, 1, 2) if l == self.levels.len() - 1 => {
                vec![
                    LevelPos::new(l, Pos(0, 2)),
                    LevelPos::new(l, Pos(1, 1)),
                    LevelPos::new(l, Pos(1, 3)),
                ]
            }
            (l, 3, 2) if l == self.levels.len() - 1 => {
                vec![
                    LevelPos::new(l, Pos(4, 2)),
                    LevelPos::new(l, Pos(3, 1)),
                    LevelPos::new(l, Pos(3, 3)),
                ]
            }
            //4 inner cases at any other level
            (l, 2, 1) => {
                vec![
                    //3 real neighbours
                    LevelPos::new(l, Pos(2, 0)),
                    LevelPos::new(l, Pos(1, 1)),
                    LevelPos::new(l, Pos(3, 1)),
                    //5 "inside" neighbours
                    LevelPos::new(l + 1, Pos(0, 0)),
                    LevelPos::new(l + 1, Pos(1, 0)),
                    LevelPos::new(l + 1, Pos(2, 0)),
                    LevelPos::new(l + 1, Pos(3, 0)),
                    LevelPos::new(l + 1, Pos(4, 0)),
                ]
            }
            (l, 2, 3) => {
                vec![
                    //3 real neighbours
                    LevelPos::new(l, Pos(2, 4)),
                    LevelPos::new(l, Pos(1, 3)),
                    LevelPos::new(l, Pos(3, 3)),
                    //5 "inside" neighbours
                    LevelPos::new(l + 1, Pos(0, 4)),
                    LevelPos::new(l + 1, Pos(1, 4)),
                    LevelPos::new(l + 1, Pos(2, 4)),
                    LevelPos::new(l + 1, Pos(3, 4)),
                    LevelPos::new(l + 1, Pos(4, 4)),
                ]
            }
            (l, 1, 2) => {
                vec![
                    //3 real neighbours
                    LevelPos::new(l, Pos(0, 2)),
                    LevelPos::new(l, Pos(1, 1)),
                    LevelPos::new(l, Pos(1, 3)),
                    //5 "inside" neighbours
                    LevelPos::new(l + 1, Pos(0, 0)),
                    LevelPos::new(l + 1, Pos(0, 1)),
                    LevelPos::new(l + 1, Pos(0, 2)),
                    LevelPos::new(l + 1, Pos(0, 3)),
                    LevelPos::new(l + 1, Pos(0, 4)),
                ]
            }
            (l, 3, 2) => {
                vec![
                    //3 real neighbours
                    LevelPos::new(l, Pos(4, 2)),
                    LevelPos::new(l, Pos(3, 1)),
                    LevelPos::new(l, Pos(3, 3)),
                    //5 "inside" neighbours
                    LevelPos::new(l + 1, Pos(4, 0)),
                    LevelPos::new(l + 1, Pos(4, 1)),
                    LevelPos::new(l + 1, Pos(4, 2)),
                    LevelPos::new(l + 1, Pos(4, 3)),
                    LevelPos::new(l + 1, Pos(4, 4)),
                ]
            }
            //4 outer sides except corners at level 0
            (0, x @ 1..=3, 0) => {
                vec![
                    LevelPos::new(0, Pos(x - 1, 0)),
                    LevelPos::new(0, Pos(x + 1, 0)),
                    LevelPos::new(0, Pos(x, 1)),
                ]
            }
            (0, x @ 1..=3, 4) => {
                vec![
                    LevelPos::new(0, Pos(x - 1, 4)),
                    LevelPos::new(0, Pos(x + 1, 4)),
                    LevelPos::new(0, Pos(x, 3)),
                ]
            }
            (0, 0, y @ 1..=3) => {
                vec![
                    LevelPos::new(0, Pos(0, y - 1)),
                    LevelPos::new(0, Pos(0, y + 1)),
                    LevelPos::new(0, Pos(1, y)),
                ]
            }
            (0, 4, y @ 1..=3) => {
                vec![
                    LevelPos::new(0, Pos(4, y - 1)),
                    LevelPos::new(0, Pos(4, y + 1)),
                    LevelPos::new(0, Pos(3, y)),
                ]
            }
            //4 outer sides except corners at any other level
            (l, x @ 1..=3, 0) => {
                vec![
                    LevelPos::new(l, Pos(x - 1, 0)),
                    LevelPos::new(l, Pos(x + 1, 0)),
                    LevelPos::new(l, Pos(x, 1)),
                    LevelPos::new(l - 1, Pos(2, 1)),
                ]
            }
            (l, x @ 1..=3, 4) => {
                vec![
                    LevelPos::new(l, Pos(x - 1, 4)),
                    LevelPos::new(l, Pos(x + 1, 4)),
                    LevelPos::new(l, Pos(x, 3)),
                    LevelPos::new(l - 1, Pos(2, 3)),
                ]
            }
            (l, 0, y @ 1..=3) => {
                vec![
                    LevelPos::new(l, Pos(0, y - 1)),
                    LevelPos::new(l, Pos(0, y + 1)),
                    LevelPos::new(l, Pos(1, y)),
                    LevelPos::new(l - 1, Pos(1, 2)),
                ]
            }
            (l, 4, y @ 1..=3) => {
                vec![
                    LevelPos::new(l, Pos(4, y - 1)),
                    LevelPos::new(l, Pos(4, y + 1)),
                    LevelPos::new(l, Pos(3, y)),
                    LevelPos::new(l - 1, Pos(3, 2)),
                ]
            }
            //4 corners at level 0
            (0, 0, 0) => {
                vec![LevelPos::new(0, Pos(0, 1)), LevelPos::new(0, Pos(1, 0))]
            }
            (0, 4, 0) => {
                vec![LevelPos::new(0, Pos(4, 1)), LevelPos::new(0, Pos(3, 0))]
            }
            (0, 0, 4) => {
                vec![LevelPos::new(0, Pos(0, 3)), LevelPos::new(0, Pos(1, 4))]
            }
            (0, 4, 4) => {
                vec![LevelPos::new(0, Pos(3, 4)), LevelPos::new(0, Pos(4, 3))]
            }
            //4 corners at any other level
            (l, 0, 0) => {
                vec![
                    LevelPos::new(l, Pos(0, 1)),
                    LevelPos::new(l, Pos(1, 0)),
                    LevelPos::new(l - 1, Pos(2, 1)),
                    LevelPos::new(l - 1, Pos(1, 2)),
                ]
            }
            (l, 4, 0) => {
                vec![
                    LevelPos::new(l, Pos(4, 1)),
                    LevelPos::new(l, Pos(3, 0)),
                    LevelPos::new(l - 1, Pos(2, 1)),
                    LevelPos::new(l - 1, Pos(3, 2)),
                ]
            }
            (l, 0, 4) => {
                vec![
                    LevelPos::new(l, Pos(0, 3)),
                    LevelPos::new(l, Pos(1, 4)),
                    LevelPos::new(l - 1, Pos(2, 3)),
                    LevelPos::new(l - 1, Pos(1, 2)),
                ]
            }
            (l, 4, 4) => {
                vec![
                    LevelPos::new(l, Pos(3, 4)),
                    LevelPos::new(l, Pos(4, 3)),
                    LevelPos::new(l - 1, Pos(2, 3)),
                    LevelPos::new(l - 1, Pos(3, 2)),
                ]
            }
            _ => Vec::new(),
        }
    }

    fn bug_at(&self, p: LevelPos) -> bool {
        if p.level >= self.levels.len() {
            false
        } else {
            self.levels[p.level].bug_at(p.pos)
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for (n, l) in self.levels.iter().enumerate() {
            println!("Level {n}");
            l.print();
        }
    }
}
impl FromStr for RecursiveEris {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bugs: Bugs = s.parse().unwrap();
        let levels: VecDeque<Bugs> = VecDeque::from([bugs]);
        Ok(RecursiveEris { levels })
    }
}
fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_24.txt").expect("Cannot open input file");
    let mut eris: Eris = s.parse().unwrap();

    println!(
        "Part1: The first biodiversity rating appearing twice is {}",
        eris.first_double_state()
    );

    //Part 2
    let mut rec_eris: RecursiveEris = s.parse().unwrap();
    println!(
        "Part2: After 200 minutes, there are {} bugs on Recursive Eris",
        rec_eris.after_time(200)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "....#
#..#.
#..##
..#..
#....";

    #[test]
    fn test_1() {
        let mut eris: Eris = EXAMPLE_1.parse().unwrap();
        let d: usize = eris.first_double_state();
        assert_eq!(d, 2129920);
    }

    #[test]
    fn test_2() {
        let mut eris: RecursiveEris = EXAMPLE_1.parse().unwrap();
        let nb: usize = eris.after_time(10);
        assert_eq!(nb, 99);
    }
}
