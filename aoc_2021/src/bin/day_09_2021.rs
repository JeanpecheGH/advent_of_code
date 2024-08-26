use itertools::Itertools;
use std::str::FromStr;
use util::coord::Pos;

#[derive(Debug, Clone)]
struct SmokeBasin {
    grid: Vec<Vec<usize>>,
}

impl SmokeBasin {
    fn is_min(&self, pos: Pos) -> bool {
        let v = self.grid[pos.1][pos.0];
        pos.neighbours_safe(self.grid[0].len(), self.grid.len())
            .iter()
            .all(|&Pos(x, y)| self.grid[y][x] > v)
    }

    fn risk_levels(&self) -> usize {
        let mut sum: usize = 0;
        for (y, row) in self.grid.iter().enumerate() {
            for (x, v) in row.iter().enumerate() {
                if self.is_min(Pos(x, y)) {
                    sum += v + 1;
                }
            }
        }
        sum
    }

    fn basin_product(&self) -> usize {
        let mut basin_grid: Vec<Vec<usize>> =
            self.grid.iter().map(|row| vec![0; row.len()]).collect();

        for depth in (0..=8).rev() {
            for (y, row) in self.grid.iter().enumerate() {
                for (x, &v) in row.iter().enumerate() {
                    if v == depth {
                        basin_grid[y][x] += 1;
                        Pos(x, y)
                            .neighbours_safe(self.grid[0].len(), self.grid.len())
                            .iter()
                            .for_each(|&Pos(ngb_x, ngb_y)| {
                                if v > self.grid[ngb_y][ngb_x] {
                                    basin_grid[ngb_y][ngb_x] += basin_grid[y][x];
                                    basin_grid[y][x] = 0;
                                }
                            })
                    }
                }
            }
        }

        basin_grid.iter().flatten().sorted().rev().take(3).product()
    }
}

impl FromStr for SmokeBasin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Ok(SmokeBasin { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_09.txt").expect("Cannot open input file");
    let basin: SmokeBasin = s.parse().unwrap();
    println!(
        "Part1: The sum of the risk levels is {}",
        basin.risk_levels()
    );
    println!(
        "Part2: The product of the size of the 3 biggest basins is {}",
        basin.basin_product()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "2199943210
3987894921
9856789892
8767896789
9899965678
";

    #[test]
    fn part_1() {
        let basin: SmokeBasin = EXAMPLE_1.parse().unwrap();
        assert_eq!(15, basin.risk_levels());
    }

    #[test]
    fn part_2() {
        let basin: SmokeBasin = EXAMPLE_1.parse().unwrap();
        assert_eq!(1134, basin.basin_product());
    }
}
