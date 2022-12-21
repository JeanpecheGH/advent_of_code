use std::str::FromStr;

type Pos = (usize, usize);
const ROCK_SIZE: usize = 4;
const ROCK_NB: usize = 5;
const WIDTH: usize = 7;
const YEAR: usize = 2022;
const BILLION: usize = 1_000_000_000_000;

const LINE: &str = "....
....
....
####";

const CROSS: &str = "....
.#..
###.
.#..";

const ANGLE: &str = "....
..#.
..#.
###.";

const COL: &str = "#...
#...
#...
#...";

const SQUARE: &str = "....
....
##..
##..";

#[derive(Debug)]
struct Jets {
    jets: Vec<isize>,
    idx: usize,
}

impl Jets {
    fn next(&mut self) -> isize {
        let res = self.jets[self.idx];
        if self.idx + 1 < self.jets.len() {
            self.idx += 1;
        } else {
            self.idx = 0;
        }
        res
    }
}

impl FromStr for Jets {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let jets: Vec<isize> = s
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| if c == '<' { -1 } else { 1 })
            .collect();
        Ok(Self { jets, idx: 0 })
    }
}

#[derive(Debug, Clone)]
struct Rock {
    grid: [[bool; ROCK_SIZE]; ROCK_SIZE],
    max_right: usize,
}

impl FromStr for Rock {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        let mut grid: [[bool; ROCK_SIZE]; ROCK_SIZE] = [[false; ROCK_SIZE]; ROCK_SIZE];
        for j in 0..ROCK_SIZE {
            for i in 0..ROCK_SIZE {
                grid[j][i] = chars[j][i] == '#';
            }
        }
        //Set the bottom line to index 0 for easier computing
        grid.reverse();

        let max_right = grid
            .iter()
            .map(|row| row.iter().rposition(|b| *b).unwrap_or(0))
            .max()
            .unwrap();

        Ok(Rock { grid, max_right })
    }
}

struct Rocks {
    rocks: [Rock; ROCK_NB],
    idx: usize,
}

impl Rocks {
    fn next(&mut self) -> Rock {
        let res = self.rocks[self.idx].clone();
        if self.idx < ROCK_NB - 1 {
            self.idx += 1;
        } else {
            self.idx = 0;
        }
        res
    }
}

struct Chamber {
    grid: [Vec<bool>; WIDTH],
    rocks: Rocks,
    jets: Jets,
    nb_rock: usize,
}

impl Chamber {
    fn new_rock(&mut self) {
        let rock: Rock = self.rocks.next();
        let tower_size = self.tower_size();
        let mut pos: Pos = (3, tower_size + 4);
        self.extend_grid(tower_size + 8);

        let mut stuck: bool = false;
        while !stuck {
            let jet: isize = self.jets.next();
            pos = self.jet_stream(&rock, pos, jet);
            (pos, stuck) = self.rock_fall(&rock, pos);
        }
        self.lay_rock(rock, pos);
        self.nb_rock += 1;
    }

    fn lay_rock(&mut self, rock: Rock, (x, y): Pos) {
        (0..ROCK_SIZE).for_each(|i| {
            (0..ROCK_SIZE).for_each(|j| {
                if rock.grid[j][i] {
                    self.grid[x + i - 1][y + j] = true;
                }
            })
        });
    }

    fn rock_fall(&mut self, rock: &Rock, pos: Pos) -> (Pos, bool) {
        let new_pos: Pos = (pos.0, pos.1 - 1);
        if self.collision(rock, &new_pos) {
            (pos, true)
        } else {
            (new_pos, false)
        }
    }

    fn jet_stream(&mut self, rock: &Rock, pos: Pos, jet: isize) -> Pos {
        let new_pos: Pos = (((pos.0 as isize) + jet) as usize, pos.1);
        if self.collision(rock, &new_pos) {
            pos
        } else {
            new_pos
        }
    }

    fn collision(&self, rock: &Rock, pos: &Pos) -> bool {
        match pos {
            (0, _) => true,
            (_, 0) => true,
            (x, _) if x + rock.max_right > 7 => true,
            (x, y) => (0..ROCK_SIZE).any(|i| {
                (0..ROCK_SIZE).any(|j| {
                    if (x + i - 1) < 7 {
                        rock.grid[j][i] && self.grid[x + i - 1][y + j]
                    } else {
                        false
                    }
                })
            }),
        }
    }

    fn tower_size(&self) -> usize {
        self.grid
            .iter()
            .map(|col| col.iter().rposition(|b| *b).unwrap_or(0))
            .max()
            .unwrap()
    }

