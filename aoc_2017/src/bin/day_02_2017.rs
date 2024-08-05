use itertools::Itertools;
use std::str::FromStr;
use util::basic_parser::usize_list;

#[derive(Debug, Clone)]
struct Checksum {
    rows: Vec<Vec<usize>>,
}

impl Checksum {
    fn check(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                let (&min, &max) = row.iter().minmax().into_option().unwrap();
                max - min
            })
            .sum()
    }

    fn divisible_check(&self) -> usize {
        self.rows
            .iter()
            .map(|row| {
                for i in 0..(row.len() - 1) {
                    for j in i + 1..row.len() {
                        let a = row[i];
                        let b = row[j];
                        if a % b == 0 {
                            return a / b;
                        } else if b % a == 0 {
                            return b / a;
                        }
                    }
                }
                0
            })
            .sum()
    }
}

impl FromStr for Checksum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<Vec<usize>> = s.lines().map(|l| usize_list(l).unwrap().1).collect();
        Ok(Checksum { rows })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_02.txt").expect("Cannot open input file");
    let checksum: Checksum = s.parse().unwrap();

    println!(
        "Part1: The checksum for the spreadsheet is {}",
        checksum.check()
    );
    println!(
        "Part2: The sum of the divisible values ratio is {}",
        checksum.divisible_check()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "5 1 9 5
7 5 3
2 4 6 8";

    const EXAMPLE_2: &str = "5 9 2 8
9 4 7 3
3 8 6 5";

    #[test]
    fn part_1() {
        let mut checksum: Checksum = EXAMPLE_1.parse().unwrap();
        assert_eq!(18, checksum.check());
    }

    #[test]
    fn part_2() {
        let mut checksum: Checksum = EXAMPLE_2.parse().unwrap();
        assert_eq!(9, checksum.divisible_check());
    }
}
