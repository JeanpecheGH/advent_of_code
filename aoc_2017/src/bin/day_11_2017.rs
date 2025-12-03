use nom::character::complete::{alpha1, char};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::cmp::max;
use std::str::FromStr;
use util::coord::PosI;

#[derive(Debug, Clone)]
enum HexMove {
    NorthWest,
    North,
    NorthEast,
    SouthWest,
    South,
    SouthEast,
}

impl HexMove {
    fn from(&self, PosI(x, y): PosI) -> PosI {
        match self {
            HexMove::NorthWest => PosI(x - 1, y + 1),
            HexMove::North => PosI(x, y + 1),
            HexMove::NorthEast => PosI(x + 1, y),
            HexMove::SouthWest => PosI(x - 1, y),
            HexMove::South => PosI(x, y - 1),
            HexMove::SouthEast => PosI(x + 1, y - 1),
        }
    }
}

impl FromStr for HexMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "nw" => Ok(HexMove::NorthWest),
            "n" => Ok(HexMove::North),
            "ne" => Ok(HexMove::NorthEast),
            "sw" => Ok(HexMove::SouthWest),
            "s" => Ok(HexMove::South),
            "se" => Ok(HexMove::SouthEast),
            _ => Err(()),
        }
    }
}
#[derive(Debug, Clone)]
struct HexEd {
    moves: Vec<HexMove>,
}

impl HexEd {
    fn dist_between(PosI(a, b): PosI, PosI(x, y): PosI) -> isize {
        let dx = x - a;
        let dy = y - b;

        if dx * dy >= 0 {
            (dx + dy).abs()
        } else {
            max(dx.abs(), dy.abs())
        }
    }

    fn dist(&self) -> (isize, isize) {
        let mut pos: PosI = PosI(0, 0);
        let mut max_dist: isize = 0;

        for m in &self.moves {
            pos = m.from(pos);
            let d: isize = HexEd::dist_between(pos, PosI(0, 0));
            if d > max_dist {
                max_dist = d;
            }
        }

        (HexEd::dist_between(pos, PosI(0, 0)), max_dist)
    }
}

impl FromStr for HexEd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_moves(s: &str) -> IResult<&str, Vec<HexMove>> {
            separated_list1(
                char(','),
                map(alpha1, |w: &str| w.parse::<HexMove>().unwrap()),
            )
            .parse(s)
        }

        let moves: Vec<HexMove> = s.lines().next().map(|l| parse_moves(l).unwrap().1).unwrap();
        Ok(HexEd { moves })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_11.txt").expect("Cannot open input file");
    let hex: HexEd = s.parse().unwrap();
    let (final_dist, max_dist) = hex.dist();

    println!("Part1: The child process is {final_dist} steps away");
    println!("Part2: At the fursthest, it went {max_dist} steps away");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "ne,ne,ne";
    const EXAMPLE_2: &str = "ne,ne,sw,sw";
    const EXAMPLE_3: &str = "ne,ne,s,s";
    const EXAMPLE_4: &str = "se,sw,se,sw,sw";

    #[test]
    fn part_1_test_1() {
        let hex: HexEd = EXAMPLE_1.parse().unwrap();
        assert_eq!((3, 3), hex.dist());
    }
    #[test]
    fn part_1_test_2() {
        let hex: HexEd = EXAMPLE_2.parse().unwrap();
        assert_eq!((0, 2), hex.dist());
    }
    #[test]
    fn part_1_test_3() {
        let hex: HexEd = EXAMPLE_3.parse().unwrap();
        assert_eq!((2, 2), hex.dist());
    }
    #[test]
    fn part_1_test_4() {
        let hex: HexEd = EXAMPLE_4.parse().unwrap();
        assert_eq!((3, 3), hex.dist());
    }
}
