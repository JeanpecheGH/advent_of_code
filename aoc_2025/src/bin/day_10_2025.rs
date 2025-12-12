use fxhash::FxHashSet;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::char;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug)]
struct Machine {
    target: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl Machine {
    fn fewest_presses_lights(&self) -> usize {
        let mut cache: Vec<FxHashSet<usize>> = Vec::new();
        let zero: FxHashSet<usize> = FxHashSet::from_iter(vec![0]);
        cache.push(zero);

        let mut i: usize = 0;
        loop {
            let mut new_set: FxHashSet<usize> = FxHashSet::default();
            for n in &cache[i] {
                for b in &self.buttons {
                    let r: usize = n ^ b;
                    if r == self.target {
                        return i + 1;
                    } else {
                        new_set.insert(r);
                    }
                }
            }
            cache.push(new_set);
            i += 1;
        }
    }

    fn fewest_presses_joltages(&self) -> usize {
        let len: usize = self.joltages.len();

        0
    }
}

impl FromStr for Machine {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_lights(s: &str) -> IResult<&str, usize> {
            let (s, v) =
                delimited(char('['), many1(alt((char('.'), char('#')))), char(']')).parse(s)?;
            let mut target: usize = 0;
            for c in v.into_iter().rev() {
                target *= 2;
                if c == '#' {
                    target += 1;
                }
            }
            Ok((s, target))
        }

        fn parse_button(s: &str) -> IResult<&str, usize> {
            let (s, toggles) = delimited(
                char('('),
                separated_list1(char(','), parse_usize),
                char(')'),
            )
            .parse(s)?;
            let button = toggles.iter().map(|&n| 2usize.pow(n as u32)).sum();
            Ok((s, button))
        }

        fn parse_joltages(s: &str) -> IResult<&str, Vec<usize>> {
            let (s, joltages) = delimited(
                char('{'),
                separated_list1(char(','), parse_usize),
                char('}'),
            )
            .parse(s)?;
            Ok((s, joltages))
        }

        fn parse_machine(s: &str) -> IResult<&str, Machine> {
            let (s, target) = parse_lights.parse(s)?;
            let (s, buttons) =
                preceded(char(' '), separated_list1(char(' '), parse_button)).parse(s)?;
            let (s, joltages) = preceded(char(' '), parse_joltages).parse(s)?;
            Ok((
                s,
                Machine {
                    target,
                    buttons,
                    joltages,
                },
            ))
        }

        Ok(parse_machine(s).unwrap().1)
    }
}

#[derive(Debug)]
struct Factory {
    machines: Vec<Machine>,
}

impl Factory {
    fn fewest_presses_lights(&self) -> usize {
        self.machines
            .iter()
            .map(|m| m.fewest_presses_lights())
            .sum()
    }

    fn fewest_presses_joltages(&self) -> usize {
        self.machines
            .iter()
            .map(|m| m.fewest_presses_joltages())
            .sum()
    }
}

impl FromStr for Factory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let machines: Vec<Machine> = s.lines().map(|l| l.parse().unwrap()).collect();

        Ok(Factory { machines })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_10.txt").expect("Cannot open input file");
    let factory: Factory = s.parse().unwrap();
    println!("Part1: {}", factory.fewest_presses_lights());
    println!("Part2: {}", 0);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}
";
    #[test]
    fn test_part_1() {
        let factory: Factory = EXAMPLE_1.parse().unwrap();
        assert_eq!(factory.fewest_presses_lights(), 7);
    }

    #[test]
    fn test_part_2() {
        let factory: Factory = EXAMPLE_1.parse().unwrap();
        assert_eq!(factory.fewest_presses_joltages(), 33);
    }
}
