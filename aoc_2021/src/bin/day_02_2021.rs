use nom::character::complete::{alpha1, space1};
use nom::sequence::separated_pair;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Copy, Clone)]
enum DiveCommand {
    Forward(usize),
    Down(usize),
    Up(usize),
}

#[derive(Debug, Clone)]
struct SubmarineDive {
    commands: Vec<DiveCommand>,
}

impl SubmarineDive {
    fn simple_dive(&self) -> usize {
        let (depth, advance): (usize, usize) =
            self.commands
                .iter()
                .fold((0, 0), |(mut depth, mut advance), com| {
                    match com {
                        DiveCommand::Forward(n) => advance += *n,
                        DiveCommand::Down(n) => depth += *n,
                        DiveCommand::Up(n) => depth -= *n,
                    }
                    (depth, advance)
                });

        depth * advance
    }
    fn oriented_dive(&self) -> usize {
        let (depth, advance, _aim): (usize, usize, usize) =
            self.commands
                .iter()
                .fold((0, 0, 0), |(mut depth, mut advance, mut aim), com| {
                    match com {
                        DiveCommand::Forward(n) => {
                            advance += *n;
                            depth += *n * aim;
                        }
                        DiveCommand::Down(n) => aim += *n,
                        DiveCommand::Up(n) => aim -= *n,
                    }
                    (depth, advance, aim)
                });

        depth * advance
    }
}

impl FromStr for SubmarineDive {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_command(s: &str) -> IResult<&str, DiveCommand> {
            let (s, (name, factor)) = separated_pair(alpha1, space1, parse_usize)(s)?;
            let command: DiveCommand = match name {
                "down" => DiveCommand::Down(factor),
                "up" => DiveCommand::Up(factor),
                _ => DiveCommand::Forward(factor),
            };

            Ok((s, command))
        }

        let commands: Vec<DiveCommand> = s.lines().map(|s| parse_command(s).unwrap().1).collect();
        Ok(SubmarineDive { commands })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_02.txt").expect("Cannot open input file");
    let dive: SubmarineDive = s.parse().unwrap();

    println!(
        "Part1: Multiplying the depth with the horizontal position gives {}",
        dive.simple_dive()
    );
    println!("Part2: When using the right set of commands, multiplying the depth with the horizontal position gives {}", dive.oriented_dive());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "forward 5
down 5
forward 8
up 3
down 8
forward 2
";

    #[test]
    fn part_1() {
        let dive: SubmarineDive = EXAMPLE_1.parse().unwrap();
        assert_eq!(150, dive.simple_dive());
    }

    #[test]
    fn part_2() {
        let dive: SubmarineDive = EXAMPLE_1.parse().unwrap();
        assert_eq!(900, dive.oriented_dive());
    }
}
