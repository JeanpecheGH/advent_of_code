use std::str::FromStr;
use util::basic_parser::parse_isize;

#[derive(Debug, Clone)]
struct Trampoline {
    instructions: Vec<isize>,
}

impl Trampoline {
    fn nb_steps_outside(&self, with_deacrease: bool) -> usize {
        let mut instrs = self.instructions.clone();

        let mut ptr: isize = 0;
        let mut steps: usize = 0;
        while ptr < instrs.len() as isize {
            let old_ptr: isize = ptr;
            ptr += instrs[old_ptr as usize];
            if with_deacrease && ptr >= (old_ptr + 3) {
                instrs[old_ptr as usize] -= 1;
            } else {
                instrs[old_ptr as usize] += 1;
            }
            steps += 1;
        }
        steps
    }
}

impl FromStr for Trampoline {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let instructions: Vec<isize> = s.lines().map(|l| parse_isize(l).unwrap().1).collect();
        Ok(Trampoline { instructions })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_05.txt").expect("Cannot open input file");
    let trampoline: Trampoline = s.parse().unwrap();

    println!(
        "Part1: The program exits in {} steps",
        trampoline.nb_steps_outside(false)
    );
    println!(
        "Part2: The program now exits in {} steps",
        trampoline.nb_steps_outside(true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0
3
0
1
-3
";

    #[test]
    fn part_1() {
        let mut trampoline: Trampoline = EXAMPLE_1.parse().unwrap();
        assert_eq!(5, trampoline.nb_steps_outside(false));
    }

    #[test]
    fn part_2() {
        let mut trampoline: Trampoline = EXAMPLE_1.parse().unwrap();
        assert_eq!(10, trampoline.nb_steps_outside(true));
    }
}
