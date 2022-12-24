use std::collections::HashSet;
use std::str::FromStr;

type Pos = (usize, usize);
const HEIGHT: usize = 20;
const WIDTH: usize = 150;

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
enum Dir {
    North,
    South,
    West,
    East,
}

#[derive(Hash, Copy, Clone, Eq, PartialEq, Debug)]
struct Blizzard {
    pos: Pos,
    dir: Dir,
}

impl Blizzard {
    fn is_at(&self, pos: &Pos) -> bool {
        self.pos == *pos
    }

    fn move_one(&self) -> Self {
        let (x, y) = self.pos;
        let (new_x, new_y) = match self.dir {
            Dir::North => (x, y - 1),
            Dir::South => (x, y + 1),
            Dir::West => (x - 1, y),
            Dir::East => (x + 1, y),
        };
        let final_x = match new_x {
            0 => WIDTH,
            n if n == WIDTH + 1 => 1,
            _ => new_x,
        };
        let final_y = match new_y {
            0 => HEIGHT,
            n if n == HEIGHT + 1 => 1,
            _ => new_y,
        };
        Blizzard {
            pos: (final_x, final_y),
            dir: self.dir,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone)]
struct Basin {
    pos: Pos,
    time: usize,
    blizzards: Vec<Blizzard>,
}

impl Basin {
    fn neighbours(&self, backwards: bool) -> Vec<Self> {
        let new_blizzards: Vec<Blizzard> =
            self.blizzards.iter().map(|bli| bli.move_one()).collect();
        let candidates: Vec<Pos> = Self::ngb_pos(&self.pos, backwards);
        candidates
            .into_iter()
            .filter(|p| !new_blizzards.iter().any(|bl| bl.is_at(p)))
            .map(|p| Basin {
                pos: p,
                time: self.time + 1,
                blizzards: new_blizzards.clone(),
            })
            .collect()
    }

    fn ngb_pos(pos: &Pos, backwards: bool) -> Vec<Pos> {
        let (x, y) = *pos;
        if y == 0 {
            vec![(1, 0), (1, 1)]
        } else if y == HEIGHT + 1 {
            vec![(WIDTH, HEIGHT), (WIDTH, HEIGHT + 1)]
        } else if x == WIDTH && y == HEIGHT && !backwards {
            vec![(WIDTH, HEIGHT + 1)]
        } else if x == 1 && y == 1 && backwards {
            vec![(1, 0)]
        } else {
            let candidates: Vec<Pos> = vec![(x - 1, y), (x, y - 1), (x, y), (x + 1, y), (x, y + 1)];
            candidates
                .into_iter()
                .filter(|(i, j)| *i > 0 && *i <= WIDTH && *j > 0 && *j <= HEIGHT)
                .collect()
        }
    }

    fn score(&self, target: &Pos) -> usize {
        target.0 - self.pos.0 + target.1 - self.pos.1 + self.time
    }
}

impl FromStr for Basin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blizzards: Vec<Blizzard> = s
            .lines()
            .enumerate()
            .flat_map(|(j, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        let pos = (i, j);
                        let opt_dir: Option<Dir> = match c {
                            '^' => Some(Dir::North),
                            'v' => Some(Dir::South),
                            '<' => Some(Dir::West),
                            '>' => Some(Dir::East),
                            _ => None,
                        };
                        if let Some(dir) = opt_dir {
                            Some(Blizzard { pos, dir })
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<Blizzard>>()
            })
            .collect();

        Ok(Basin {
            pos: (1, 0),
            time: 0,
            blizzards,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_24.txt").expect("Cannot open input file");
    let basin: Basin = s.parse().unwrap();
    let start: Pos = (1, 0);
    let target: Pos = (WIDTH, HEIGHT + 1);

    let target_basin = a_star(basin, &target, false);
    println!(
        "Part1: You take {} minutes to reach the target",
        target_basin.time
    );
    let start_again = a_star(target_basin, &start, true);
    println!(
        "And then {} minutes to reach the back to the start",
        start_again.time
    );
    let target_basin_again = a_star(start_again, &target, false);
    println!(
        "Part1: You take {} total minutes to reach the target after going back to get the snack of this silly Elf",
        target_basin_again.time
    );
    println!("Computing time: {:?}", now.elapsed());
}

fn a_star(start: Basin, target: &Pos, backwards: bool) -> Basin {
    let mut current_basins: Vec<Basin> = vec![start.clone()];
    let mut visited_basin: HashSet<Basin> = HashSet::new();
    visited_basin.insert(start);

    loop {
        let best_basin: Basin = current_basins.pop().unwrap();
        // if best_basin.score(&target) % 50 == 0 {
        //     println!(
        //         "Number of Basins: {}, pos: {:?}, score: {}",
        //         current_basins.len(),
        //         best_basin.pos,
        //         best_basin.score(&target)
        //     );
        // }
        if best_basin.pos == *target {
            return best_basin;
        }
        best_basin
            .neighbours(backwards)
            .into_iter()
            .filter(|ngb| visited_basin.insert(ngb.clone()))
            .for_each(|ngb| {
                let score: usize = ngb.score(&target);
                let idx: usize =
                    current_basins.partition_point(|other| other.score(&target) > score);
                current_basins.insert(idx, ngb);
            })
    }
}
