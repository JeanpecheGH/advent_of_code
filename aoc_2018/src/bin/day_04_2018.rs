use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::{tag, take};
use nom::character::complete::char;
use nom::combinator::rest;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded, separated_pair, terminated};
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Eq, PartialEq)]
struct Date {
    year: usize,
    month: usize,
    day: usize,
    hour: usize,
    minute: usize,
}

impl Ord for Date {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year
            .cmp(&other.year)
            .then(self.month.cmp(&other.month))
            .then(self.day.cmp(&other.day))
            .then(self.hour.cmp(&other.hour))
            .then(self.minute.cmp(&other.minute))
    }
}

impl PartialOrd for Date {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Date {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_date(s: &str) -> IResult<&str, Date> {
            let (s, day_elems) = separated_list1(char('-'), parse_usize)(s)?;
            let (s, (hour, minute)) = preceded(
                char(' '),
                separated_pair(parse_usize, char(':'), parse_usize),
            )(s)?;

            Ok((
                s,
                Date {
                    year: day_elems[0],
                    month: day_elems[1],
                    day: day_elems[2],
                    hour,
                    minute,
                },
            ))
        }

        Ok(parse_date(s).unwrap().1)
    }
}

#[derive(Debug)]
enum Action {
    BeginShift(usize),
    FallAsleep,
    WakeUp,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_shift(s: &str) -> IResult<&str, Action> {
            let (s, id) = preceded(
                tag("Guard #"),
                terminated(parse_usize, tag(" begins shift")),
            )(s)?;
            Ok((s, Action::BeginShift(id)))
        }

        fn parse_fall_asleep(s: &str) -> IResult<&str, Action> {
            let (s, _) = tag("falls asleep")(s)?;

            Ok((s, Action::FallAsleep))
        }

        fn parse_wake_up(s: &str) -> IResult<&str, Action> {
            let (s, _) = tag("wakes up")(s)?;

            Ok((s, Action::WakeUp))
        }
        fn parse_action(s: &str) -> IResult<&str, Action> {
            alt((parse_shift, parse_fall_asleep, parse_wake_up))(s)
        }

        Ok(parse_action(s).unwrap().1)
    }
}

#[derive(Debug)]
struct Log {
    date: Date,
    action: Action,
}

impl FromStr for Log {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_log(s: &str) -> IResult<&str, Log> {
            let (s, date_str): (&str, &str) = delimited(char('['), take(16usize), char(']'))(s)?;
            let (s, action_str): (&str, &str) = preceded(char(' '), rest)(s)?;

            let date: Date = date_str.parse().unwrap();
            let action: Action = action_str.parse().unwrap();
            Ok((s, Log { date, action }))
        }

        Ok(parse_log(s).unwrap().1)
    }
}

#[derive(Debug)]
struct Records {
    logs: Vec<Log>,
}

impl Records {
    fn sort(&mut self) {
        self.logs.sort_by(|a, b| a.date.cmp(&b.date))
    }

    fn strategies(&mut self) -> (usize, usize) {
        self.sort();

        let mut guard_map: FxHashMap<usize, [usize; 60]> = FxHashMap::default();

        let mut id: usize = 0;
        let mut start: usize = 0;

        for log in self.logs.iter() {
            match log.action {
                Action::BeginShift(n) => id = n,
                Action::FallAsleep => start = log.date.minute,
                Action::WakeUp => {
                    let e = guard_map.entry(id).or_insert([0; 60]);
                    for x in e.iter_mut().take(log.date.minute).skip(start) {
                        *x += 1;
                    }
                }
            }
        }

        let sleepy_minutes_map: FxHashMap<usize, (usize, usize)> = guard_map
            .iter()
            .map(|(&id, naps)| {
                (
                    id,
                    naps.iter()
                        .enumerate()
                        .fold((0usize, 0usize), |(minute, max), (i, &v)| {
                            if v > max {
                                (i, v)
                            } else {
                                (minute, max)
                            }
                        }),
                )
            })
            .collect();

        let sleepy_id_1: usize = guard_map
            .iter()
            .map(|(&id, naps)| (id, naps.iter().sum::<usize>()))
            .max_by(|&a, &b| a.1.cmp(&b.1))
            .unwrap()
            .0;

        let sleepy_minute_1: usize = sleepy_minutes_map.get(&sleepy_id_1).unwrap().0;

        let (sleepy_id_2, sleepy_minute_2) = sleepy_minutes_map
            .iter()
            .max_by(|(_, (_, max_1)), (_, (_, max_2))| max_1.cmp(max_2))
            .map(|(id, (m, _))| (id, m))
            .unwrap();
        (sleepy_id_1 * sleepy_minute_1, sleepy_id_2 * sleepy_minute_2)
    }
}

impl FromStr for Records {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let logs: Vec<Log> = s.lines().map(|l| l.parse().unwrap()).collect();

        Ok(Records { logs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_04.txt").expect("Cannot open input file");
    let mut records: Records = s.parse().unwrap();
    let (part_1, part_2) = records.strategies();

    println!("Part1: When using strategy 1, the product is {part_1}");
    println!("Part2: When using strategy 2, the product is {part_2}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "[1518-11-01 00:00] Guard #10 begins shift
[1518-11-01 00:05] falls asleep
[1518-11-01 00:25] wakes up
[1518-11-01 00:30] falls asleep
[1518-11-01 00:55] wakes up
[1518-11-01 23:58] Guard #99 begins shift
[1518-11-02 00:40] falls asleep
[1518-11-02 00:50] wakes up
[1518-11-03 00:05] Guard #10 begins shift
[1518-11-03 00:24] falls asleep
[1518-11-03 00:29] wakes up
[1518-11-04 00:02] Guard #99 begins shift
[1518-11-04 00:36] falls asleep
[1518-11-04 00:46] wakes up
[1518-11-05 00:03] Guard #99 begins shift
[1518-11-05 00:45] falls asleep
[1518-11-05 00:55] wakes up
";

    #[test]
    fn part_1() {
        let mut records: Records = EXAMPLE_1.parse().unwrap();

        assert_eq!(records.strategies(), (240, 4455));
    }
}
