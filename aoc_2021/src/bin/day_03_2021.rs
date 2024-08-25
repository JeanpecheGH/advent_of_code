use std::str::FromStr;

#[derive(Debug, Clone)]
struct BinaryDiagnostic {
    reports: Vec<Vec<bool>>,
}

impl BinaryDiagnostic {
    fn binary_to_int(v: Vec<bool>) -> usize {
        v.iter()
            .fold(0, |acc, &b| if b { (acc * 2) + 1 } else { acc * 2 })
    }
    fn power_consumption(&self) -> usize {
        let report_size: usize = self.reports.len();
        let gamma_rate: Vec<bool> = self
            .reports
            .iter()
            .fold(vec![0; self.reports[0].len()], |mut sums, rep| {
                for (n, b) in rep.iter().enumerate() {
                    if *b {
                        sums[n] += 1;
                    }
                }
                sums
            })
            .iter()
            .map(|&sum| sum > report_size / 2)
            .collect();

        let epsilon_rate: Vec<bool> = gamma_rate.iter().map(|&b| !b).collect();

        BinaryDiagnostic::binary_to_int(gamma_rate) * BinaryDiagnostic::binary_to_int(epsilon_rate)
    }

    fn bit_criteria(&self, majority: bool) -> Vec<bool> {
        fn chosen_bit(reports: &[Vec<bool>], index: usize, majority: bool) -> bool {
            let report_size: usize = reports.len();
            let nb_true = reports.iter().filter(|r| r[index]).count();
            ((nb_true * 2) >= report_size) == majority
        }
        let mut filtered_reports: Vec<Vec<bool>> = self.reports.clone();
        let mut i: usize = 0;
        while filtered_reports.len() > 1 {
            //Get the more/least common bit on index i
            let b: bool = chosen_bit(&filtered_reports, i, majority);
            //Filter

            filtered_reports.retain(|r| r[i] == b);
            i += 1;
        }
        filtered_reports[0].clone()
    }
    fn life_support_rating(&self) -> usize {
        let oxygen_rating: Vec<bool> = self.bit_criteria(true);
        let co2_rating: Vec<bool> = self.bit_criteria(false);

        BinaryDiagnostic::binary_to_int(oxygen_rating) * BinaryDiagnostic::binary_to_int(co2_rating)
    }
}

impl FromStr for BinaryDiagnostic {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reports: Vec<Vec<bool>> = s
            .lines()
            .map(|s| s.chars().map(|c| c == '1').collect())
            .collect();
        Ok(BinaryDiagnostic { reports })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_03.txt").expect("Cannot open input file");
    let diag: BinaryDiagnostic = s.parse().unwrap();

    println!(
        "Part1: The power consumption of the submarine is {}",
        diag.power_consumption()
    );
    println!(
        "Part2: The life support rating of the submarine is {}",
        diag.life_support_rating()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
";

    #[test]
    fn part_1() {
        let diag: BinaryDiagnostic = EXAMPLE_1.parse().unwrap();
        assert_eq!(198, diag.power_consumption());
    }

    #[test]
    fn part_2() {
        let diag: BinaryDiagnostic = EXAMPLE_1.parse().unwrap();
        assert_eq!(230, diag.life_support_rating());
    }
}
