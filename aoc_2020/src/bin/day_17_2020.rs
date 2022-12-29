use itertools::Itertools;
use std::str::FromStr;

const PLANE_SIZE: usize = 19; //limit size of the starting plane (x and y)
const HYPER_SIZE: usize = 14; //limit size of the additional dimensions (z and w)
const NB_CYCLE: usize = 6;
type Pos3D = (usize, usize, usize);
type Pos4D = (usize, usize, usize, usize);

struct Pocket3D {
    space: [[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE],
}

impl Pocket3D {
    fn n_cycles(&mut self, n: usize) {
        for _ in 0..n {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        let mut new_space: [[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE] =
            [[[false; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE];
        for (k, plane) in self.space.iter().enumerate().skip(1) {
            for (j, row) in plane.iter().enumerate().skip(1) {
                for (i, &cube) in row.iter().enumerate().skip(1) {
                    new_space[k][j][i] = matches!(
                        (cube, self.active_ngb(&(i, j, k))),
                        (true, 2..=3) | (false, 3)
                    );
                }
            }
        }
        self.space = new_space;
    }

    fn active_ngb(&self, pos: &Pos3D) -> usize {
        Self::neighbours(pos)
            .into_iter()
            .map(|(x, y, z)| self.space[z][y][x])
            .filter(|b| *b)
            .count()
    }

    fn neighbours(pos: &Pos3D) -> Vec<Pos3D> {
        let (x, y, z) = *pos;
        (x - 1..=x + 1)
            .cartesian_product((y - 1..=y + 1).cartesian_product(z - 1..=z + 1))
            .map(|(i, (j, k))| (i, j, k))
            .filter(|&(i, j, k)| i != x || j != y || k != z)
            .filter(|&(i, j, k)| i < PLANE_SIZE && j < PLANE_SIZE && k < HYPER_SIZE)
            .collect()
    }

    fn nb_active(&self) -> usize {
        self.space
            .iter()
            .map(|plane| {
                plane
                    .iter()
                    .map(|row| row.iter().filter(|cube| **cube).count())
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for Pocket3D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //We suppose the given plan has x==y
        //The starting plan will have z=SIZE/2
        let mut space: [[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE] =
            [[[false; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE];
        let rows: Vec<&str> = s.lines().collect();
        let square_dim: usize = rows.len();
        let start: usize = (PLANE_SIZE - square_dim) / 2;

        rows.into_iter().enumerate().for_each(|(j, row)| {
            row.chars().enumerate().for_each(|(i, c)| {
                if c == '#' {
                    space[HYPER_SIZE / 2][start + j][start + i] = true
                }
            });
        });
        Ok(Pocket3D { space })
    }
}

struct Pocket4D {
    space: [[[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE]; HYPER_SIZE],
}

impl Pocket4D {
    fn n_cycles(&mut self, n: usize) {
        for _ in 0..n {
            self.cycle();
        }
    }

    fn cycle(&mut self) {
        let mut new_space: [[[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE]; HYPER_SIZE] =
            [[[[false; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE]; HYPER_SIZE];

        for (l, space) in self.space.iter().enumerate().skip(1) {
            for (k, plane) in space.iter().enumerate().skip(1) {
                for (j, row) in plane.iter().enumerate().skip(1) {
                    for (i, &cube) in row.iter().enumerate().skip(1) {
                        new_space[l][k][j][i] = matches!(
                            (cube, self.active_ngb(&(i, j, k, l))),
                            (true, 2..=3) | (false, 3)
                        );
                    }
                }
            }
        }
        self.space = new_space;
    }

    fn active_ngb(&self, pos: &Pos4D) -> usize {
        Self::neighbours(pos)
            .into_iter()
            .map(|(x, y, z, w)| self.space[w][z][y][x])
            .filter(|b| *b)
            .count()
    }

    fn neighbours(pos: &Pos4D) -> Vec<Pos4D> {
        let (x, y, z, w) = *pos;
        (x - 1..=x + 1)
            .cartesian_product(
                (y - 1..=y + 1).cartesian_product((z - 1..=z + 1).cartesian_product(w - 1..=w + 1)),
            )
            .map(|(i, (j, (k, l)))| (i, j, k, l))
            .filter(|&(i, j, k, l)| i != x || j != y || k != z || l != w)
            .filter(|&(i, j, k, l)| {
                i < PLANE_SIZE && j < PLANE_SIZE && k < HYPER_SIZE && l < HYPER_SIZE
            })
            .collect()
    }

    fn nb_active(&self) -> usize {
        self.space
            .iter()
            .map(|space| {
                space
                    .iter()
                    .map(|plane| {
                        plane
                            .iter()
                            .map(|row| row.iter().filter(|hypercube| **hypercube).count())
                            .sum::<usize>()
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for Pocket4D {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        //We suppose the given plan has x==y
        //The starting plan will have z=w=SIZE/2
        let mut space: [[[[bool; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE]; HYPER_SIZE] =
            [[[[false; PLANE_SIZE]; PLANE_SIZE]; HYPER_SIZE]; HYPER_SIZE];
        let rows: Vec<&str> = s.lines().collect();
        let square_dim: usize = rows.len();
        let start: usize = (PLANE_SIZE - square_dim) / 2;

        rows.into_iter().enumerate().for_each(|(j, row)| {
            row.chars().enumerate().for_each(|(i, c)| {
                if c == '#' {
                    space[HYPER_SIZE / 2][HYPER_SIZE / 2][start + j][start + i] = true
                }
            });
        });
        Ok(Pocket4D { space })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_17.txt").expect("Cannot open input file");
    let mut pocket3: Pocket3D = s.parse().unwrap();
    pocket3.n_cycles(NB_CYCLE);
    println!(
        "Part1: In 3 dimensions, there are {} active cube after {} cycles",
        pocket3.nb_active(),
        NB_CYCLE
    );
    let mut pocket4: Pocket4D = s.parse().unwrap();
    pocket4.n_cycles(NB_CYCLE);
    println!(
        "Part1: In 4 dimensions, there are {} active hypercube after {} cycles",
        pocket4.nb_active(),
        NB_CYCLE
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ".#.
..#
###";

    #[test]
    fn part_1() {
        let mut pocket: Pocket3D = INPUT.parse().unwrap();
        pocket.n_cycles(NB_CYCLE);
        assert_eq!(pocket.nb_active(), 112);
    }

    #[test]
    fn part_2() {
        let mut pocket: Pocket4D = INPUT.parse().unwrap();
        pocket.n_cycles(NB_CYCLE);
        assert_eq!(pocket.nb_active(), 848);
    }
}
