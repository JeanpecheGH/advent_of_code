use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_isize;

#[derive(Debug, Clone)]
struct ProbeLauncher {
    min_x: isize,
    max_x: isize,
    min_y: isize,
    max_y: isize,
}

impl ProbeLauncher {
    fn max_y_speed(&self) -> isize {
        //For any positive value d_y, the probe will reach y=0 with a speed -d_y.
        //So the max speed possible is the one that will reach min_y in a single tick after that
        self.min_y.abs() - 1
    }

    fn min_x_speed(&self) -> isize {
        //Min possible x speed is (1+s)*s/2 >= min_x
        //Else we won't even reach the target
        //s²+s >= 2*min_x
        (1..).find(|n| n * n + n >= (self.min_x) * 2).unwrap()
    }
    fn max_height(&self) -> isize {
        //Max height attained is the sum of all elements between 1 and max_y_speed
        let max_speed: isize = self.max_y_speed();
        (1 + max_speed) * max_speed / 2
    }

    fn reach_target(&self, init_x_vel: isize, init_y_vel: isize) -> bool {
        let mut x_vel: isize = init_x_vel;
        let mut y_vel: isize = init_y_vel;
        let mut x: isize = 0;
        let mut y: isize = 0;

        while x <= self.max_x && y >= self.min_y {
            x += x_vel;
            y += y_vel;
            if x >= self.min_x && x <= self.max_x && y >= self.min_y && y <= self.max_y {
                return true;
            }
            if x_vel > 0 {
                x_vel -= 1;
            }
            y_vel -= 1;
        }
        false
    }

    fn nb_possible_velocities(&self) -> usize {
        let mut count: usize = 0;
        //Max possible x speed is max_x
        //Min possible y speed is min_y
        for y in self.min_y..=self.max_y_speed() {
            for x in self.min_x_speed()..=self.max_x {
                if self.reach_target(x, y) {
                    count += 1;
                }
            }
        }
        count
    }
}

impl FromStr for ProbeLauncher {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_launcher(s: &str) -> IResult<&str, ProbeLauncher> {
            let (s, _) = tag("target area: x=")(s)?;
            let (s, (min_x, max_x)) =
                separated_pair(parse_isize, tag(".."), parse_isize).parse(s)?;
            let (s, _) = tag(", y=")(s)?;
            let (s, (min_y, max_y)) =
                separated_pair(parse_isize, tag(".."), parse_isize).parse(s)?;

            Ok((
                s,
                ProbeLauncher {
                    min_x,
                    max_x,
                    min_y,
                    max_y,
                },
            ))
        }
        Ok(parse_launcher(s).unwrap().1)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_17.txt").expect("Cannot open input file");
    let launcher: ProbeLauncher = s.parse().unwrap();
    println!(
        "Part1: The highest altitude reached is {}",
        launcher.max_height()
    );
    println!(
        "Part2: {} initial velocities can reach the target",
        launcher.nb_possible_velocities()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "target area: x=20..30, y=-10..-5";

    #[test]
    fn part_1() {
        let launcher: ProbeLauncher = EXAMPLE_1.parse().unwrap();
        assert_eq!(45, launcher.max_height());
    }
    #[test]
    fn part_2() {
        let launcher: ProbeLauncher = EXAMPLE_1.parse().unwrap();
        assert_eq!(112, launcher.nb_possible_velocities());
    }
}
