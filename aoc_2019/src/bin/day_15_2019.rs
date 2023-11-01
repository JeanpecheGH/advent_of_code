use itertools::MinMaxResult::MinMax;
use itertools::{Itertools, MinMaxResult};
use std::collections::{HashMap, HashSet};
use util::coord::PosI;
use util::intcode::IntCode;
use util::orientation::Dir;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Wall = 0,
    Empty = 1,
    Oxygen = 2,
}

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
    pos: PosI,
}

impl Position {
    fn neighbours(&self) -> Vec<(PosI, Dir)> {
        let PosI(x, y): PosI = self.pos;
        vec![
            (PosI(x, y - 1), Dir::North),
            (PosI(x + 1, y), Dir::East),
            (PosI(x, y + 1), Dir::South),
            (PosI(x - 1, y), Dir::West),
        ]
    }

    fn tile_in_dir(&self, dir: &Dir) -> PosI {
        match dir {
            Dir::North => PosI(self.pos.0, self.pos.1 - 1),
            Dir::South => PosI(self.pos.0, self.pos.1 + 1),
            Dir::West => PosI(self.pos.0 - 1, self.pos.1),
            Dir::East => PosI(self.pos.0 + 1, self.pos.1),
        }
    }

    fn move_dir(&mut self, dir: &Dir) {
        self.pos = self.tile_in_dir(dir);
    }
}

struct Droid {
    code: IntCode,
    pos: Position,
    tiles: HashMap<PosI, (Tile, usize)>,
    finished: HashSet<PosI>,
}

impl Droid {
    fn dir_code(dir: &Dir) -> isize {
        match dir {
            Dir::North => 1,
            Dir::East => 4,
            Dir::South => 2,
            Dir::West => 3,
        }
    }

    fn from_code(code: IntCode) -> Self {
        Self {
            code,
            pos: Position { pos: PosI(0, 0) },
            tiles: HashMap::new(),
            finished: HashSet::new(),
        }
    }

    fn dist_to_oxygen(&self) -> usize {
        self.tiles
            .values()
            .find_map(|(tile, dist)| {
                if *tile == Tile::Oxygen {
                    Some(*dist)
                } else {
                    None
                }
            })
            .unwrap()
    }

    fn time_to_fill(&self) -> usize {
        let mut time: usize = 0;
        let mut to_visit: HashSet<PosI> = self
            .tiles
            .iter()
            .filter_map(|(pos, (tile, _))| {
                if *tile == Tile::Empty {
                    Some(*pos)
                } else {
                    None
                }
            })
            .collect();
        let mut current: HashSet<Position> = self
            .tiles
            .iter()
            .filter_map(|(pos, (tile, _))| {
                if *tile == Tile::Oxygen {
                    Some(Position { pos: *pos })
                } else {
                    None
                }
            })
            .collect();

        while !to_visit.is_empty() {
            current = current
                .into_iter()
                .flat_map(|position| {
                    position
                        .neighbours()
                        .into_iter()
                        .filter_map(|(pos, _)| {
                            if to_visit.remove(&pos) {
                                Some(Position { pos })
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<Position>>()
                })
                .collect();
            time += 1;
        }
        time
    }

    fn explore(&mut self) {
        //Set origin to empty
        self.tiles.insert(self.pos.pos, (Tile::Empty, 0));
        let mut ngb: Vec<(PosI, Dir)> = self.pos.neighbours();
        while ngb.iter().any(|(p, _)| !self.finished.contains(p)) {
            let move_into: Option<Dir> = ngb.iter().find_map(|(p, d)| {
                if !self.tiles.contains_key(p) {
                    Some(*d)
                } else {
                    None
                }
            });

            if let Some(dir) = move_into {
                self.move_one(&dir, true);
            } else {
                self.finished.insert(self.pos.pos);
                //We already visited all the neighbours, time to go back to the unfinished ones
                let move_into: Option<Dir> = ngb.iter().find_map(|(p, d)| {
                    if !self.finished.contains(p) {
                        Some(*d)
                    } else {
                        None
                    }
                });
                if let Some(dir) = move_into {
                    self.move_one(&dir, false);
                }
            }
            ngb = self.pos.neighbours();
        }
    }

    fn move_one(&mut self, dir: &Dir, first: bool) {
        self.code.compute(&mut vec![Self::dir_code(dir)]);
        let res: isize = self.code.output.pop().unwrap();
        let dist: usize = self.tiles.get(&self.pos.pos).unwrap().1 + 1;
        match res {
            1 => {
                self.pos.move_dir(dir);
                if first {
                    self.tiles.insert(self.pos.pos, (Tile::Empty, dist));
                }
            }
            2 => {
                self.pos.move_dir(dir);
                self.tiles.insert(self.pos.pos, (Tile::Oxygen, dist));
            }
            _ => {
                let wall: PosI = self.pos.tile_in_dir(dir);
                self.tiles.insert(wall, (Tile::Wall, dist));
                self.finished.insert(wall);
            }
        }
    }

    fn print(&self) {
        let min_max_x: MinMaxResult<isize> = self.tiles.keys().map(|PosI(x, _)| *x).minmax();
        let min_max_y: MinMaxResult<isize> = self.tiles.keys().map(|PosI(_, y)| *y).minmax();
        let (min_x, max_x): (isize, isize) = if let MinMax(a, b) = min_max_x {
            (a, b)
        } else {
            (0, 0)
        };
        let (min_y, max_y): (isize, isize) = if let MinMax(a, b) = min_max_y {
            (a, b)
        } else {
            (0, 0)
        };

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c: char = match self.tiles.get(&PosI(x, y)) {
                    None => ' ',
                    Some((Tile::Oxygen, _)) => 'O',
                    Some((Tile::Wall, _)) => '#',
                    _ => '.',
                };
                if x == 0 && y == 0 {
                    print!("X");
                } else {
                    print!("{c}");
                }
            }
            println!();
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_15.txt").expect("Cannot open input file");
    let code: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut droid: Droid = Droid::from_code(code);
    droid.explore();
    droid.print();
    println!(
        "Part1: The distance to the Oxygen System is {}",
        droid.dist_to_oxygen()
    );
    println!(
        "Part2: It will take {} minutes to fill the area with Oxygen",
        droid.time_to_fill()
    );
    println!("Computing time: {:?}", now.elapsed());
}
