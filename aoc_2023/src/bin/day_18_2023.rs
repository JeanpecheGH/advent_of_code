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

    fn dir(&self, big: bool) -> Dir {
        if big {
            self.big_dir
        } else {
            self.dir
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
            map_res(anychar, |c| Dir::from_char(c)),
            char(' '),
            separated_pair(
                parse_usize,
                char(' '),
                delimited(
                    tag("(#"),
                    pair(
                        map_res(take(5_usize), |s| from_hex(s)),
                        map_res(anychar, |c| dir_from_n(c)),
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
    fn volume(&self, ext_is_right: bool, big: bool) -> usize {
        //TODO: compute outside ? (ext_is_right)

        //Build a vec of all the corner we get to while digging
        let mut pos: PosI = PosI(0, 0);
        let mut nodes: Vec<PosI> = vec![pos];

        for tr in self.trenches.iter() {
            pos = tr.dig_to(pos, big);
            nodes.push(pos);
        }

        //Transform nodes to "outside nodes" depending on how we rotate
        let mut trenches_dir: Vec<Dir> = self.trenches.iter().map(|tr| tr.dir(big)).collect();

        let last_dir: Dir = trenches_dir.iter().last().copied().unwrap();
        trenches_dir.push(trenches_dir.first().copied().unwrap());
        trenches_dir.reverse();
        trenches_dir.push(last_dir);
        trenches_dir.reverse();

        let mut i = 0;
        let mut outside_nodes: Vec<PosI> = Vec::new();
        for pair in trenches_dir.windows(2) {
            let PosI(x, y): PosI = nodes[i];

            let mod_pos: PosI = match (pair[0], pair[1], ext_is_right) {
                (Dir::North, Dir::East, true) => PosI(x + 1, y + 1),
                (Dir::North, Dir::East, false) => PosI(x, y),
                (Dir::North, Dir::West, true) => PosI(x + 1, y),
                (Dir::North, Dir::West, false) => PosI(x, y + 1),
                (Dir::South, Dir::East, true) => PosI(x, y + 1),
                (Dir::South, Dir::East, false) => PosI(x + 1, y),
                (Dir::South, Dir::West, true) => PosI(x, y),
                (Dir::South, Dir::West, false) => PosI(x + 1, y + 1),
                (Dir::East, Dir::North, true) => PosI(x + 1, y + 1),
                (Dir::East, Dir::North, false) => PosI(x, y),
                (Dir::East, Dir::South, true) => PosI(x, y + 1),
                (Dir::East, Dir::South, false) => PosI(x + 1, y),
                (Dir::West, Dir::North, true) => PosI(x + 1, y),
                (Dir::West, Dir::North, false) => PosI(x, y + 1),
                (Dir::West, Dir::South, true) => PosI(x, y),
                (Dir::West, Dir::South, false) => PosI(x + 1, y + 1),
                _ => panic!(
                    "Impossible Direction combination: {:?} {:?}",
                    pair[0], pair[1]
                ),
            };
            outside_nodes.push(mod_pos);
            i += 1;
        }

        //Use formula to compute area
        let a: isize = outside_nodes
            .windows(2)
            .map(|pair| pair[0].0 * pair[1].1)
            .sum();
        let b: isize = outside_nodes
            .windows(2)
            .map(|pair| pair[0].1 * pair[1].0)
            .sum();

        (a - b).unsigned_abs() / 2
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
        lagoon.volume(false, false)
    );
    println!("Part2: With the right instructions, the lagoon will be able to hold {} cubic meters of lava", lagoon.volume(false, true));
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
        assert_eq!(lagoon.volume(false, false), 62);
    }
    #[test]
    fn part_2_test_1() {
        let lagoon: LavaLagoon = EXAMPLE_1.parse().unwrap();
        assert_eq!(lagoon.volume(false, true), 952408144115);
    }
}
