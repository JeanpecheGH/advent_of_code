use fxhash::FxHashSet;
use itertools::Itertools;
use itertools::MinMaxResult::MinMax;
use std::str::FromStr;
use util::coord::PosI;
use util::split_blocks;

#[derive(Debug, Clone)]
struct TrenchImage {
    enhancement_hash: Vec<bool>,
    pixels: FxHashSet<PosI>,
}

impl TrenchImage {
    fn enhance(&self) -> (usize, usize) {
        let mut pixels: FxHashSet<PosI> = self.pixels.clone();

        let mut two: usize = 0;

        for i in 0..50 {
            let mut new_pixels: FxHashSet<PosI> = FxHashSet::default();
            if let MinMax(min_x, max_x) = pixels.iter().map(|PosI(x, _)| *x).minmax() {
                if let MinMax(min_y, max_y) = pixels.iter().map(|PosI(_, y)| *y).minmax() {
                    for y in min_y - 1..=max_y + 1 {
                        for x in min_x - 1..=max_x + 1 {
                            let nine: Vec<bool> = (-1..=1)
                                .cartesian_product(-1..=1)
                                .map(|(d_y, d_x)| {
                                    let new_x = x + d_x;
                                    let new_y = y + d_y;
                                    if new_x >= min_x
                                        && new_x <= max_x
                                        && new_y >= min_y
                                        && new_y <= max_y
                                    {
                                        pixels.contains(&PosI(x + d_x, y + d_y))
                                    } else if self.enhancement_hash[0] {
                                        //The image is scintillating
                                        //Every two steps, all pixels illuminate
                                        i % 2 == 1
                                    } else {
                                        false
                                    }
                                })
                                .collect();
                            let index: usize =
                                nine.into_iter().fold(
                                    0,
                                    |acc, b| {
                                        if b {
                                            acc * 2 + 1
                                        } else {
                                            acc * 2
                                        }
                                    },
                                );
                            if self.enhancement_hash[index] {
                                new_pixels.insert(PosI(x, y));
                            }
                        }
                    }
                }
            }
            pixels = new_pixels;

            if i == 1 {
                two = pixels.len();
            }
        }

        (two, pixels.len())
    }
}

impl FromStr for TrenchImage {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = split_blocks(s);
        let enhancement_hash: Vec<bool> = blocks[0]
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c == '#')
            .collect();
        let pixels: FxHashSet<PosI> = blocks[1]
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| {
                    if c == '#' {
                        Some(PosI(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .collect();
        Ok(TrenchImage {
            enhancement_hash,
            pixels,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_20.txt").expect("Cannot open input file");
    let image: TrenchImage = s.parse().unwrap();
    let (two, fifty) = image.enhance();
    println!("Part1: After 2 enhancements, {two} pixels are lit");
    println!("Part2: After 50 enhancements, {fifty} pixels are lit");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###
";

    #[test]
    fn test() {
        let image: TrenchImage = EXAMPLE_1.parse().unwrap();
        assert_eq!((35, 3351), image.enhance());
    }
}
