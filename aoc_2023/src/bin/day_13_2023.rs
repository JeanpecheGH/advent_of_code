use std::str::FromStr;
use util::split_blocks;

struct Mirror {
    grid: Vec<Vec<bool>>,
}

impl Mirror {
    fn score(&self, diff: usize) -> usize {
        //We will either have a vertical mirror, or a horizontal one, but not both
        self.columns(diff) + self.rows(diff) * 100
    }

    fn columns(&self, diff: usize) -> usize {
        let h: usize = self.grid.len();
        let w: usize = self.grid[0].len();
        for x in 0..(w - 1) {
            let d: usize = (0..=x)
                .map(|i| {
                    let other_i: usize = x + x - i + 1;
                    if other_i < w {
                        (0..h)
                            .filter(|&y| self.grid[y][i] != self.grid[y][other_i])
                            .count()
                    } else {
                        0
                    }
                })
                .sum();
            if d == diff {
                return x + 1;
            }
        }
        //No vertical mirror found, we return 0
        0
    }

    fn rows(&self, diff: usize) -> usize {
        let h: usize = self.grid.len();
        let w: usize = self.grid[0].len();
        for y in 0..(h - 1) {
            let d: usize = (0..=y)
                .map(|j| {
                    let other_j: usize = y + y - j + 1;
                    if other_j < h {
                        (0..w)
                            .filter(|&x| self.grid[j][x] != self.grid[other_j][x])
                            .count()
                    } else {
                        0
                    }
                })
                .sum();
            if d == diff {
                return y + 1;
            }
        }
        //No horizontal mirror found, we return 0
        0
    }
}
impl FromStr for Mirror {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<bool>> = s
            .lines()
            .map(|line| line.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect();
        Ok(Mirror { grid })
    }
}

struct Notes {
    mirrors: Vec<Mirror>,
}

impl Notes {
    fn summarize(&self, diff: usize) -> usize {
        self.mirrors.iter().map(|m| m.score(diff)).sum()
    }
}

impl FromStr for Notes {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mirrors: Vec<Mirror> = split_blocks(s)
            .iter()
            .map(|&block| block.parse().unwrap())
            .collect();
        Ok(Notes { mirrors })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_13.txt").expect("Cannot open input file");
    let notes: Notes = s.parse().unwrap();

    println!(
        "Part1: When finding all the mirrors, we get a sum of {}",
        notes.summarize(0)
    );
    println!(
        "Part2: After fixing all the smudges on the mirrors, the new sum is {}",
        notes.summarize(1)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#
";

    #[test]
    fn part_1() {
        let notes: Notes = EXAMPLE_1.parse().unwrap();
        assert_eq!(notes.summarize(0), 405);
    }
    #[test]
    fn part_2() {
        let notes: Notes = EXAMPLE_1.parse().unwrap();
        assert_eq!(notes.summarize(1), 400);
    }
}
