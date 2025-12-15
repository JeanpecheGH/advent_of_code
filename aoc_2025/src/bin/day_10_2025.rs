use itertools::Itertools;
use nom::IResult;
use nom::Parser;
use nom::branch::alt;
use nom::character::char;
use nom::multi::{many1, separated_list1};
use nom::sequence::{delimited, preceded};
use std::iter::Sum;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use z3::Optimize;
use z3::ast::Int;

#[derive(Debug)]
struct Machine {
    target: usize,
    buttons: Vec<usize>,
    joltages: Vec<usize>,
}

impl Machine {
    fn fewest_presses_lights(&self) -> usize {
        // Pressing a button 2 times accomplishes nothing
        // The order in which buttons are pressed is irrelevant
        for i in 1.. {
            for combo in self.buttons.iter().combinations(i) {
                if combo.into_iter().fold(0, |acc, b| acc ^ b) == self.target {
                    return i;
                }
            }
        }
        0
    }

    fn buttons(&self) -> Vec<Vec<bool>> {
        let mut buttons: Vec<Vec<bool>> = Vec::new();
        for &b in &self.buttons {
            let v: Vec<bool> = (0..self.joltages.len())
                .map(|i| (b & 2usize.pow(i as u32)) > 0)
                .collect();

            buttons.push(v);
        }

        buttons
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
        let mut nb_presses: usize = 0;

        let opt = Optimize::new();

        for machine in &self.machines {
            opt.push();

            let joltages: &Vec<usize> = &machine.joltages;
            let buttons: Vec<Vec<bool>> = machine.buttons();

            // Declare buttons presses variables
            let vars: Vec<Int> = (0..buttons.len())
                .map(|i| Int::new_const(format!("x{}", i)))
                // Should be useless, but we need it
                .inspect(|v| opt.assert(&v.ge(0)))
                .collect();

            // Declare constraint on sum of presses
            let count: Int = Int::add(&vars);
            opt.minimize(&count);

            // Declare constraints on target joltages
            for (j, &joltage) in joltages.iter().enumerate() {
                let sum: Int = Int::sum(
                    vars.iter()
                        .enumerate()
                        .filter(|&(i, _)| buttons[i][j])
                        .map(|(_, var)| var),
                );
                opt.assert(&sum.eq(joltage as u16));
            }

            if let z3::SatResult::Sat = opt.check(&[]) {
                let model = opt.get_model().unwrap();
                let count = model.eval(&count, true).unwrap();
                nb_presses += count.as_u64().unwrap_or_default() as usize;
            } else {
                println!("Failed solving z3 for machine {machine:?}");
            }

            opt.pop();
        }

        nb_presses
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
    println!(
        "Part1: The fewest button presses to configure the lights is {}",
        factory.fewest_presses_lights()
    );
    println!(
        "Part2: To configure the joltage levels, we need {} presses",
        factory.fewest_presses_joltages()
    );
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
