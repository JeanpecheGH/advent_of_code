use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;

struct Reports {
    reports: Vec<Vec<usize>>,
}

impl Reports {
    fn is_safe(r: &[usize]) -> bool {
        let mut inc: Option<bool> = None;

        for pair in r.windows(2) {
            match (inc, pair[0].cmp(&pair[1])) {
                (None, Ordering::Less) => inc = Some(true),
                (None, Ordering::Greater) => inc = Some(false),
                (Some(true), Ordering::Less) => (),
                (Some(false), Ordering::Greater) => (),
                _ => return false,
            }
            if pair[0].abs_diff(pair[1]) > 3 {
                return false;
            }
        }
        true
    }

    fn is_dampened_safe(r: &[usize]) -> bool {
        (0..r.len()).any(|i| {
            let mut dampened_report: Vec<usize> = r.to_vec();
            dampened_report.remove(i);
            Reports::is_safe(&dampened_report)
        })
    }

    fn nb_safe(&self) -> usize {
        self.reports.iter().filter(|r| Reports::is_safe(r)).count()
    }

    fn nb_dampened_safe(&self) -> usize {
        self.reports
            .iter()
            .filter(|r| Reports::is_safe(r) || Reports::is_dampened_safe(r))
            .count()
    }
}

impl FromStr for Reports {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_report(s: &str) -> IResult<&str, Vec<usize>> {
            let (s, l) = separated_list1(char(' '), parse_usize).parse(s)?;

            Ok((s, l))
        }
        let reports: Vec<Vec<usize>> = s.lines().map(|l| parse_report(l).unwrap().1).collect();

        Ok(Reports { reports })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_02.txt").expect("Cannot open input file");
    let reports: Reports = s.parse().unwrap();

    println!("Part1: {} reports are safe", reports.nb_safe());
    println!(
        "Part2: Using the Problem Dampener, {} reports are safe",
        reports.nb_dampened_safe()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";
    #[test]
    fn part_1() {
        let reports: Reports = EXAMPLE_1.parse().unwrap();
        assert_eq!(reports.nb_safe(), 2);
    }
    #[test]
    fn part_2() {
        let reports: Reports = EXAMPLE_1.parse().unwrap();
        assert_eq!(reports.nb_dampened_safe(), 4);
    }
}
