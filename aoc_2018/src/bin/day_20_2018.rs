use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::combinator::map;
use nom::multi::{many0, separated_list0};
use nom::sequence::delimited;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::coord::PosI;

const NB_DOORS: usize = 1000;

#[derive(Debug)]
enum RegToken {
    North,
    South,
    East,
    West,
    Reg(RegMap),
}

#[derive(Debug)]
struct RegBranch {
    tokens: Vec<RegToken>,
}

impl RegBranch {
    fn walk(&self, start_pos: PosI, current_dist: usize) -> FxHashMap<PosI, usize> {
        let mut map: FxHashMap<PosI, usize> = FxHashMap::default();
        let mut pos: PosI = start_pos;
        let mut dist: usize = current_dist;
        for t in &self.tokens {
            match t {
                RegToken::Reg(m) => {
                    let mut map_map = m.walk(pos, dist);
                    map_map.extend(&map);
                    map = map_map;
                }
                _ => {
                    match t {
                        RegToken::North => pos.1 += 1,
                        RegToken::South => pos.1 -= 1,
                        RegToken::East => pos.0 += 1,
                        RegToken::West => pos.0 -= 1,
                        _ => {}
                    }
                    dist += 1;
                    let _ = map.entry(pos).or_insert(dist);
                }
            }
        }

        map
    }
}

#[derive(Debug)]
struct RegMap {
    branches: Vec<RegBranch>,
}

impl RegMap {
    fn length_and_rooms(&self, min_doors: usize) -> (usize, usize) {
        let map = self.walk(PosI(0, 0), 0);
        let far_rooms: Vec<usize> = map
            .values()
            .filter_map(|&v| if v >= min_doors { Some(v) } else { None })
            .collect();
        (far_rooms.iter().max().copied().unwrap(), far_rooms.len())
    }

    fn walk(&self, start_pos: PosI, current_dist: usize) -> FxHashMap<PosI, usize> {
        let mut map: FxHashMap<PosI, usize> = FxHashMap::default();
        for b in &self.branches {
            let mut branch_map = b.walk(start_pos, current_dist);
            branch_map.extend(&map);
            map = branch_map;
        }
        map
    }
}

impl FromStr for RegMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_reg_branch(s: &str) -> IResult<&str, RegBranch> {
            let north = map(char('N'), |_| RegToken::North);
            let south = map(char('S'), |_| RegToken::South);
            let east = map(char('E'), |_| RegToken::East);
            let west = map(char('W'), |_| RegToken::West);
            let rm = map(parse_reg_map, RegToken::Reg);
            let parse_token = alt((north, south, east, west, rm));
            let (s, tokens) = many0(parse_token).parse(s)?;

            Ok((s, RegBranch { tokens }))
        }
        fn parse_reg_map(s: &str) -> IResult<&str, RegMap> {
            let (s, branches) = delimited(
                alt((char('('), char('^'))),
                separated_list0(tag("|"), parse_reg_branch),
                alt((char(')'), char('$'))),
            )
            .parse(s)?;
            Ok((s, RegMap { branches }))
        }

        Ok(parse_reg_map(s).unwrap().1)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_20.txt").expect("Cannot open input file");
    let map: RegMap = s.parse().unwrap();

    let (length, rooms) = map.length_and_rooms(NB_DOORS);
    println!("Part1: The furthest room is can be reached by opening {length} doors");
    println!("Part2: There are {rooms} rooms at {NB_DOORS} doors distance or more");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "^WNE$";
    const EXAMPLE_2: &str = "^ENWWW(NEEE|SSE(EE|N))$";
    const EXAMPLE_3: &str = "^ENNWSWW(NEWS|)SSSEEN(WNSE|)EE(SWEN|)NNN$";
    const EXAMPLE_4: &str = "^ESSWWN(E|NNENN(EESS(WNSE|)SSS|WWWSSSSE(SW|NNNE)))$";
    const EXAMPLE_5: &str = "^WSSEESWWWNW(S|NENNEEEENN(ESSSSW(NWSW|SSEN)|WSWWN(E|WWS(E|SS))))$";

    #[test]
    fn part_1_test_1() {
        let map: RegMap = EXAMPLE_1.parse().unwrap();
        assert_eq!((3, 2), map.length_and_rooms(2));
    }

    #[test]
    fn part_1_test_2() {
        let map: RegMap = EXAMPLE_2.parse().unwrap();
        assert_eq!((10, 10), map.length_and_rooms(6));
    }

    #[test]
    fn part_1_test_3() {
        let map: RegMap = EXAMPLE_3.parse().unwrap();
        assert_eq!((18, 13), map.length_and_rooms(10));
    }

    #[test]
    fn part_1_test_4() {
        let map: RegMap = EXAMPLE_4.parse().unwrap();
        assert_eq!((23, 6), map.length_and_rooms(20));
    }

    #[test]
    fn part_1_test_5() {
        let map: RegMap = EXAMPLE_5.parse().unwrap();
        assert_eq!((31, 7), map.length_and_rooms(30));
    }
}
