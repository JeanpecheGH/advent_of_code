use nom::character::complete::{char, space0, space1};
use nom::combinator::opt;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Debug, Clone)]
struct BingoBoard {
    grid: Vec<Vec<Option<usize>>>,
}

impl BingoBoard {
    fn mark(&mut self, n: usize) {
        self.grid.iter_mut().for_each(|row| {
            row.iter_mut().for_each(|opt| {
                if let Some(v) = *opt {
                    if v == n {
                        *opt = None;
                    }
                }
            })
        })
    }

    fn score(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().flatten().sum::<usize>())
            .sum()
    }

    fn win_score(&self) -> Option<usize> {
        let win: bool = self.grid.iter().any(|r| r.iter().all(|o| o.is_none()));
        let win: bool = if win {
            win
        } else {
            (0..5).any(|x| (0..5).all(|y| self.grid[y][x].is_none()))
        };
        if win {
            Some(self.score())
        } else {
            None
        }
    }
}

impl FromStr for BingoBoard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(s: &str) -> IResult<&str, Vec<Option<usize>>> {
            let (s, nbs) = separated_list1(space1, preceded(opt(space0), parse_usize))(s)?;

            let row: Vec<Option<usize>> = nbs.into_iter().map(Some).collect();
            Ok((s, row))
        }

        let grid: Vec<Vec<Option<usize>>> = s.lines().map(|l| parse_line(l).unwrap().1).collect();
        Ok(BingoBoard { grid })
    }
}

#[derive(Debug, Clone)]
struct Bingo {
    numbers: Vec<usize>,
    boards: Vec<BingoBoard>,
}

impl Bingo {
    fn play(&self) -> (usize, usize) {
        let mut boards = self.boards.clone();
        let mut first: usize = 0;
        let mut last: usize = 0;

        for &n in &self.numbers {
            boards.iter_mut().for_each(|b| b.mark(n));
            boards.retain(|board| {
                if let Some(s) = board.win_score() {
                    if first == 0 {
                        first = s * n;
                    }
                    last = s * n;
                    false
                } else {
                    true
                }
            })
        }
        (first, last)
    }
}

impl FromStr for Bingo {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_numbers(s: &str) -> IResult<&str, Vec<usize>> {
            separated_list1(char(','), parse_usize)(s)
        }

        let blocks: Vec<&str> = split_blocks(s);
        let numbers: Vec<usize> = parse_numbers(blocks[0]).unwrap().1;
        let boards: Vec<BingoBoard> = blocks[1..].iter().map(|s| s.parse().unwrap()).collect();
        Ok(Bingo { numbers, boards })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_04.txt").expect("Cannot open input file");
    let bingo: Bingo = s.parse().unwrap();

    let (first, last) = bingo.play();
    println!("Part1: The winning board has a score of {}", first);
    println!("Part2: The last winning board has a score of {}", last);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
";

    #[test]
    fn part_1() {
        let bingo: Bingo = EXAMPLE_1.parse().unwrap();
        assert_eq!((4512, 1924), bingo.play());
    }
}
