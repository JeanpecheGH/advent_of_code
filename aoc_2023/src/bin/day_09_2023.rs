use std::str::FromStr;
use util::basic_parser::isize_list;

struct Sequence {
    values: Vec<isize>,
}

impl Sequence {
    fn extrapolate(&self) -> (isize, isize) {
        if self.values.iter().all(|&n| n == 0) {
            (0, 0)
        } else {
            let (to_remove, to_add): (isize, isize) = self.diff_sequence().extrapolate();
            (
                self.values.first().unwrap() - to_remove,
                self.values.last().unwrap() + to_add,
            )
        }
    }

    fn diff_sequence(&self) -> Sequence {
        let values: Vec<isize> = self
            .values
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .collect();
        Sequence { values }
    }
}

impl FromStr for Sequence {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<isize> = isize_list(s).unwrap().1;
        Ok(Sequence { values })
    }
}

struct Oasis {
    sequences: Vec<Sequence>,
}

impl Oasis {
    fn extrapolated_values(&self) -> (isize, isize) {
        self.sequences
            .iter()
            .map(|s| s.extrapolate())
            .fold((0, 0), |(left, right), (first, last)| {
                ((left + first), (right + last))
            })
    }
}

impl FromStr for Oasis {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let sequences: Vec<Sequence> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Oasis { sequences })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_09.txt").expect("Cannot open input file");
    let oasis: Oasis = s.parse().unwrap();

    let (first, last) = oasis.extrapolated_values();

    println!(
        "Part1: The sum of the extrapolated values to the right is {}",
        last
    );
    println!(
        "Part2: The sum of the extrapolated values to the left is {}",
        first
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

    #[test]
    fn part_1() {
        let oasis: Oasis = EXAMPLE_1.parse().unwrap();
        let (first, last) = oasis.extrapolated_values();
        assert_eq!(first, 2);
        assert_eq!(last, 114);
    }
}
