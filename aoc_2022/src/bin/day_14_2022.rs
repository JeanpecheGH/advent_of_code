use std::cmp::{max, min};
use std::str::FromStr;

type Pos = (usize, usize);
const MARGIN: usize = 10;
const SECOND_PART_MARGIN: usize = 200;
const FLOOR_MARGIN: usize = 3;

#[derive(Debug)]
struct RockPath {
    points: Vec<Pos>,
}

impl RockPath {
    fn min_x(&self) -> usize {
        self.points.iter().map(|(x, _)| x).min().cloned().unwrap()
    }
    fn max_x(&self) -> usize {
        self.points.iter().map(|(x, _)| x).max().cloned().unwrap()
    }

    fn max_y(&self) -> usize {
        self.points.iter().map(|(_, y)| y).max().cloned().unwrap()
    }
}

impl FromStr for RockPath {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let points: Vec<Pos> = s
            .split(" -> ")
            .map(|coords| {
                let w: Vec<&str> = coords.split(',').collect();
                (w[0].parse().unwrap(), w[1].parse().unwrap())
            })
            .collect();
        Ok(RockPath { points })
    }
}

struct RockStructure {
    grid: Vec<Vec<Option<bool>>>,
    sand_entry: usize,
    min_x: usize,
    max_y: usize,
}

impl RockStructure {
    fn pour_sand(&mut self) {
        let mut x = self.sand_entry - self.min_x;
        let mut y = 0;
        loop {
            while (y + 1) < self.max_y && self.grid[y + 1][x].is_none() {
                y += 1;
            }
            if (y + 1) == self.max_y {
                break;
            }
            let left = self.grid[y + 1][x - 1];
            let right = self.grid[y + 1][x + 1];
            match (left, right) {
                (None, _) => {
                    y += 1;
                    x -= 1;
                }
                (Some(_), None) => {
                    y += 1;
                    x += 1;
                }
                _ => {
                    self.grid[y][x] = Some(true);
                    break;
                }
            }
        }
    }

    fn nb_sand(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|p| matches!(p, Some(true))).count())
            .sum()
    }

    fn is_sand_stuck(&self) -> bool {
        self.grid[0][self.sand_entry - self.min_x].is_some()
    }

    fn fill_rocks(&mut self, rocks: &[RockPath]) {
        rocks.iter().for_each(|rock_path| {
            rock_path.points.windows(2).for_each(|line| {
                let min_x = min(line[0].0, line[1].0) - self.min_x;
                let max_x = max(line[0].0, line[1].0) - self.min_x;
                let min_y = min(line[0].1, line[1].1);
                let max_y = max(line[0].1, line[1].1);
                for i in min_x..=max_x {
                    for j in min_y..=max_y {
                        self.grid[j][i] = Some(false);
                    }
                }
            })
        })
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter() {
            for p in row.iter() {
                let c: char = match p {
                    Some(true) => 'o',
                    Some(false) => '#',
                    None => '.',
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_14.txt").expect("Cannot open input file");

    let rock_paths: Vec<RockPath> = s.lines().map(|l| l.parse().unwrap()).collect();
    let sand_entry: usize = 500;
    let min_x = rock_paths.iter().map(|rp| rp.min_x()).min().unwrap() - MARGIN;
    let max_x = rock_paths.iter().map(|rp| rp.max_x()).max().unwrap() + MARGIN;
    let max_y = rock_paths.iter().map(|rp| rp.max_y()).max().unwrap() + MARGIN;

    let mut rocks: RockStructure = RockStructure {
        grid: vec![vec![None; max_x - min_x]; max_y],
        sand_entry,
        min_x,
        max_y,
    };
    rocks.fill_rocks(&rock_paths);
    let mut nb_sand = 0;
    let mut previous_nb_sand = usize::MAX;
    while previous_nb_sand != nb_sand {
        previous_nb_sand = nb_sand;
        rocks.pour_sand();
        nb_sand = rocks.nb_sand();
    }
    println!(
        "Part1: The rock structure holds {} unit of sand before it falls into the abyss",
        nb_sand
    );

    //Part 2
    let min_x = rock_paths.iter().map(|rp| rp.min_x()).min().unwrap() - SECOND_PART_MARGIN;
    let max_x = rock_paths.iter().map(|rp| rp.max_x()).max().unwrap() + SECOND_PART_MARGIN;
    let max_y = rock_paths.iter().map(|rp| rp.max_y()).max().unwrap() + FLOOR_MARGIN;

    let mut rocks: RockStructure = RockStructure {
        grid: vec![vec![None; max_x - min_x]; max_y],
        sand_entry,
        min_x,
        max_y,
    };
    rocks.fill_rocks(&rock_paths);
    let floor: RockPath = RockPath {
        points: vec![(min_x, max_y - 1), (max_x - 1, max_y - 1)],
    };
    rocks.fill_rocks(&[floor]);
    while !rocks.is_sand_stuck() {
        rocks.pour_sand();
    }
    println!(
        "Part2: The rock structure holds {} unit of sand before it blocks the entry",
        rocks.nb_sand()
    );
    println!("Computing time: {:?}", now.elapsed());
}
