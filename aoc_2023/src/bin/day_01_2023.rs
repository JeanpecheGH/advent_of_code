use std::str::FromStr;

struct Calibration {
    values: Vec<String>,
}

impl Calibration {
    fn sum_of_values(&self) -> u32 {
        self.values
            .iter()
            .map(|l| {
                let digits: Vec<u32> = l.chars().filter_map(|c| c.to_digit(10)).collect();
                let first: u32 = digits.first().copied().unwrap();
                let last: u32 = digits.last().copied().unwrap();
                first * 10 + last
            })
            .sum()
    }

    fn sum_of_values_with_letters(&self) -> u32 {
        self.values
            .iter()
            .map(|l| {
                let chars: Vec<char> = l.chars().collect();
                let len: usize = chars.len();
                let mut i = 0;
                let mut digits: Vec<u32> = Vec::new();
                while i < len {
                    //Digit can be written with 1 (already a digit), 3, 4 or 5 letters
                    if let Some(d) = chars[i].to_digit(10) {
                        digits.push(d);
                        i += 1;
                    } else {
                        if i + 3 <= len {
                            match &l[i..i + 3] {
                                "one" => digits.push(1),
                                "two" => digits.push(2),
                                "six" => digits.push(6),
                                _ => (),
                            }
                        }

                        if i + 4 <= len {
                            match &l[i..i + 4] {
                                "four" => digits.push(4),
                                "five" => digits.push(5),
                                "nine" => digits.push(9),
                                _ => (),
                            }
                        }

                        if i + 5 <= len {
                            match &l[i..i + 5] {
                                "three" => digits.push(3),
                                "seven" => digits.push(7),
                                "eight" => digits.push(8),
                                _ => (),
                            }
                        }
                        i += 1;
                    }
                }

                let first: u32 = digits.first().copied().unwrap();
                let last: u32 = digits.last().copied().unwrap();
                first * 10 + last
            })
            .sum()
    }
}

impl FromStr for Calibration {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<String> = s.lines().map(|l| l.to_string()).collect();

        Ok(Calibration { values })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_01.txt").expect("Cannot open input file");
    let cal: Calibration = s.parse().unwrap();

    println!(
        "Part1: The sum of the calibration values is {}",
        cal.sum_of_values()
    );
    println!(
        "Part2: The new sum of the calibration values is {}",
        cal.sum_of_values_with_letters()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";

    const EXAMPLE_2: &str = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";

    #[test]
    fn part_1() {
        let cal: Calibration = EXAMPLE_1.parse().unwrap();
        assert_eq!(cal.sum_of_values(), 142);
    }
    #[test]
    fn part_2() {
        let cal: Calibration = EXAMPLE_2.parse().unwrap();
        assert_eq!(cal.sum_of_values_with_letters(), 281);
    }
}
