use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::{Pos3I, PosI};

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HailStone2D {
    pos: PosI,
    vel: PosI,
}

impl HailStone2D {
    fn change_vel(&self, x: isize, y: isize) -> HailStone2D {
        let new_vel: PosI = PosI(self.vel.0 + x, self.vel.1 + y);
        HailStone2D {
            pos: self.pos,
            vel: new_vel,
        }
    }

    fn equation(&self) -> (f64, f64) {
        let a: f64 = self.vel.1 as f64 / self.vel.0 as f64;
        let b: f64 = self.pos.1 as f64 - (self.pos.0 as f64) * a;

        (a, b)
    }
    fn crossing_point(&self, other: &HailStone2D) -> (f64, f64) {
        if self.vel.0 == 0 {
            let x0: f64 = self.pos.0 as f64;
            let (i, j) = other.equation();
            let y0: f64 = i * x0 + j;
            (x0, y0)
        } else if other.vel.0 == 0 {
            let x0: f64 = other.pos.0 as f64;
            let (a, b) = self.equation();
            let y0: f64 = a * x0 + b;
            (x0, y0)
        } else {
            let (a, b) = self.equation();
            let (i, j) = other.equation();
            //y0 - ax0 = b
            //y0 - ix0 = j

            //y0 - ax0 = b
            //(a-i)x0 = j - b

            let x0: f64 = (j - b) / (a - i);
            let y0: f64 = a * x0 + b;

            (x0, y0)
        }
    }

    fn x_is_after(&self, x: f64) -> bool {
        match self.vel.0.cmp(&0) {
            Ordering::Less => x < self.pos.0 as f64,
            Ordering::Equal => x == self.pos.0 as f64,
            Ordering::Greater => x > self.pos.0 as f64,
        }
    }

    fn intersect_2d(&self, other: &HailStone2D) -> Result<Option<PosI>, ()> {
        let a_x: isize = self.vel.0;
        let b_x: isize = other.vel.0;

        let a_y: isize = self.vel.1;
        let b_y: isize = other.vel.1;
        //Check if the path are parallel
        let par: bool = a_x * b_y == b_x * a_y;
        if par {
            Err(())
        } else {
            //Compute crossing point
            let (c_x, c_y): (f64, f64) = self.crossing_point(other);
            //Chech if cross happened in past for any stone
            if self.x_is_after(c_x) && other.x_is_after(c_x) {
                Ok(Some(PosI(c_x.round() as isize, c_y.round() as isize)))
            } else {
                Ok(None)
            }
        }
    }
    fn intersect_2d_min_max(&self, other: &HailStone2D, min: isize, max: isize) -> Option<PosI> {
        match self.intersect_2d(other) {
            Ok(Some(PosI(x, y))) => {
                if (min..=max).contains(&x) && (min..=max).contains(&y) {
                    Some(PosI(x, y))
                } else {
                    None
                }
            }
            _ => None,
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HailStone {
    pos: Pos3I,
    vel: Pos3I,
}

impl HailStone {
    fn as_xy(&self) -> HailStone2D {
        HailStone2D {
            pos: PosI(self.pos.0, self.pos.1),
            vel: PosI(self.vel.0, self.vel.1),
        }
    }

    fn as_xz(&self) -> HailStone2D {
        HailStone2D {
            pos: PosI(self.pos.0, self.pos.2),
            vel: PosI(self.vel.0, self.vel.2),
        }
    }

    fn as_yz(&self) -> HailStone2D {
        HailStone2D {
            pos: PosI(self.pos.1, self.pos.2),
            vel: PosI(self.vel.1, self.vel.2),
        }
    }
}

impl FromStr for HailStone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos3i(s: &str) -> IResult<&str, Pos3I> {
            let (s, coords) = separated_list1(tag(", "), parse_isize)(s)?;
            Ok((s, Pos3I(coords[0], coords[1], coords[2])))
        }
        fn parse_hailstone(s: &str) -> IResult<&str, HailStone> {
            let (s, (pos, vel)) = separated_pair(parse_pos3i, tag(" @ "), parse_pos3i)(s)?;
            Ok((s, HailStone { pos, vel }))
        }
        Ok(parse_hailstone(s).unwrap().1)
    }
}

#[derive(Clone, Debug)]
struct HailCloud {
    stones: Vec<HailStone>,
}

impl HailCloud {
    fn nb_intersection_in_2d(&self, min: isize, max: isize) -> usize {
        let stones_2d: Vec<HailStone2D> = self
            .stones
            .iter()
            .map(|s| HailStone2D {
                pos: PosI(s.pos.0, s.pos.1),
                vel: PosI(s.vel.0, s.vel.1),
            })
            .collect();
        let mut nb_intersect: usize = 0;
        for i in 0..stones_2d.len() {
            for j in i + 1..stones_2d.len() {
                let a: HailStone2D = stones_2d[i];
                let b: HailStone2D = stones_2d[j];
                if a.intersect_2d_min_max(&b, min, max).is_some() {
                    nb_intersect += 1;
                }
            }
        }
        nb_intersect
    }

