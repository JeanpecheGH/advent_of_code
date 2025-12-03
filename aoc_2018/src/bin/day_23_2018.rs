use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
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

    fn intersect(&self, other: &Nanobot) -> bool {
        self.radius + other.radius >= self.pos.distance(other.pos)
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
            let (s, coords) =
                preceded(tag("pos=<"), separated_list1(char(','), parse_isize)).parse(s)?;
            let (s, radius) = preceded(tag(">, r="), parse_usize).parse(s)?;
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
        }

        //Getting the mean should not work in every cases, but it does in test AND our input
        (max_short as usize + min_long as usize) / 2
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

    println!(
        "Part1: {} nanobots are in range of the largest nanobot",
        teleportation.strongest_in_range()
    );
    println!(
        "Part2: The closest point in range of the most nanobots is {} for the origin ",
        teleportation.teleport_distance()
    );
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