    fn extend_grid(&mut self, to: usize) {
        let n: usize = to - self.grid[0].len();
        if n > 0 {
            for col in self.grid.iter_mut() {
                col.extend(vec![false; n]);
            }
        }
    }

    #[allow(dead_code)]
    fn line_at(&self, n: usize) -> Vec<bool> {
        (0..WIDTH).map(|i| self.grid[i][n]).collect()
    }

    #[allow(dead_code)]
    fn print(&self) {
        println!("Printing chamber");
        let size = self.tower_size() + 1;
        for j in (0..size).rev() {
            for i in 0..WIDTH {
                let c: char = if self.grid[i][j] { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_17.txt").expect("Cannot open input file");

    let jets: Jets = s.parse().unwrap();
    let rocks: Rocks = build_rocks();

    const EMPTY: Vec<bool> = Vec::new();
    let mut chamber = Chamber {
        grid: [EMPTY; WIDTH],
        rocks,
        jets,
        nb_rock: 0,
    };

    while chamber.nb_rock < YEAR {
        chamber.new_rock();
    }
    println!(
        "Part1: When using {} pieces, the tower size is {}",
        YEAR,
        chamber.tower_size()
    );

    let line_by_cycle: usize = 2671;
    let rock_by_cycle: usize = 1695;
    let rest: usize = BILLION % rock_by_cycle;

    //We want a number of rocks large enough before stopping to be sure the cycle has already started
    while chamber.nb_rock < rest + 3 * rock_by_cycle {
        chamber.new_rock();
    }

    let starting_lines: usize = chamber.tower_size();
    let response = line_by_cycle * (BILLION / rock_by_cycle - 3) + starting_lines;
    println!(
        "Part2: When using {} pieces, the tower size is {}",
        BILLION, response
    );

    // The code below was used to find the number of line by cycle, then computing the number of rocks by cycle
    //
    // while chamber.nb_rock < 202200 {
    //     chamber.new_rock();
    // }
    //
    // let mut line_by_cycle = 0;
    // loop {
    //     let start_line = 5700;
    //     line_by_cycle += 1;
    //     if (0..50).all(|i| {
    //         let line_n = chamber.line_at(start_line + i);
    //         (1..=50).all(|j| line_n == chamber.line_at(start_line + i + j * line_by_cycle))
    //     }) {
    //         break;
    //     }
    // }
    //
    // let nb_lines = chamber.tower_size();
    // let nb_rocks = chamber.nb_rock;
    // while chamber.tower_size() < nb_lines + line_by_cycle {
    //     chamber.new_rock();
    // }
    // let rock_by_cycle: usize = chamber.nb_rock - nb_rocks;
    // println!(
    //     "Line by cycle size: {}, Rocks by cycle: {}",
    //     line_by_cycle, rock_by_cycle
    // );

    println!("Computing time: {:?}", now.elapsed());
}

fn build_rocks() -> Rocks {
    let line: Rock = LINE.parse().unwrap();
    let cross: Rock = CROSS.parse().unwrap();
    let angle: Rock = ANGLE.parse().unwrap();
    let col: Rock = COL.parse().unwrap();
    let square: Rock = SQUARE.parse().unwrap();

    Rocks {
        rocks: [line, cross, angle, col, square],
        idx: 0,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part_1() {
        let jets: Jets = INPUT.parse().unwrap();
        let rocks: Rocks = build_rocks();

        const EMPTY: Vec<bool> = Vec::new();
        let mut chamber = Chamber {
            grid: [EMPTY; WIDTH],
            rocks,
            jets,
            nb_rock: 0,
        };

        while chamber.nb_rock < YEAR {
            chamber.new_rock();
        }
        assert_eq!(chamber.tower_size(), 3068);
    }

    #[test]
    fn part_2() {
        let jets: Jets = INPUT.parse().unwrap();
        let rocks: Rocks = build_rocks();

        const EMPTY: Vec<bool> = Vec::new();
        let mut chamber = Chamber {
            grid: [EMPTY; WIDTH],
            rocks,
            jets,
            nb_rock: 0,
        };

        let line_by_cycle: usize = 53;
        let rock_by_cycle: usize = 35;
        let rest: usize = BILLION % rock_by_cycle;

        //We want a number of rocks large enough before stopping to be sure the cycle has already started
        while chamber.nb_rock < rest + 3 * rock_by_cycle {
            chamber.new_rock();
        }

        let starting_lines: usize = chamber.tower_size();
        let response = line_by_cycle * (BILLION / rock_by_cycle - 3) + starting_lines;

        assert_eq!(response, 1_514_285_714_288);
    }
}