    fn rock_score(&self, min: isize, max: isize) -> Option<isize> {
        fn inner(
            stones: &[HailStone2D],
            min_a: isize,
            max_a: isize,
            min_b: isize,
            max_b: isize,
        ) -> Option<(isize, isize, isize, isize)> {
            for a in min_a..=max_a {
                for b in min_b..=max_b {
                    let crossings: Vec<Option<PosI>> = stones
                        .iter()
                        .map(|s| s.change_vel(a, b))
                        .circular_tuple_windows::<(_, _)>()
                        .filter_map(|(s_1, s_2)| {
                            if let Ok(cross_opt) = s_1.intersect_2d(&s_2) {
                                Some(cross_opt)
                            } else {
                                None
                            }
                        })
                        .collect();
                    if !crossings.is_empty()
                        && crossings[0].is_some()
                        && crossings.iter().all_equal()
                    {
                        let PosI(x, y) = crossings[0].unwrap();
                        return Some((a, b, x, y));
                    }
                }
            }
            None
        }

        let stones_xy: Vec<HailStone2D> = self.stones[0..5].iter().map(|s| s.as_xy()).collect();
        let stones_xz: Vec<HailStone2D> = self.stones[0..5].iter().map(|s| s.as_xz()).collect();
        let stones_yz: Vec<HailStone2D> = self.stones[0..5].iter().map(|s| s.as_yz()).collect();

        let Some((vel_x, vel_y, x_0_1, y_0_1)) = inner(&stones_xy, min, max, min, max) else {
            return None;
        };
        let Some((_, vel_z, x_0_2, z_0_1)) = inner(&stones_xz, vel_x, vel_x, min, max) else {
            return None;
        };
        let Some((_, _, y_0_2, z_0_2)) = inner(&stones_yz, vel_y, vel_y, vel_z, vel_z) else {
            return None;
        };

        if x_0_1 != x_0_2 || y_0_1 != y_0_2 || z_0_1 != z_0_2 {
            None
        } else {
            Some(x_0_1 + y_0_1 + z_0_2)
        }
    }
}

impl FromStr for HailCloud {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stones: Vec<HailStone> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(HailCloud { stones })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_24.txt").expect("Cannot open input file");
    let cloud: HailCloud = s.parse().unwrap();
    println!(
        "Part1: {} intersections are occurring in the test area",
        cloud.nb_intersection_in_2d(200_000_000_000_000, 400_000_000_000_000)
    );
    println!(
        "Part2: The sum of the coordinates of the rock at launch is {:?}",
        cloud.rock_score(-500, 500).unwrap()
    );
    assert_eq!(cloud.rock_score(-500, 500), Some(618_534_564_836_937));
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "19, 13, 30 @ -2, 1, -2
18, 19, 22 @ -1, -1, -2
20, 25, 34 @ -2, -2, -4
12, 31, 28 @ -1, -2, -1
20, 19, 15 @ 1, -5, -3
";

    #[test]
    fn part_1() {
        let cloud: HailCloud = EXAMPLE_1.parse().unwrap();
        assert_eq!(cloud.nb_intersection_in_2d(7, 27), 2);
    }
    #[test]
    fn part_2() {
        let cloud: HailCloud = EXAMPLE_1.parse().unwrap();
        assert_eq!(cloud.rock_score(-20, 20), Some(47));
    }
}
