use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::{char, space0};
use nom::sequence::{delimited, preceded, separated_pair};
use nom::IResult;
use std::cmp::{max, min};
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::PosI;

struct Star {
    pos: PosI,
    velocity: PosI,
}

impl Star {
    fn pos_at(&self, t: usize) -> PosI {
        let PosI(x, y) = self.pos;
        let PosI(d_x, d_y) = self.velocity;
        PosI(x + t as isize * d_x, y + t as isize * d_y)
    }
}

impl FromStr for Star {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn space_isize(s: &str) -> IResult<&str, isize> {
            preceded(space0, parse_isize)(s)
        }

        fn parse_posi(s: &str) -> IResult<&str, PosI> {
            let (s, (x, y)) = delimited(
                char('<'),
                separated_pair(space_isize, char(','), space_isize),
                char('>'),
            )(s)?;

            Ok((s, PosI(x, y)))
        }
        fn parse_star(s: &str) -> IResult<&str, Star> {
            let (s, pos) = preceded(tag("position="), parse_posi)(s)?;
            let (s, velocity) = preceded(tag(" velocity="), parse_posi)(s)?;

            Ok((s, Star { pos, velocity }))
        }

        Ok(parse_star(s).unwrap().1)
    }
}

struct StarryNight {
    stars: Vec<Star>,
}

impl StarryNight {
    fn min_max_at(&self, t: usize, nb_stars: usize) -> (isize, isize, isize, isize) {
        self.stars[0..nb_stars].iter().map(|s| s.pos_at(t)).fold(
            (isize::MAX, isize::MIN, isize::MAX, isize::MIN),
            |(min_x, max_x, min_y, max_y), PosI(x, y)| {
                (min(min_x, x), max(max_x, x), min(min_y, y), max(max_y, y))
            },
        )
    }
    fn align(&self, nb_stars: usize) -> usize {
        let mut diff_x: isize = isize::MAX;
        let mut diff_y: isize = isize::MAX;

        let mut t: usize = 0;
        loop {
            let (min_x, max_x, min_y, max_y): (isize, isize, isize, isize) =
                self.min_max_at(t, nb_stars);

            let d_x: isize = max_x - min_x;
            let d_y: isize = max_y - min_y;

            if d_x < diff_x && d_y < diff_y {
                diff_x = d_x;
                diff_y = d_y;
            } else {
                self.print_at(t - 1);
                return t - 1;
            }

            t += 1;
        }
    }

    fn print_at(&self, t: usize) {
        let moved_stars: FxHashSet<PosI> = self.stars.iter().map(|s| s.pos_at(t)).collect();

        let (min_x, max_x, min_y, max_y): (isize, isize, isize, isize) =
            self.min_max_at(t, self.stars.len());

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let c: char = if moved_stars.contains(&PosI(x, y)) {
                    '█'
                } else {
                    ' '
                };
                print!("{c}{c}");
            }
            println!();
        }
    }
}

impl FromStr for StarryNight {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stars: Vec<Star> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(StarryNight { stars })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_10.txt").expect("Cannot open input file");
    let night: StarryNight = s.parse().unwrap();

    println!("Part1: ");
    let time: usize = night.align(15);
    println!("Part2: The stars took {time} seconds to align");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "position=< 9,  1> velocity=< 0,  2>
position=< 7,  0> velocity=<-1,  0>
position=< 3, -2> velocity=<-1,  1>
position=< 6, 10> velocity=<-2, -1>
position=< 2, -4> velocity=< 2,  2>
position=<-6, 10> velocity=< 2, -2>
position=< 1,  8> velocity=< 1, -1>
position=< 1,  7> velocity=< 1,  0>
position=<-3, 11> velocity=< 1, -2>
position=< 7,  6> velocity=<-1, -1>
position=<-2,  3> velocity=< 1,  0>
position=<-4,  3> velocity=< 2,  0>
position=<10, -3> velocity=<-1,  1>
position=< 5, 11> velocity=< 1, -2>
position=< 4,  7> velocity=< 0, -1>
position=< 8, -2> velocity=< 0,  1>
position=<15,  0> velocity=<-2,  0>
position=< 1,  6> velocity=< 1,  0>
position=< 8,  9> velocity=< 0, -1>
position=< 3,  3> velocity=<-1,  1>
position=< 0,  5> velocity=< 0, -1>
position=<-2,  2> velocity=< 2,  0>
position=< 5, -2> velocity=< 1,  2>
position=< 1,  4> velocity=< 2,  1>
position=<-2,  7> velocity=< 2, -2>
position=< 3,  6> velocity=<-1, -1>
position=< 5,  0> velocity=< 1,  0>
position=<-6,  0> velocity=< 2,  0>
position=< 5,  9> velocity=< 1, -2>
position=<14,  7> velocity=<-2,  0>
position=<-3,  6> velocity=< 2, -1>
";
    #[test]
    fn part_1() {
        let night: StarryNight = EXAMPLE_1.parse().unwrap();
        let time = night.align(15);
        assert_eq!(time, 3);
    }
}
