use std::collections::VecDeque;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Captcha {
    numbers: VecDeque<u32>,
}

impl Captcha {
    fn sum(&mut self) -> u32 {
        let mut sum: u32 = 0;
        for _ in 0..self.numbers.len() {
            if self.numbers.front().unwrap() == self.numbers.back().unwrap() {
                sum += self.numbers.front().unwrap();
            }
            self.numbers.rotate_right(1);
        }
        sum
    }

    fn halfway_sum(&mut self) -> u32 {
        let len = self.numbers.len();

        let mut sum: u32 = 0;
        for _ in 0..len {
            if self.numbers[0] == self.numbers[len / 2] {
                sum += self.numbers[0];
            }
            self.numbers.rotate_right(1);
        }
        sum
    }
}

impl FromStr for Captcha {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: VecDeque<u32> = s
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        Ok(Captcha { numbers })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_01.txt").expect("Cannot open input file");
    let mut captcha: Captcha = s.parse().unwrap();

    println!(
        "Part1: The captcha sum when checking the next digit is {}",
        captcha.sum()
    );
    println!(
        "Part2: When checking the digit halfway around, the catcha sum is now {}",
        captcha.halfway_sum()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "1122";
    const EXAMPLE_2: &str = "1111";
    const EXAMPLE_3: &str = "1234";
    const EXAMPLE_4: &str = "91212129";
    const EXAMPLE_5: &str = "1212";
    const EXAMPLE_6: &str = "1221";
    const EXAMPLE_7: &str = "123425";
    const EXAMPLE_8: &str = "123123";
    const EXAMPLE_9: &str = "12131415";

    #[test]
    fn part_1_test_1() {
        let mut captcha: Captcha = EXAMPLE_1.parse().unwrap();
        assert_eq!(3, captcha.sum());
    }
    #[test]
    fn part_1_test_2() {
        let mut captcha: Captcha = EXAMPLE_2.parse().unwrap();
        assert_eq!(4, captcha.sum());
    }
    #[test]
    fn part_1_test_3() {
        let mut captcha: Captcha = EXAMPLE_3.parse().unwrap();
        assert_eq!(0, captcha.sum());
    }
    #[test]
    fn part_1_test_4() {
        let mut captcha: Captcha = EXAMPLE_4.parse().unwrap();
        assert_eq!(9, captcha.sum());
    }
    #[test]
    fn part_2_test_1() {
        let mut captcha: Captcha = EXAMPLE_5.parse().unwrap();
        assert_eq!(6, captcha.halfway_sum());
    }
    #[test]
    fn part_2_test_2() {
        let mut captcha: Captcha = EXAMPLE_6.parse().unwrap();
        assert_eq!(0, captcha.halfway_sum());
    }
    #[test]
    fn part_2_test_3() {
        let mut captcha: Captcha = EXAMPLE_7.parse().unwrap();
        assert_eq!(4, captcha.halfway_sum());
    }
    #[test]
    fn part_2_test_4() {
        let mut captcha: Captcha = EXAMPLE_8.parse().unwrap();
        assert_eq!(12, captcha.halfway_sum());
    }
    #[test]
    fn part_2_test_5() {
        let mut captcha: Captcha = EXAMPLE_9.parse().unwrap();
        assert_eq!(4, captcha.halfway_sum());
    }
}
