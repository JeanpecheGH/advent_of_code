use std::cmp::{max, Ordering};
use std::collections::HashSet;
use std::str::FromStr;

type Pos = (isize, isize);
const MIN: isize = 0;
const MAX: isize = 4_000_000;

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl Sensor {
    fn covered_range(&self, line: isize) -> Option<(isize, isize)> {
        let x: isize = self.pos.0;
        let max_range: isize =
            (x.abs_diff(self.beacon.0) + self.pos.1.abs_diff(self.beacon.1)) as isize;
        let dist_to_line: isize = self.pos.1.abs_diff(line) as isize;
        if max_range >= dist_to_line {
            let side_range = max_range - dist_to_line;
            Some((x - side_range, x + side_range + 1))
        } else {
            None
        }
    }

    fn beacon_y(&self) -> isize {
        self.beacon.1
    }

    fn beacon_x(&self) -> isize {
        self.beacon.0
    }
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&['=', ',', ' ', ':']).collect();
        let x_sensor: isize = words[3].parse().unwrap();
        let y_sensor: isize = words[6].parse().unwrap();
        let x_beacon: isize = words[13].parse().unwrap();
        let y_beacon: isize = words[16].parse().unwrap();
        Ok(Self {
            pos: (x_sensor, y_sensor),
            beacon: (x_beacon, y_beacon),
        })
    }
}

struct SensorSystem {
    sensors: Vec<Sensor>,
}

impl SensorSystem {
    fn distress_beacon(&self, line: isize, min_x: isize, max_x: isize) -> Option<isize> {
        let ranges = self.sorted_ranges(line);

        let mut acc_end: isize = isize::MIN;
        for (r_start, r_end) in ranges {
            if r_start == (acc_end + 1) && (min_x..=max_x).contains(&acc_end) {
                return Some(acc_end);
            }
            if r_start > acc_end {
                acc_end = r_end;
            } else {
                acc_end = max(acc_end, r_end);
            }
        }
        None
    }

    fn forbidden_pos(&self, line: isize) -> isize {
        let ranges = self.sorted_ranges(line);

        let mut cnt: isize = 0;
        let (mut acc_start, mut acc_end): (isize, isize) = (isize::MIN, isize::MIN);
        for (r_start, r_end) in ranges {
            if r_start > acc_end {
                cnt += acc_end - acc_start;
                (acc_start, acc_end) = (r_start, r_end);
            } else {
                acc_end = max(acc_end, r_end);
            }
        }
        cnt += acc_end - acc_start;
        cnt -= self.nb_beacon_on_line(line) as isize;
        cnt
    }

    fn sorted_ranges(&self, line: isize) -> Vec<(isize, isize)> {
        let mut ranges: Vec<(isize, isize)> = self
            .sensors
            .iter()
            .flat_map(|s| s.covered_range(line))
            .collect();

        //Sort the ranges by starting value
        ranges.sort_by(|(s_a, e_a), (s_b, e_b)| match s_a.cmp(s_b) {
            Ordering::Equal => e_a.cmp(e_b),
            Ordering::Less => Ordering::Less,
            Ordering::Greater => Ordering::Greater,
        });
        ranges
    }

    fn nb_beacon_on_line(&self, line: isize) -> usize {
        let beacons: HashSet<isize> = self
            .sensors
            .iter()
            .filter_map(|s| {
                if s.beacon_y() == line {
                    Some(s.beacon_x())
                } else {
                    None
                }
            })
            .collect();
        beacons.len()
    }
}
impl FromStr for SensorSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sensors: Vec<Sensor> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { sensors })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_15.txt").expect("Cannot open input file");

    let system: SensorSystem = s.parse().unwrap();

    let nb_forbidden: isize = system.forbidden_pos(2_000_000);
    println!(
        "Part1: {} positions cannot contain a beacon on line 2000000",
        nb_forbidden
    );

    let pos: (isize, isize) = (MIN..=MAX)
        .find_map(|y| system.distress_beacon(y, MIN, MAX).map(|x| (x, y)))
        .unwrap();
    println!(
        "Part2: The distress beacon coordinates are {:?}, its tuning frequency is {}",
        pos,
        pos.0 * MAX + pos.1
    );
    println!("Computation time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_MAX: isize = 20;

    const INPUT: &str = "Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3";

    #[test]
    fn part_1() {
        let system: SensorSystem = INPUT.parse().unwrap();

        assert_eq!(system.forbidden_pos(10), 26);
    }

    #[test]
    fn part_2() {
        let system: SensorSystem = INPUT.parse().unwrap();
        let pos: (isize, isize) = (MIN..=TEST_MAX)
            .find_map(|y| system.distress_beacon(y, MIN, TEST_MAX).map(|x| (x, y)))
            .unwrap();
        let tuning_freq = pos.0 * MAX + pos.1;
        assert_eq!(pos, (14, 11));
        assert_eq!(tuning_freq, 56_000_011);
    }
}
