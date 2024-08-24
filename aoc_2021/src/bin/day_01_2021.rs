use std::str::FromStr;

#[derive(Debug, Clone)]
struct Sonar {
    measurements: Vec<usize>,
}

impl Sonar {
    fn nb_increases(&self) -> usize {
        self.measurements
            .windows(2)
            .filter(|pair| pair[0] < pair[1])
            .count()
    }
    fn nb_increases_window(&self) -> usize {
        let means: Vec<usize> = self
            .measurements
            .windows(3)
            .map(|triplet| triplet.iter().sum::<usize>())
            .collect();

        means.windows(2).filter(|pair| pair[0] < pair[1]).count()
    }
}

impl FromStr for Sonar {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let measurements: Vec<usize> = s.lines().map(|s| s.parse().unwrap()).collect();
        Ok(Sonar { measurements })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_01.txt").expect("Cannot open input file");
    let sonar: Sonar = s.parse().unwrap();

    println!(
        "Part1: {} measures are higher than the one before",
        sonar.nb_increases()
    );
    println!("Part2: When lumping measures by sliding windows of 3, {} measures are higher than the one before", sonar.nb_increases_window());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "199
200
208
210
200
207
240
269
260
263
";

    #[test]
    fn part_1() {
        let sonar: Sonar = EXAMPLE_1.parse().unwrap();
        assert_eq!(7, sonar.nb_increases());
    }

    #[test]
    fn part_2() {
        let sonar: Sonar = EXAMPLE_1.parse().unwrap();
        assert_eq!(5, sonar.nb_increases_window());
    }
}
