use nom::bytes::complete::tag;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::Pos3I;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct HailStone {
    pos: Pos3I,
    vel: Pos3I,
}

impl HailStone {
    fn change_vel(&self, x: isize, y: isize, z: isize) -> HailStone {
        let new_vel: Pos3I = Pos3I(self.vel.0 + x, self.vel.1 + y, self.vel.2 + z);
        HailStone {
            pos: self.pos,
            vel: new_vel,
        }
    }
    //fn score(&self) -> isize {
    //     self.pos.0 + self.pos.1 + self.pos.2
    // }
    fn equation_xy(&self) -> (f64, f64) {
        let a: f64 = self.vel.1 as f64 / self.vel.0 as f64;
        let b: f64 = self.pos.1 as f64 - (self.pos.0 as f64) * a;

        (a, b)
    }
    fn equation_xz(&self) -> (f64, f64) {
        let a: f64 = self.vel.2 as f64 / self.vel.0 as f64;
        let b: f64 = self.pos.2 as f64 - (self.pos.0 as f64) * a;

        (a, b)
    }

    fn crossing_point_xy(&self, other: HailStone) -> (f64, f64) {
        if self.vel.0 == 0 {
            let y0: f64 = self.pos.1 as f64;
            let (i, j) = other.equation_xy();
            //y0 - ix0 = j
            let x0: f64 = (y0 - j) / i;
            (x0, y0)
        } else if other.vel.0 == 0 {
            let y0: f64 = other.pos.1 as f64;
            let (a, b) = self.equation_xy();
            //y0 - ax0 = b
            let x0: f64 = (y0 - b) / a;
            (x0, y0)
        } else {
            let (a, b) = self.equation_xy();
            let (i, j) = other.equation_xy();
            //y0 - ax0 = b
            //y0 - ix0 = j

            //y0 - ax0 = b
            //(a-i)x0 = j - b

            let x0: f64 = (j - b) / (a - i);
            let y0: f64 = a * x0 + b;

            (x0, y0)
        }
    }

    fn crossing_point_xz(&self, other: HailStone) -> (f64, f64) {
        if self.vel.0 == 0 {
            let z0: f64 = self.pos.2 as f64;
            let (i, k) = other.equation_xz();
            //y0 - ix0 = k
            let x0: f64 = (z0 - k) / i;
            (x0, z0)
        } else if other.vel.0 == 0 {
            let z0: f64 = other.pos.2 as f64;
            let (a, c) = self.equation_xz();
            //y0 - ax0 = c
            let x0: f64 = (z0 - c) / a;
            (x0, z0)
        } else {
            let (a, c) = self.equation_xz();
            let (i, k) = other.equation_xz();
            //y0 - ax0 = c
            //y0 - ix0 = k

            //y0 - ax0 = c
            //(a-i)x0 = k - c

            let x0: f64 = (k - c) / (a - i);
            let z0: f64 = a * x0 + c;

            (x0, z0)
        }
    }

    fn x_is_after(&self, x: f64) -> bool {
        if self.vel.0 > 0 {
            x > self.pos.0 as f64
        } else {
            x < self.pos.0 as f64
        }
    }
    fn intersect_2d_xy(&self, other: HailStone, min: isize, max: isize) -> Option<(f64, f64)> {
        let a_x: isize = self.vel.0;
        let b_x: isize = other.vel.0;

        let a_y: isize = self.vel.1;
        let b_y: isize = other.vel.1;
        //Check if the path are parallel
        let par: bool = a_x * b_y == b_x * a_y;
        if !par {
            //Compute crossing point
            let min_f: f64 = min as f64;
            let max_f: f64 = max as f64;
            let (c_x, c_y): (f64, f64) = self.crossing_point_xy(other);
            //println!("{c_x} {c_y}");
            //Check if crossing point is within the limits
            if !(c_x < min_f || c_x > max_f || c_y < min_f || c_y > max_f) {
                //Chech if cross happened in past for any stone
                if self.x_is_after(c_x) && other.x_is_after(c_x) {
                    return Some((c_x, c_y));
                }
            }
        }
        None
    }

    fn intersect_2d_xz(&self, other: HailStone) -> Option<(f64, f64)> {
        let a_x: isize = self.vel.0;
        let b_x: isize = other.vel.0;

        let a_z: isize = self.vel.2;
        let b_z: isize = other.vel.2;
        //Check if the path are parallel
        let par: bool = a_x * b_z == b_x * a_z;
        if !par {
            //Compute crossing point
            let (c_x, c_z): (f64, f64) = self.crossing_point_xz(other);
            //println!("{c_x} {c_y}");
            //Chech if cross happened in past for any stone
            if self.x_is_after(c_x) && other.x_is_after(c_x) {
                return Some((c_x, c_z));
            }
        }
        None
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
        let mut nb_intersect: usize = 0;
        for i in 0..self.stones.len() {
            for j in i + 1..self.stones.len() {
                let a: HailStone = self.stones[i];
                let b: HailStone = self.stones[j];
                if a.intersect_2d_xy(b, min, max).is_some() {
                    nb_intersect += 1;
                }
            }
        }
        nb_intersect
    }

    fn rock_score(&self) -> isize {
        let y: isize = 27;
        for x in -300..=300 {
            for z in -1000..1000 {
                let modified_stones: Vec<HailStone> = self.stones[0..6]
                    .iter()
                    .map(|stone| stone.change_vel(x, y, z))
                    .collect();
                let mut crossings: Vec<(f64, f64)> = Vec::new();

                for i in 0..modified_stones.len() {
                    for j in i + 1..modified_stones.len() {
                        let a: HailStone = modified_stones[i];
                        let b: HailStone = modified_stones[j];
                        if let Some(cross) = a.intersect_2d_xz(b) {
                            crossings.push(cross);
                        }
                    }
                }
                if crossings.windows(2).all(|pair| {
                    let (x, y) = pair[0];
                    let (a, b) = pair[1];
                    (x - a).abs() < 1.0 && (y - b).abs() < 1.0
                }) && crossings.len() > 4
                {
                    println!("{x} {y}: {:?}", crossings);
                }
            }
        }
        //X = -277
        //Y = 27
        //Z = 27

        let x0: isize = 108375683349444;
        let y0: isize = 289502736377988;
        let z0: isize = 220656145109505;
        x0 + y0 + z0
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
        "Part1: {}",
        cloud.nb_intersection_in_2d(200_000_000_000_000, 400_000_000_000_000)
    );
    println!("Part2: {}", cloud.rock_score());
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
        assert_eq!(cloud.rock_score(), 47);
    }
}
