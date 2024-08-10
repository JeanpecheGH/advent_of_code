use fxhash::FxHashMap;
use itertools::Itertools;
use nom::branch::alt;
use nom::bytes::complete::take_till;
use nom::character::complete::{anychar, char};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Copy, Clone)]
enum DanceMove {
    Spin(usize),
    Exchange(usize, usize),
    Partner(char, char),
}

impl FromStr for DanceMove {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_spin(s: &str) -> IResult<&str, DanceMove> {
            let (s, n) = preceded(char('s'), parse_usize)(s)?;

            Ok((s, DanceMove::Spin(n)))
        }
        fn parse_exchange(s: &str) -> IResult<&str, DanceMove> {
            let (s, (a, b)) = preceded(
                char('x'),
                separated_pair(parse_usize, char('/'), parse_usize),
            )(s)?;

            Ok((s, DanceMove::Exchange(a, b)))
        }
        fn parse_partner(s: &str) -> IResult<&str, DanceMove> {
            let (s, (a, b)) = preceded(char('p'), separated_pair(anychar, char('/'), anychar))(s)?;

            Ok((s, DanceMove::Partner(a, b)))
        }

        Ok(alt((parse_spin, parse_exchange, parse_partner))(s)
            .unwrap()
            .1)
    }
}

#[derive(Debug, Clone)]
struct ProgramDance {
    moves: Vec<DanceMove>,
}

impl ProgramDance {
    fn one_dance(&self, dancers: &mut [char]) {
        for m in self.moves.iter() {
            match m {
                DanceMove::Spin(n) => dancers.rotate_right(*n),
                DanceMove::Exchange(a, b) => dancers.swap(*a, *b),
                DanceMove::Partner(a, b) => {
                    let pos_a: usize = dancers.iter().find_position(|c| *c == a).unwrap().0;
                    let pos_b: usize = dancers.iter().find_position(|c| *c == b).unwrap().0;
                    dancers.swap(pos_a, pos_b);
                }
            }
        }
    }

    fn dance_once(&self, len: u8) -> String {
        let mut dancers: Vec<char> = (b'a'..(b'a' + len)).map(|c| c as char).collect();

        self.one_dance(&mut dancers);

        dancers.into_iter().join("")
    }

    fn dance_multiple(&self, len: u8, times: usize) -> String {
        let mut dancers: Vec<char> = (b'a'..(b'a' + len)).map(|c| c as char).collect();
        let mut cache: FxHashMap<String, usize> = FxHashMap::default();
        let mut nb_dance: usize = 0;

        let mut ret: Option<usize> = cache.insert(dancers.iter().join(""), nb_dance);

        //We stop at the first position repetition
        while ret.is_none() {
            self.one_dance(&mut dancers);
            nb_dance += 1;
            ret = cache.insert(dancers.iter().join(""), nb_dance);
        }

        //Compute the number of dances still to be completed
        let r = ret.unwrap();
        let still_to_dance: usize = (times - r) % (nb_dance - r);

        //Find this position in the cache instead of playing it out
        let (last_position, _) = cache
            .iter()
            .find(|&(_, &v)| {
                if still_to_dance == 0 {
                    v == nb_dance
                } else {
                    v == r + still_to_dance
                }
            })
            .unwrap();

        last_position.clone()
    }
}

impl FromStr for ProgramDance {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_moves(s: &str) -> IResult<&str, Vec<DanceMove>> {
            separated_list1(
                char(','),
                map(take_till(|c| c == ','), |w: &str| w.parse().unwrap()),
            )(s)
        }

        let moves: Vec<DanceMove> = s.lines().next().map(|l| parse_moves(l).unwrap().1).unwrap();
        Ok(ProgramDance { moves })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_16.txt").expect("Cannot open input file");
    let dance: ProgramDance = s.parse().unwrap();

    println!(
        "Part1: After one dance, the programs stand in position \"{}\"",
        dance.dance_once(16)
    );
    println!(
        "Part2: After 1 billion dances, the programs stand in position \"{}\"",
        dance.dance_multiple(16, 1_000_000_000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "s1,x3/4,pe/b
";

    #[test]
    fn part_1() {
        let dance: ProgramDance = EXAMPLE_1.parse().unwrap();
        assert_eq!("baedc", dance.dance_once(5));
    }

    #[test]
    fn part_2() {
        let dance: ProgramDance = EXAMPLE_1.parse().unwrap();
        assert_eq!("abcde", dance.dance_multiple(5, 1_000_000_000));
    }
}
