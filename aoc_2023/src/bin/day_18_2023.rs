use nom::bytes::complete::{tag, take};
use nom::character::complete::{anychar, char};
use nom::combinator::map_res;
use nom::sequence::{delimited, pair, separated_pair};
use std::str::FromStr;
use util::basic_parser::{from_hex, parse_usize};
use util::coord::PosI;
use util::orientation::Dir;

#[derive(Copy, Clone, Debug)]
struct Trench {
    size: usize,
    dir: Dir,
    big_size: usize,
    big_dir: Dir,
}

impl Trench {
    fn size_and_dir(&self, big: bool) -> (usize, Dir) {
        if big {
            (self.big_size, self.big_dir)
        } else {
            (self.size, self.dir)
        }
    }

    fn dig_to(&self, PosI(x, y): PosI, big: bool) -> PosI {
        let (l, d): (usize, Dir) = self.size_and_dir(big);

        match d {
            Dir::North => PosI(x, y - l as isize),
            Dir::East => PosI(x + l as isize, y),
            Dir::South => PosI(x, y + l as isize),
            Dir::West => PosI(x - l as isize, y),
        }
    }
}

impl FromStr for Trench {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn dir_from_n(c: char) -> Result<Dir, String> {
            match c {
                '0' => Ok(Dir::East),
                '1' => Ok(Dir::South),
                '2' => Ok(Dir::West),
                '3' => Ok(Dir::North),
                _ => Err(format!("Invalid digging direction [{c}]")),
            }
        }

        let (_, (dir, (size, (big_size, big_dir)))) = separated_pair(
            map_res(anychar, Dir::from_char),
            char(' '),
            separated_pair(
                parse_usize,
                char(' '),
                delimited(
                    tag("(#"),
                    pair(
                        map_res(take(5_usize), from_hex),
                        map_res(anychar, dir_from_n),
                    ),
                    char(')'),
                ),
            ),
        )(s)
        .unwrap();
        Ok(Trench {
            size,
            dir,
            big_size,
            big_dir,
        })
    }
}

struct LavaLagoon {
    trenches: Vec<Trench>,
}

impl LavaLagoon {
    fn volume(&self, big: bool) -> usize {
        //Build a vec of all the corner we get to while digging
        let mut pos: PosI = PosI(0, 0);
        let mut nodes: Vec<PosI> = vec![pos];

        for tr in self.trenches.iter() {
            pos = tr.dig_to(pos, big);
            nodes.push(pos);
        }

        //Use shoelace formula to compute the "inside" area
        let shoelace: usize = nodes
            .windows(2)
            .map(|pair| pair[0].0 * pair[1].1 - pair[0].1 * pair[1].0)
            .sum::<isize>()
            .unsigned_abs()
            / 2;

        // We have 2n+5 points in "nodes"
        // n "inside" corners, n+4 "outside" corners and the starting point (0,0) twice
        // An outside corner adds 3/4, an inside one adds 1/4, so we have n+3 added area from the corners
        let corners_area: usize = (nodes.len() - 5) / 2 + 3;

        //For each side, the added area is (length-1)/2
        let sides_area: usize = self
            .trenches
            .iter()
            .map(|tr| tr.size_and_dir(big).0 - 1)
            .sum::<usize>()
            / 2;

        shoelace + corners_area + sides_area
    }
}

impl FromStr for LavaLagoon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let trenches: Vec<Trench> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(LavaLagoon { trenches })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_18.txt").expect("Cannot open input file");
    let lagoon: LavaLagoon = s.parse().unwrap();
    println!(
        "Part1: The lagoon will hold {} cubic meters of lava ",
        lagoon.volume(false)
    );
    println!("Part2: With the right instructions, the lagoon will be able to hold {} cubic meters of lava", lagoon.volume( true));
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)
";

    #[test]
    fn part_1() {
        let lagoon: LavaLagoon = EXAMPLE_1.parse().unwrap();
        assert_eq!(lagoon.volume(false), 62);
    }
    #[test]
    fn part_2() {
        let lagoon: LavaLagoon = EXAMPLE_1.parse().unwrap();
        assert_eq!(lagoon.volume(true), 952408144115);
    }
}
