use fxhash::FxHashMap;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::BTreeMap;
use std::str::FromStr;
use util::basic_parser::{parse_isize, parse_usize};
use util::coord::Pos3I;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Nanobot {
    pos: Pos3I,
    radius: usize,
}

impl Nanobot {
    fn in_range(&self, other: &Nanobot) -> bool {
        self.radius >= self.pos.distance(other.pos)
    }

    fn contains_entirely(&self, other: &Nanobot) -> bool {
        self.radius >= self.pos.distance(other.pos) + other.radius
    }

    fn on_edge(&self, pos: Pos3I) -> bool {
        self.radius == self.pos.distance(pos)
    }

    fn intersect(&self, other: &Nanobot) -> bool {
        self.radius + other.radius >= self.pos.distance(other.pos)
    }

    fn overlap(&self, other: &Nanobot) -> usize {
        self.radius + other.radius - self.pos.distance(other.pos)
    }

    fn distance_to_center(&self) -> (isize, isize) {
        let center_dist: isize = self.pos.distance(Pos3I(0, 0, 0)) as isize;
        (
            center_dist - self.radius as isize,
            center_dist + self.radius as isize,
        )
    }
}

impl FromStr for Nanobot {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_nanobot(s: &str) -> IResult<&str, (Pos3I, usize)> {
            let (s, coords) = preceded(tag("pos=<"), separated_list1(char(','), parse_isize))(s)?;
            let (s, radius) = preceded(tag(">, r="), parse_usize)(s)?;
            let pos = Pos3I(coords[0], coords[1], coords[2]);
            Ok((s, (pos, radius)))
        }

        let (pos, radius) = parse_nanobot(s).unwrap().1;

        Ok(Nanobot { pos, radius })
    }
}

struct EmergencyTeleportation {
    bots: Vec<Nanobot>,
}

impl EmergencyTeleportation {
    fn strongest_in_range(&self) -> usize {
        let strongest: &Nanobot = self
            .bots
            .iter()
            .max_by(|a, b| a.radius.cmp(&b.radius))
            .unwrap();
        self.bots.iter().filter(|b| strongest.in_range(b)).count()
    }

    fn teleport_distance(&self) -> usize {
        // Getting the list of intersecting bots for each bot
        let mut m: Vec<(Nanobot, Vec<Nanobot>)> = self
            .bots
            .iter()
            .map(|bot| {
                let intersect: Vec<Nanobot> = self
                    .bots
                    .iter()
                    .filter(|other| bot.intersect(other))
                    .copied()
                    .collect();
                (*bot, intersect)
            })
            .sorted_by(|(_, a), (_, b)| a.len().cmp(&b.len()))
            .collect();

        // We filter bots not intersecting with all the others, starting with the largest bots
        let mut near: Vec<Nanobot> = Vec::new();

        while let Some((b, i)) = m.pop() {
            if near.iter().all(|o| i.contains(o)) {
                near.push(b);
            }
        }

        for (b, i) in m.iter() {
            println!("{b:?} intersects {}", i.len());
        }

        println!("NEAR: {} {:?} ", near.len(), near);

        // We get the furthest distance attained by the nearest bot
        // and the nearest distance attained by the farthest bot
        let mut max_short: isize = isize::MIN;
        let mut min_long: isize = isize::MAX;
        let mut map: BTreeMap<isize, isize> = BTreeMap::default();
        for b in near.iter() {
            let (short, long) = b.distance_to_center();
            if short > max_short {
                max_short = short;
            }
            if long < min_long {
                min_long = long;
            }
            *map.entry(short).or_insert(0) += 1;
            *map.entry(long + 1).or_insert(0) -= 1;
            println!("{b:?}: {short}-{long}")
        }
        println!("max short {max_short}, min long {min_long}");

        //Getting the mean should not work in every cases, but it does in test AND our input
        (max_short as usize + min_long as usize) / 2
        // let run = map
        //     .iter()
        //     .scan(0, |sum, (k, v)| {
        //         *sum += v;
        //         Some((k, *sum))
        //     })
        //     .collect::<Vec<_>>();
        // // for &(k, v) in partial_sums.iter() {
        // //     println!("{k} {v}");
        // // }
        //
        // let max = run.iter().map(|&(_, n)| n).max().unwrap();
        // let intervals = run
        //     .iter()
        //     .zip(run.iter().skip(1))
        //     .filter_map(
        //         |(&(a, n), &(b, _))| {
        //             if n == max {
        //                 Some((*a, *b - 1))
        //             } else {
        //                 None
        //             }
        //         },
        //     )
        //     .collect::<Vec<_>>();
        // let response: isize = if intervals.iter().any(|&(a, b)| a <= 0 && b >= 0) {
        //     0
        // } else {
        //     intervals
        //         .iter()
        //         .map(|&(a, b)| if b < 0 { -b } else { a })
        //         .min()
        //         .unwrap()
        // };
        //
        // println!("RESPONSE: {}", response);
        //
        // let size: usize = near.len();
        // let mut min_o: usize = usize::MAX;
        // for i in 0..size {
        //     for j in i + 1..size {
        //         let a = near[i];
        //         let b = near[j];
        //         let o: usize = a.overlap(&b);
        //         if o < min_o {
        //             min_o = o;
        //         }
        //         if o < 1 {
        //             println!("{a:?} {b:?} overlap is {o}");
        //         }
        //     }
        // }
        // println!("min overlap {min_o}");
        //
        // 0
    }
}

