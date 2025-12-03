use nom::IResult;
use nom::Parser;
use nom::character::complete::anychar;
use nom::sequence::pair;
use std::str::FromStr;
use util::basic_parser::parse_isize;

#[derive(Copy, Clone, Debug)]
enum Rotation {
    Left(isize),
    Right(isize),
}

impl FromStr for Rotation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_rot(s: &str) -> IResult<&str, (char, isize)> {
            let (s, (c, n)) = pair(anychar, parse_isize).parse(s)?;
            Ok((s, (c, n)))
        }
        let (c, n) = parse_rot(s).unwrap().1;
        match c {
            'L' => Ok(Rotation::Left(n)),
            'R' => Ok(Rotation::Right(n)),
            _ => Err(format!("invalid rotation: {}", s)),
        }
    }
}

struct Safe {
    rotations: Vec<Rotation>,
}

impl Safe {
    fn solve(&self) -> (isize, isize) {
        let mut pos: isize = 50;
        let mut nb_zero_stop: isize = 0;
        let mut nb_zero: isize = 0;

        for &rotation in &self.rotations {
            match rotation {
                Rotation::Left(n) => {
                    let was_zero: isize = (pos == 0) as isize;
                    let s: isize = pos - n;
                    pos = s.rem_euclid(100);
                    let is_zero: isize = (pos == 0) as isize;
                    //If we already were on 0, we pass on it one less time than expected.
                    //If we stop on 0, we pass on it one time more than expected.
                    nb_zero += s.div_euclid(-100) - was_zero + is_zero;
                }
                Rotation::Right(n) => {
                    let s: isize = pos + n;
                    //Whether we end on a 0 or not, we already passed on it
                    nb_zero += s / 100;
                    pos = s % 100;
                }
            }
            if pos == 0 {
                nb_zero_stop += 1;
            }
        }
        (nb_zero_stop, nb_zero)
    }
}

impl FromStr for Safe {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rotations: Vec<Rotation> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Safe { rotations })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_01.txt").expect("Cannot open input file");
    let safe: Safe = s.parse().unwrap();

    let (nb_zero_stop, nb_zero) = safe.solve();
    println!("Part1: The password is {}", nb_zero_stop);
    println!(
        "Part2: When count every time we passed on 0, the password is {}",
        nb_zero
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
";
    #[test]
    fn test() {
        let safe: Safe = EXAMPLE_1.parse().unwrap();
        assert_eq!(safe.solve(), (3, 6));
    }
}
