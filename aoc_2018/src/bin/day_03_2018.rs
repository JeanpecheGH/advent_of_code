use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;

#[derive(Debug)]
struct Claim {
    width: Pos,
    height: Pos,
}

impl FromStr for Claim {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_claim(s: &str) -> IResult<&str, Claim> {
            let (s, _id) = preceded(char('#'), parse_usize).parse(s)?;
            let (s, (min_x, min_y)) = preceded(
                tag(" @ "),
                separated_pair(parse_usize, char(','), parse_usize),
            )
            .parse(s)?;
            let (s, (width, height)) = preceded(
                tag(": "),
                separated_pair(parse_usize, char('x'), parse_usize),
            )
            .parse(s)?;

            let claim: Claim = Claim {
                width: Pos(min_x, min_x + width),
                height: Pos(min_y, min_y + height),
            };
            Ok((s, claim))
        }

        Ok(parse_claim(s).unwrap().1)
    }
}

#[derive(Debug)]
struct Fabric {
    claims: Vec<Claim>,
}

impl Fabric {
    fn overlap(&self) -> (usize, Option<usize>) {
        let mut coord_set: FxHashSet<Pos> = FxHashSet::default();

        let mut overlap_set: FxHashSet<Pos> = FxHashSet::default();

        for c in self.claims.iter() {
            let Pos(i, j) = c.width;
            let Pos(k, l) = c.height;

            for x in i..j {
                for y in k..l {
                    if !coord_set.insert(Pos(x, y)) {
                        overlap_set.insert(Pos(x, y));
                    }
                }
            }
        }

        let mut no_overlap: Option<usize> = None;
        for (id, c) in self.claims.iter().enumerate() {
            let Pos(i, j) = c.width;
            let Pos(k, l) = c.height;
            let mut overlap: bool = false;

            for x in i..j {
                for y in k..l {
                    if overlap_set.contains(&Pos(x, y)) {
                        overlap = true
                    }
                }
            }
            if !overlap {
                no_overlap = Some(id + 1)
            }
        }

        (overlap_set.len(), no_overlap)
    }
}

impl FromStr for Fabric {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let claims: Vec<Claim> = s.lines().map(|l| l.parse().unwrap()).collect();

        Ok(Fabric { claims })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_03.txt").expect("Cannot open input file");
    let fabric: Fabric = s.parse().unwrap();
    let (overlap_size, no_overlap) = fabric.overlap();

    println!(
        "Part1: {} square inches of fabric are within two or more claims",
        overlap_size
    );
    println!(
        "Part2: The claim #{} does not overlap at all",
        no_overlap.unwrap()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "#1 @ 1,3: 4x4
#2 @ 3,1: 4x4
#3 @ 5,5: 2x2
";

    #[test]
    fn part_1() {
        let fabric: Fabric = EXAMPLE_1.parse().unwrap();
        let (overlap_size, no_overlap) = fabric.overlap();

        assert_eq!(overlap_size, 4);
        assert_eq!(no_overlap.unwrap(), 3);
    }
}
