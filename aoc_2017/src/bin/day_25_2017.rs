use fxhash::FxHashMap;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char, line_ending};
use nom::sequence::{delimited, terminated};
use nom::IResult;
use std::collections::VecDeque;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Debug, Copy, Clone)]
struct TuringBranch {
    value: bool,
    move_right: bool,
    next_state: char,
}

#[derive(Debug, Copy, Clone)]
struct TuringState {
    name: char,
    false_branch: TuringBranch,
    true_branch: TuringBranch,
}

impl FromStr for TuringState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_name(s: &str) -> IResult<&str, char> {
            delimited(tag("In state "), anychar, char(':'))(s)
        }

        fn parse_value(s: &str) -> IResult<&str, bool> {
            let (s, value) = delimited(tag("    - Write the value "), parse_usize, char('.'))(s)?;
            if value == 1 {
                Ok((s, true))
            } else {
                Ok((s, false))
            }
        }

        fn parse_move(s: &str) -> IResult<&str, bool> {
            let (s, value) = delimited(
                tag("    - Move one slot to the "),
                alt((tag("right"), tag("left"))),
                char('.'),
            )(s)?;
            if value == "right" {
                Ok((s, true))
            } else {
                Ok((s, false))
            }
        }

        fn parse_next_state(s: &str) -> IResult<&str, char> {
            delimited(tag("    - Continue with state "), anychar, char('.'))(s)
        }

        let lines: Vec<&str> = s.lines().collect();
        let name: char = parse_name(lines[0]).unwrap().1;
        let value: bool = parse_value(lines[2]).unwrap().1;
        let move_right: bool = parse_move(lines[3]).unwrap().1;
        let next_state: char = parse_next_state(lines[4]).unwrap().1;
        let false_branch = TuringBranch {
            value,
            move_right,
            next_state,
        };
        let value: bool = parse_value(lines[6]).unwrap().1;
        let move_right: bool = parse_move(lines[7]).unwrap().1;
        let next_state: char = parse_next_state(lines[8]).unwrap().1;
        let true_branch = TuringBranch {
            value,
            move_right,
            next_state,
        };

        Ok(TuringState {
            name,
            false_branch,
            true_branch,
        })
    }
}

#[derive(Debug, Clone)]
struct TuringMachine {
    current_state: char,
    nb_steps: usize,
    states: FxHashMap<char, TuringState>,
}

impl TuringMachine {
    fn diagnostic_checksum(&mut self) -> usize {
        let mut tape: VecDeque<bool> = VecDeque::new();
        tape.push_back(false);

        let mut index: usize = 0;

        for _ in 0..self.nb_steps {
            let b = tape[index];
            let state = self.states.get(&self.current_state).copied().unwrap();
            let branch = if b {
                state.true_branch
            } else {
                state.false_branch
            };

            tape[index] = branch.value;
            if branch.move_right {
                index += 1;
                if index >= tape.len() {
                    tape.push_back(false);
                }
            } else if index == 0 {
                tape.push_front(false);
            } else {
                index -= 1;
            }
            self.current_state = branch.next_state;
        }

        tape.iter().filter(|&&b| b).count()
    }
}

impl FromStr for TuringMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_start(s: &str) -> IResult<&str, (char, usize)> {
            let (s, name) = delimited(
                tag("Begin in state "),
                anychar,
                terminated(char('.'), line_ending),
            )(s)?;
            let (s, nb_steps) = delimited(
                tag("Perform a diagnostic checksum after "),
                parse_usize,
                tag(" steps."),
            )(s)?;
            Ok((s, (name, nb_steps)))
        }

        let blocks: Vec<&str> = split_blocks(s);
        let mut iter = blocks.into_iter();
        let (current_state, nb_steps) = parse_start(iter.next().unwrap()).unwrap().1;
        let states: FxHashMap<char, TuringState> = iter
            .map(|block| {
                let state: TuringState = block.parse().unwrap();
                (state.name, state)
            })
            .collect();

        Ok(TuringMachine {
            current_state,
            nb_steps,
            states,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_25.txt").expect("Cannot open input file");
    let mut machine: TuringMachine = s.parse().unwrap();

    println!(
        "Part1: The diagnostic checksum is {}",
        machine.diagnostic_checksum()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "Begin in state A.
Perform a diagnostic checksum after 6 steps.

In state A:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state B.
  If the current value is 1:
    - Write the value 0.
    - Move one slot to the left.
    - Continue with state B.

In state B:
  If the current value is 0:
    - Write the value 1.
    - Move one slot to the left.
    - Continue with state A.
  If the current value is 1:
    - Write the value 1.
    - Move one slot to the right.
    - Continue with state A.
";
    #[test]
    fn part_1() {
        let mut machine: TuringMachine = EXAMPLE_1.parse().unwrap();
        assert_eq!(3, machine.diagnostic_checksum());
    }
}
