use itertools::Itertools;
use util::intcode::IntCode;

struct Amplifier {
    codes: Vec<IntCode>,
}

impl Amplifier {
    fn from_code(code: &IntCode) -> Self {
        Self {
            codes: vec![code.clone(); 5],
        }
    }

    fn reset(&mut self) {
        for c in self.codes.iter_mut() {
            c.reset();
        }
    }

    fn amplify(&mut self, input: isize, phases: &[isize], reset: bool) -> Option<isize> {
        let mut value: isize = input;
        for (i, code) in self.codes.iter_mut().enumerate() {
            let mut inputs: Vec<isize> = if phases.is_empty() {
                vec![value]
            } else {
                vec![phases[i], value]
            };
            code.compute(&mut inputs);
            value = code.output.pop()?;
        }
        if reset {
            self.reset();
        }
        Some(value)
    }

    fn highest_signal(&mut self, input: isize) -> isize {
        (0..=4)
            .permutations(5)
            .map(|phases| self.amplify(input, &phases, true).unwrap())
            .max()
            .unwrap()
    }

    fn feedback_loop(&mut self, input: isize) -> isize {
        (5..=9)
            .permutations(5)
            .map(|phases| {
                let mut value = input;
                value = self.amplify(value, &phases, false).unwrap();
                while let Some(v) = self.amplify(value, &[], false) {
                    value = v;
                }
                self.reset();
                value
            })
            .max()
            .unwrap()
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_07.txt").expect("Cannot open input file");
    let code: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut amp: Amplifier = Amplifier::from_code(&code);
    println!(
        "Part1: The highest possible signal is {}",
        amp.highest_signal(0)
    );
    println!(
        "Part2: The highest possible signal with a feedback loop is {}",
        amp.feedback_loop(0)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_part_1() {
        let code: IntCode = "3,15,3,16,1002,16,10,16,1,16,15,15,4,15,99,0,0"
            .parse()
            .unwrap();
        let mut amp: Amplifier = Amplifier::from_code(&code);

        assert_eq!(amp.highest_signal(0), 43210);
    }

    #[test]
    fn test_2_part_1() {
        let code: IntCode =
            "3,23,3,24,1002,24,10,24,1002,23,-1,23,101,5,23,23,1,24,23,23,4,23,99,0,0"
                .parse()
                .unwrap();
        let mut amp: Amplifier = Amplifier::from_code(&code);

        assert_eq!(amp.highest_signal(0), 54321);
    }

    #[test]
    fn test_3_part_1() {
        let code: IntCode = "3,31,3,32,1002,32,10,32,1001,31,-2,31,1007,31,0,33,1002,33,7,33,1,33,31,31,1,32,31,31,4,31,99,0,0,0"
            .parse()
            .unwrap();
        let mut amp: Amplifier = Amplifier::from_code(&code);

        assert_eq!(amp.highest_signal(0), 65210);
    }

    #[test]
    fn test_1_part_2() {
        let code: IntCode =
            "3,26,1001,26,-4,26,3,27,1002,27,2,27,1,27,26,27,4,27,1001,28,-1,28,1005,28,6,99,0,0,5"
                .parse()
                .unwrap();
        let mut amp: Amplifier = Amplifier::from_code(&code);

        assert_eq!(amp.feedback_loop(0), 139629729);
    }

    #[test]
    fn test_2_part_2() {
        let code: IntCode =
            "3,52,1001,52,-5,52,3,53,1,52,56,54,1007,54,5,55,1005,55,26,1001,54,-5,54,1105,1,12,1,53,54,53,1008,54,0,55,1001,55,1,55,2,53,55,53,4,53,1001,56,-1,56,1005,56,6,99,0,0,0,0,10"
                .parse()
                .unwrap();
        let mut amp: Amplifier = Amplifier::from_code(&code);

        assert_eq!(amp.feedback_loop(0), 18216);
    }
}
