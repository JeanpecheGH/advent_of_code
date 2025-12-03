use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::cmp::{max, min, Ordering};
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;

struct Coordinates {
    coords: Vec<Pos>,
}

impl Coordinates {
    fn areas(&self, max_dist: usize) -> (usize, usize) {
        //Get min/max for x and y
        let (min_x, max_x, min_y, max_y): (usize, usize, usize, usize) = self.coords.iter().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(min_x, max_x, min_y, max_y), &Pos(x, y)| {
                (min(min_x, x), max(max_x, x), min(min_y, y), max(max_y, y))
            },
        );

        let mut area_map: FxHashMap<Pos, usize> = FxHashMap::default();
        let mut eliminated: FxHashSet<Pos> = FxHashSet::default();

        let mut in_range: usize = 0;

        //Eliminate nodes that are on the outside
        //Add each element inside to its nearest point
        for i in min_x..=max_x {
            for j in min_y..=max_y {
                let mut ngb: Option<Pos> = None;
                let mut min_dist: usize = usize::MAX;
                let mut dist_sum: usize = 0;

                for &p in self.coords.iter() {
                    let d: usize = p.distance(Pos(i, j));
                    dist_sum += d;
                    match d.cmp(&min_dist) {
                        Ordering::Less => {
                            min_dist = d;
                            ngb = Some(p);
                        }
                        Ordering::Equal => ngb = None,
                        Ordering::Greater => (),
                    }
                }

                if dist_sum < max_dist {
                    in_range += 1;
                }

                if let Some(p) = ngb {
                    if i == min_x || i == max_x || j == min_y || j == max_y {
                        eliminated.insert(p);
                    }
                    let e = area_map.entry(p).or_insert(0);
                    *e += 1;
                }
            }
        }

        let largest_area: usize = area_map
            .into_iter()
            .filter_map(|(p, v)| {
                if !eliminated.contains(&p) {
                    Some(v)
                } else {
                    None
                }
            })
            .max()
            .unwrap();

        (largest_area, in_range)
    }
}

impl FromStr for Coordinates {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> IResult<&str, Pos> {
            let (s, (x, y)) = separated_pair(parse_usize, tag(", "), parse_usize).parse(s)?;
            Ok((s, Pos(x, y)))
        }

        let coords: Vec<Pos> = s.lines().map(|l| parse_pos(l).unwrap().1).collect();

        Ok(Coordinates { coords })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_06.txt").expect("Cannot open input file");
    let coordinates: Coordinates = s.parse().unwrap();
    let (largest_area, in_range_area) = coordinates.areas(10000);

    println!("Part1: The largest area for an inside coordinate is {largest_area}");
    println!("Part2: The area of the region with a total distance to the points less thant 10000 is {in_range_area}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "1, 1
1, 6
8, 3
3, 4
5, 5
8, 9
";

    #[test]
    fn part_1() {
        let coordinates: Coordinates = EXAMPLE_1.parse().unwrap();
        assert_eq!(coordinates.areas(32), (17, 16));
    }
}
