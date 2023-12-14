use itertools::Either;
use std::collections::HashMap;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Copy, Clone, Eq, PartialEq, Debug, Hash)]
enum Rock {
    Round,
    Cube,
    None,
}

impl Rock {
    fn from_char(c: char) -> Rock {
        match c {
            'O' => Rock::Round,
            '#' => Rock::Cube,
            _ => Rock::None,
        }
    }
}

#[derive(Clone, Eq, PartialEq, Hash)]
struct Platform {
    rocks: Vec<Vec<Rock>>,
}

impl Platform {
    fn north_load(&self) -> usize {
        let height: usize = self.rocks.len();
        self.rocks
            .iter()
            .enumerate()
            .map(|(y, row)| {
                let nb_round = row.iter().filter(|&&r| r == Rock::Round).count();
                nb_round * (height - y)
            })
            .sum()
    }

    fn move_rock(&mut self, Pos(x, y): Pos, dir: Dir) -> Pos {
        let w: isize = self.rocks[0].len() as isize;
        let h: isize = self.rocks.len() as isize;
        let (mut new_x, mut new_y) = (x as isize, y as isize);
        let (d_x, d_y) = match dir {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        while (new_x + d_x) >= 0 && (new_x + d_x) < w && (new_y + d_y) >= 0 && (new_y + d_y) < h {
            if self.rocks[(new_y + d_y) as usize][(new_x + d_x) as usize] == Rock::None {
                new_x += d_x;
                new_y += d_y;
            } else {
                break;
            }
        }
        Pos(new_x as usize, new_y as usize)
    }

    fn x_range(&self, dir: Dir) -> impl Iterator<Item = usize> {
        let w: usize = self.rocks[0].len();
        match dir {
            Dir::North => Either::Left(0..w),
            Dir::East => Either::Right((0..w).rev()),
            Dir::South => Either::Left(0..w),
            Dir::West => Either::Left(0..w),
        }
    }

    fn y_range(&self, dir: Dir) -> impl Iterator<Item = usize> {
        let h: usize = self.rocks.len();
        match dir {
            Dir::North => Either::Left(0..h),
            Dir::East => Either::Left(0..h),
            Dir::South => Either::Right((0..h).rev()),
            Dir::West => Either::Left(0..h),
        }
    }

    fn tilt(&mut self, dir: Dir) {
        for y in self.y_range(dir) {
            for x in self.x_range(dir) {
                if self.rocks[y][x] == Rock::Round {
                    let Pos(i, j): Pos = self.move_rock(Pos(x, y), dir);
                    if i != x || j != y {
                        self.rocks[y][x] = Rock::None;
                        self.rocks[j][i] = Rock::Round;
                    }
                }
            }
        }
    }

    fn cycle(&mut self) {
        self.tilt(Dir::North);
        self.tilt(Dir::West);
        self.tilt(Dir::South);
        self.tilt(Dir::East);
    }

    fn n_cycles(&mut self, nb_cycle: usize) {
        let mut map: HashMap<Platform, usize> = HashMap::new();

        let mut i = 0;
        //We cycle until we find the first repeated position
        let cycle_length: usize = loop {
            if let Some(old_id) = map.insert(self.clone(), i) {
                break i - old_id;
            }
            self.cycle();
            i += 1;
        };

        //Compute the number of remaining cycles needed to simulate nb_cycle iterations
        let remaining: usize = (nb_cycle - i) % cycle_length;
        for _ in 0..remaining {
            self.cycle();
        }
    }
}

impl FromStr for Platform {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rocks: Vec<Vec<Rock>> = s
            .lines()
            .map(|l| l.chars().map(Rock::from_char).collect::<Vec<Rock>>())
            .collect();
        Ok(Platform { rocks })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_14.txt").expect("Cannot open input file");
    let mut platform: Platform = s.parse().unwrap();
    platform.tilt(Dir::North);

    println!(
        "Part1: When tilting the platform to the north, the total load is {}",
        platform.north_load()
    );
    platform.n_cycles(1000000000);
    println!(
        "Part2: After 1000000000 cycles, the total load is now {}",
        platform.north_load()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....
";

    #[test]
    fn part_1() {
        let mut platform: Platform = EXAMPLE_1.parse().unwrap();
        platform.tilt(Dir::North);
        assert_eq!(platform.north_load(), 136);
    }
    #[test]
    fn part_2() {
        let mut platform: Platform = EXAMPLE_1.parse().unwrap();
        platform.n_cycles(1000000000);
        assert_eq!(platform.north_load(), 64);
    }
}
