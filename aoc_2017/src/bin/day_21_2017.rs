use fxhash::FxHashMap;
use nom::bytes::complete::{tag, take_till};
use nom::character::complete::char;
use nom::combinator::rest;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::str::FromStr;

const INPUT: &str = ".#./..#/###";

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
struct Square {
    grid: Vec<Vec<bool>>,
}

impl Square {
    fn from_slice(grid_slice: &[&[bool]]) -> Square {
        let new_grid: Vec<Vec<bool>> = grid_slice.to_vec().iter().map(|row| row.to_vec()).collect();

        Square { grid: new_grid }
    }

    fn mirror(&self) -> Square {
        let new_grid: Vec<Vec<bool>> = self
            .grid
            .iter()
            .map(|row| {
                let mut new_row = row.clone();
                new_row.reverse();
                new_row
            })
            .collect();

        Square { grid: new_grid }
    }

    fn rotate(&self) -> Square {
        let size: usize = self.grid.len();
        let mut new_grid: Vec<Vec<bool>> = self.grid.clone();

        for y in 0..size {
            for x in 0..size {
                new_grid[size - x - 1][y] = self.grid[y][x];
            }
        }
        Square { grid: new_grid }
    }

    fn number_on(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|b| **b).count())
            .sum()
    }

    fn split(&self) -> Vec<Vec<Square>> {
        let size: usize = self.grid.len();
        let s: usize = if size.is_multiple_of(2) { 2 } else { 3 };

        let split: Vec<Vec<Square>> = (0..size)
            .step_by(s)
            .map(|y| {
                (0..size)
                    .step_by(s)
                    .map(|x| {
                        let rows_slice = &self.grid[y..y + s];
                        let square_slice: Vec<&[bool]> =
                            rows_slice.iter().map(|row| &row[x..x + s]).collect();
                        Square::from_slice(&square_slice)
                    })
                    .collect::<Vec<Square>>()
            })
            .collect();

        split
    }

    fn from_squares(squares: Vec<Vec<Square>>) -> Square {
        let grid: Vec<Vec<bool>> = squares
            .into_iter()
            .flat_map(|row| {
                let mut partial_grid: Vec<Vec<bool>> = vec![Vec::new(); row[0].grid.len()];

                for sq in row {
                    for (n, sub_row) in sq.grid.iter().enumerate() {
                        partial_grid[n].extend(sub_row.iter());
                    }
                }

                partial_grid
            })
            .collect();

        Square { grid }
    }
    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for b in row {
                let c: char = if *b { '#' } else { '.' };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Square {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_square(s: &str) -> IResult<&str, Square> {
            let (s, rows): (&str, Vec<&str>) =
                separated_list1(char('/'), take_till(|c| c == '/'))(s)?;

            let grid: Vec<Vec<bool>> = rows
                .into_iter()
                .map(|r| r.chars().map(|c| c == '#').collect())
                .collect();

            Ok((s, Square { grid }))
        }

        Ok(parse_square(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct Fractal {
    rules: FxHashMap<Square, Square>,
}

impl Fractal {
    fn iterate_once(&self, square: &Square) -> Square {
        //Split the big square
        let mut squares = square.split();

        //Find the corresponding rule for each submatrix
        squares.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|sq| loop {
                if let Some(target) = self.rules.get(sq) {
                    *sq = target.clone();
                    break;
                }
                let mirror: Square = sq.mirror();
                if let Some(target) = self.rules.get(&mirror) {
                    *sq = target.clone();
                    break;
                }
                *sq = sq.rotate();
            })
        });

        //Reassemble the big square
        Square::from_squares(squares)
    }
    fn iterate(&self, times: usize) -> usize {
        let mut square: Square = INPUT.parse().unwrap();

        for _ in 0..times {
            square = self.iterate_once(&square);
        }

        square.number_on()
    }
}

impl FromStr for Fractal {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_squares(s: &str) -> IResult<&str, (Square, Square)> {
            let (s, (source, target)): (&str, (&str, &str)) =
                separated_pair(take_till(|c| c == ' '), tag(" => "), rest)(s)?;

            Ok((s, (source.parse().unwrap(), target.parse().unwrap())))
        }

        let rules: FxHashMap<Square, Square> =
            s.lines().map(|l| parse_squares(l).unwrap().1).collect();

        Ok(Fractal { rules })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_21.txt").expect("Cannot open input file");
    let fractal: Fractal = s.parse().unwrap();

    println!(
        "Part1: After 5 iterations, {} pixels are on",
        fractal.iterate(5)
    );
    println!(
        "Part2: After 18 iterations, {} pixels are on",
        fractal.iterate(18)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "../.# => ##./#../...
.#./..#/### => #..#/..../..../#..#
";

    #[test]
    fn test_square_rotation() {
        let sq1: Square = "##./#.#/...".parse().unwrap();
        let sq2: Square = ".##/..#/.#.".parse().unwrap();

        assert_eq!(sq1, sq2.rotate())
    }

    #[test]
    fn test_square_mirror() {
        let sq1: Square = ".##/##./#.#".parse().unwrap();
        let sq2: Square = "##./.##/#.#".parse().unwrap();

        assert_eq!(sq1, sq2.mirror())
    }

    #[test]
    fn part_1() {
        let fractal: Fractal = EXAMPLE_1.parse().unwrap();
        assert_eq!(12, fractal.iterate(2));
    }
}
