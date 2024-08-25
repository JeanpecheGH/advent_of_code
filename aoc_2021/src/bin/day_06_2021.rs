use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct Lanternfish {
    fishes: Vec<usize>,
}

impl Lanternfish {
    fn days(&self, d: usize) -> usize {
        let mut fish_pop: Vec<usize> = vec![0; 9];

        for &n in &self.fishes {
            fish_pop[n] += 1;
        }

        for _ in 0..d {
            fish_pop.rotate_left(1);
            fish_pop[6] += fish_pop[8]
        }

        fish_pop.iter().sum()
    }
}

impl FromStr for Lanternfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(s: &str) -> IResult<&str, Vec<usize>> {
            separated_list1(char(','), parse_usize)(s)
        }

        let fishes: Vec<usize> = s.lines().next().map(|l| parse_line(l).unwrap().1).unwrap();
        Ok(Lanternfish { fishes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_06.txt").expect("Cannot open input file");
    let fishes: Lanternfish = s.parse().unwrap();

    println!(
        "Part1: After 80 days there are {} lanternfishes",
        fishes.days(80)
    );
    println!(
        "Part2: After 256 days there are {} lanternfishes",
        fishes.days(256)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "3,4,3,1,2";

    #[test]
    fn part_1() {
        let fishes: Lanternfish = EXAMPLE_1.parse().unwrap();
        assert_eq!(5934, fishes.days(80));
    }
    #[test]
    fn part_2() {
        let fishes: Lanternfish = EXAMPLE_1.parse().unwrap();
        assert_eq!(26984457539, fishes.days(256));
    }
}
