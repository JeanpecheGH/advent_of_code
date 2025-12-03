use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::{preceded, separated_pair};
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::chinese_remainders::smallest_remainder;
use util::coord::PosI;

#[derive(Copy, Clone, Debug)]
struct Robot {
    pos: PosI,
    vel: PosI,
}

impl Robot {
    fn motion(&mut self, max_x: isize, max_y: isize, times: isize) {
        // We add a constant to avoid negative coordinates
        let new_x = (self.pos.0 + times * (self.vel.0 + max_x)) % max_x;
        let new_y = (self.pos.1 + times * (self.vel.1 + max_y)) % max_y;
        self.pos = PosI(new_x, new_y)
    }
}

impl FromStr for Robot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_robot(s: &str) -> IResult<&str, Robot> {
            let (s, (x, y)) = preceded(
                tag("p="),
                separated_pair(parse_isize, char(','), parse_isize),
            )
            .parse(s)?;
            let (s, (i, j)) = preceded(
                tag(" v="),
                separated_pair(parse_isize, char(','), parse_isize),
            )
            .parse(s)?;

            let pos = PosI(x, y);
            let vel = PosI(i, j);
            Ok((s, Robot { pos, vel }))
        }

        Ok(parse_robot(s).unwrap().1)
    }
}
struct Restroom {
    robots: Vec<Robot>,
}
impl Restroom {
    #[allow(dead_code)]
    fn print_positions(max_x: isize, max_y: isize, positions: FxHashSet<PosI>) {
        for y in 0..max_y {
            for x in 0..max_x {
                let p = PosI(x, y);
                if positions.contains(&p) {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!()
        }
    }

    fn quadrants(robots: &[Robot], max_x: isize, max_y: isize) -> [usize; 4] {
        let mut quadrants: [usize; 4] = [0; 4];
        let mid_x = max_x / 2;
        let mid_y = max_y / 2;

        for robot in robots {
            match robot.pos {
                PosI(x, y) if x < mid_x && y < mid_y => quadrants[0] += 1,
                PosI(x, y) if x > mid_x && y < mid_y => quadrants[1] += 1,
                PosI(x, y) if x < mid_x && y > mid_y => quadrants[2] += 1,
                PosI(x, y) if x > mid_x && y > mid_y => quadrants[3] += 1,
                _ => (),
            }
        }
        quadrants
    }

    fn safety_factor(robots: &[Robot], max_x: isize, max_y: isize) -> usize {
        Self::quadrants(robots, max_x, max_y).iter().product()
    }

    fn entropy(robots: &[Robot], max_x: isize, max_y: isize) -> (f64, f64) {
        let q: [usize; 4] = Self::quadrants(robots, max_x, max_y);
        let horizontal: f64 = ((q[0] + q[2]) as f64).ln() + ((q[1] + q[3]) as f64).ln();
        let vertical: f64 = ((q[0] + q[1]) as f64).ln() + ((q[2] + q[3]) as f64).ln();
        (horizontal, vertical)
    }

    // The shannon entropy of the robots distribution drops periodically vertically and horizontally
    // When the cycles align, this is when the pattern is attained
    fn tree_pattern(h_entropies: &[f64], v_entropies: &[f64], max_x: isize, max_y: isize) -> usize {
        let h_mean: f64 = h_entropies.iter().sum::<f64>() / 100f64;
        let h_dev: f64 = h_entropies.iter().map(|&e| (e - h_mean).abs()).sum::<f64>() / 100f64;
        let v_mean: f64 = v_entropies.iter().sum::<f64>() / 100f64;
        let v_dev: f64 = v_entropies.iter().map(|&e| (e - v_mean).abs()).sum::<f64>() / 100f64;

        let h_offset: usize = h_entropies
            .iter()
            .position(|e| (e - h_mean).abs() > 5f64 * h_dev)
            .unwrap()
            + 1;
        let v_offset: usize = v_entropies
            .iter()
            .position(|e| (e - v_mean).abs() > 5f64 * v_dev)
            .unwrap()
            + 1;
        let r = smallest_remainder(vec![
            (max_x as i128, h_offset as i128),
            (max_y as i128, v_offset as i128),
        ]);
        r as usize
    }

    fn solve(&self, max_x: isize, max_y: isize, test: bool) -> (usize, usize) {
        let mut robots = self.robots.clone();
        let mut h_entropies: Vec<f64> = Vec::new();
        let mut v_entropies: Vec<f64> = Vec::new();

        // We first do the 100s iteration
        for _ in 0..100 {
            robots.iter_mut().for_each(|r| r.motion(max_x, max_y, 1));
            let (horizontal, vertical) = Self::entropy(&robots, max_x, max_y);
            h_entropies.push(horizontal);
            v_entropies.push(vertical)
        }
        let factor = Self::safety_factor(&robots, max_x, max_y);
        let tree_time: usize = if test {
            0
        } else {
            Self::tree_pattern(&h_entropies, &v_entropies, max_x, max_y)
        };
        (factor, tree_time)
    }
}

impl FromStr for Restroom {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let robots: Vec<Robot> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Restroom { robots })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_14.txt").expect("Cannot open input file");
    let restroom: Restroom = s.parse().unwrap();
    let (safety_factor, seconds) = restroom.solve(101, 103, false);
    println!("Part1: The safety factor after 100 seconds is {safety_factor}");
    println!("Part2: The robots take {seconds}s to align in a Christmas tree pattern");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3
";

    #[test]
    fn part_1() {
        let restroom: Restroom = EXAMPLE_1.parse().unwrap();
        assert_eq!(restroom.solve(11, 7, true).0, 12);
    }
}
