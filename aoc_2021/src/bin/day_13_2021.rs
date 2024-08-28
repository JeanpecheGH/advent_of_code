use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;
use util::split_blocks;

#[derive(Debug, Clone)]
enum OrigamiFold {
    Horizontal(usize),
    Vertical(usize),
}

impl FromStr for OrigamiFold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_fold(s: &str) -> IResult<&str, OrigamiFold> {
            let (s, (axis, v)) = preceded(
                tag("fold along "),
                separated_pair(anychar, char('='), parse_usize),
            )(s)?;
            let fold: OrigamiFold = if axis == 'x' {
                OrigamiFold::Vertical(v)
            } else {
                OrigamiFold::Horizontal(v)
            };
            Ok((s, fold))
        }

        Ok(parse_fold(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct Origami {
    dots: FxHashSet<Pos>,
    folds: Vec<OrigamiFold>,
}

impl Origami {
    fn fold(&self) -> usize {
        let mut dots: FxHashSet<Pos> = self.dots.clone();
        let mut dots_after_one_fold: usize = 0;

        for (n, fold) in self.folds.iter().enumerate() {
            match fold {
                OrigamiFold::Vertical(v) => {
                    dots = dots
                        .into_iter()
                        .map(|Pos(x, y)| {
                            if x > *v {
                                Pos(2 * *v - x, y)
                            } else {
                                Pos(x, y)
                            }
                        })
                        .collect();
                }
                OrigamiFold::Horizontal(v) => {
                    dots = dots
                        .into_iter()
                        .map(|Pos(x, y)| {
                            if y > *v {
                                Pos(x, 2 * *v - y)
                            } else {
                                Pos(x, y)
                            }
                        })
                        .collect();
                }
            }
            if n == 0 {
                dots_after_one_fold = dots.len();
            }
        }
        //Print the resulting dots
        let max_x: usize = dots.iter().map(|p| p.0).max().unwrap();
        let max_y: usize = dots.iter().map(|p| p.1).max().unwrap();

        for y in 0..=max_y {
            for x in 0..=max_x {
                let c: char = if dots.contains(&Pos(x, y)) {
                    '█'
                } else {
                    ' '
                };
                print!("{c}{c}");
            }
            println!();
        }

        dots_after_one_fold
    }
}

impl FromStr for Origami {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> IResult<&str, Pos> {
            let (s, (x, y)) = separated_pair(parse_usize, char(','), parse_usize)(s)?;
            Ok((s, Pos(x, y)))
        }
        let blocks = split_blocks(s);

        let dots: FxHashSet<Pos> = blocks[0].lines().map(|l| parse_pos(l).unwrap().1).collect();
        let folds: Vec<OrigamiFold> = blocks[1].lines().map(|l| l.parse().unwrap()).collect();

        Ok(Origami { dots, folds })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_13.txt").expect("Cannot open input file");
    let origami: Origami = s.parse().unwrap();
    println!("Part1: After one fold, {} dots remain", origami.fold());
    println!("Part2: Look up !");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5
";

    #[test]
    fn part_1_test_1() {
        let origami: Origami = EXAMPLE_1.parse().unwrap();
        assert_eq!(17, origami.fold());
    }
}