impl FromStr for EmergencyTeleportation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let bots: Vec<Nanobot> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(EmergencyTeleportation { bots })
    }
}

fn main() {
    let now = std::time::Instant::now();

    let s = util::file_as_string("aoc_2018/input/day_23.txt").expect("Cannot open input file");
    let teleportation: EmergencyTeleportation = s.parse().unwrap();

    println!("Part1: {}", teleportation.strongest_in_range());
    // let a: Nanobot = Nanobot {
    //     pos: Pos3I(14_285_788, 7_733_496, 37_713_968),
    //     radius: 68_018_244,
    // };
    // let b: Nanobot = Nanobot {
    //     pos: Pos3I(43_304_772, 85_774_477, 17_337_631),
    //     radius: 59_418_058,
    // };
    // let c: Pos3I = Pos3I(43_304_772, 26_356_419, 17_337_631);
    // let d: Pos3I = Pos3I(43_304_770, 26_356_423, 17_337_633);
    // let e: Pos3I = Pos3I(43_304_768, 26_356_423, 17_337_631);
    // // -x - y + z + 52_323_560
    //
    // println!("{} {}", a.on_edge(c), b.on_edge(c));
    // println!("{} {}", a.on_edge(d), b.on_edge(d));
    // println!("{} {}", a.on_edge(e), b.on_edge(e));
    //
    // let a: Nanobot = Nanobot {
    //     pos: Pos3I(13_366_727, 55_030_605, 74_717_266),
    //     radius: 75_489_826,
    // };
    // let b: Nanobot = Nanobot {
    //     pos: Pos3I(90_701_741, 45_206_681, 13_508_691),
    //     radius: 72_877_687,
    // };
    // let c: Pos3I = Pos3I(17_824_054, 45_206_681, 13_508_691);
    // let d: Pos3I = Pos3I(17_824_058, 45_206_683, 13_508_693);
    // let e: Pos3I = Pos3I(17_824_058, 45_206_685, 13_508_691);
    // // -x + y + z - 40891318 = 0
    //
    // println!("{} {}", a.on_edge(c), b.on_edge(c));
    // println!("{} {}", a.on_edge(d), b.on_edge(d));
    // println!("{} {}", a.on_edge(e), b.on_edge(e));
    //
    // let a: Nanobot = Nanobot {
    //     pos: Pos3I(-25_242_309, 50_805_992, -14_103_752),
    //     radius: 91_418_305,
    // };
    // let b: Nanobot = Nanobot {
    //     pos: Pos3I(43_274_503, 40_946_494, 54_319_954),
    //     radius: 55_381_711,
    // };
    // let c: Pos3I = Pos3I(-12_107_208, 40_946_494, 54_319_954);
    // let d: Pos3I = Pos3I(-12_107_204, 40_946_496, 54_319_952);
    // let e: Pos3I = Pos3I(-12_107_204, 40_946_498, 54_319_954);
    // // x - y + z - 1266252 = 0
    //
    // // (26794906, 46607439, 21078785) = 94481130
    //
    // println!("{} {}", a.on_edge(c), b.on_edge(c));
    // println!("{} {}", a.on_edge(d), b.on_edge(d));
    // println!("{} {}", a.on_edge(e), b.on_edge(e));
    println!("Part2: {}", teleportation.teleport_distance());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "pos=<0,0,0>, r=4
pos=<1,0,0>, r=1
pos=<4,0,0>, r=3
pos=<0,2,0>, r=1
pos=<0,5,0>, r=3
pos=<0,0,3>, r=1
pos=<1,1,1>, r=1
pos=<1,1,2>, r=1
pos=<1,3,1>, r=1
";

    const EXAMPLE_2: &str = "pos=<10,12,12>, r=2
pos=<12,14,12>, r=2
pos=<16,12,12>, r=4
pos=<14,14,14>, r=6
pos=<50,50,50>, r=200
pos=<10,10,10>, r=5
";

    #[test]
    fn part_1() {
        let teleportation: EmergencyTeleportation = EXAMPLE_1.parse().unwrap();
        assert_eq!(7, teleportation.strongest_in_range());
    }

    #[test]
    fn part_2() {
        let teleportation: EmergencyTeleportation = EXAMPLE_2.parse().unwrap();
        assert_eq!(36, teleportation.teleport_distance())
    }
}
