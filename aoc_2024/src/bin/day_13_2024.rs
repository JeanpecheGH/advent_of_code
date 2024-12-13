use nom::bytes::complete::tag;
use nom::character::complete::one_of;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::{parse_usize, title};
use util::coord::Pos;
use util::split_blocks;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct ClawMachine {
    a: Pos,
    b: Pos,
    prize: Pos,
}

impl ClawMachine {
    fn fewest_tokens(&self, offset: usize) -> Option<usize> {
        // Solve  the following system
        // a*xa + b*xb = offset + xprize
        // a*ya + b*yb = offset + yprize
        // Using https://en.wikipedia.org/wiki/Cramer%27s_rule
        let Pos(xa, ya) = self.a;
        let Pos(xb, yb) = self.b;
        let xp = self.prize.0 + offset;
        let yp = self.prize.1 + offset;
        let div: isize = (xa * yb) as isize - (xb * ya) as isize;
        if div == 0 {
            return None;
        }
        let up_a: isize = (xp * yb) as isize - (xb * yp) as isize;
        let up_b: isize = (xa * yp) as isize - (xp * ya) as isize;

        //We check that our solutions are integers
        if up_a % div == 0 && up_b % div == 0 {
            Some((up_a / div) as usize * 3 + (up_b / div) as usize)
        } else {
            None
        }
    }
}

impl FromStr for ClawMachine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> IResult<&str, Pos> {
            let (s, _) = title(s)?;
            let (s, x) = preceded(preceded(tag("X"), one_of("+=")), parse_usize)(s)?;
            let (s, y) = preceded(preceded(tag(", Y"), one_of("+=")), parse_usize)(s)?;

            Ok((s, Pos(x, y)))
        }

        let lines: Vec<&str> = s.lines().collect();
        let a: Pos = parse_pos(lines[0]).unwrap().1;
        let b: Pos = parse_pos(lines[1]).unwrap().1;
        let prize: Pos = parse_pos(lines[2]).unwrap().1;

        Ok(ClawMachine { a, b, prize })
    }
}

struct Arcade {
    machines: Vec<ClawMachine>,
}
impl Arcade {
    fn fewest_tokens(&self, offset: usize) -> usize {
        self.machines
            .iter()
            .flat_map(|m| m.fewest_tokens(offset))
            .sum()
    }
}

impl FromStr for Arcade {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = split_blocks(s);
        let machines: Vec<ClawMachine> = blocks.iter().map(|l| l.parse().unwrap()).collect();
        Ok(Arcade { machines })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_13.txt").expect("Cannot open input file");
    let arcade: Arcade = s.parse().unwrap();
    println!(
        "Part1: We need {} tokens to win all the possible prizes",
        arcade.fewest_tokens(0)
    );
    println!(
        "Part2: We need {} tokens to win all the possible prizes",
        arcade.fewest_tokens(10000000000000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279
";

    #[test]
    fn part_1() {
        let arcade: Arcade = EXAMPLE_1.parse().unwrap();
        assert_eq!(arcade.fewest_tokens(0), 480);
    }
}
