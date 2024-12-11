use fxhash::{FxBuildHasher, FxHashMap};
use std::str::FromStr;
use util::basic_parser::usize_list;

struct PlutonianStones {
    stones: Vec<usize>,
}
impl PlutonianStones {
    fn blink(&self, times: usize) -> usize {
        let mut moving_stones: FxHashMap<usize, usize> =
            FxHashMap::with_capacity_and_hasher(self.stones.len(), FxBuildHasher::default());
        for &v in self.stones.iter() {
            *moving_stones.entry(v).or_default() += 1;
        }

        for _ in 0..times {
            let mut new_stones: FxHashMap<usize, usize> = FxHashMap::with_capacity_and_hasher(
                moving_stones.len() * 3 / 2,
                FxBuildHasher::default(),
            );
            for (k, v) in moving_stones {
                match k {
                    0 => *new_stones.entry(1).or_default() += v,
                    n if (n.ilog10() + 1) % 2 == 0 => {
                        let div: usize = 10usize.pow((n.ilog10() + 1) / 2);
                        *new_stones.entry(n / div).or_default() += v;
                        *new_stones.entry(n % div).or_default() += v;
                    }
                    n => *new_stones.entry(n * 2024).or_default() += v,
                }
            }
            moving_stones = new_stones;
        }

        moving_stones.values().sum()
    }
}

impl FromStr for PlutonianStones {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let start_line: &str = s.lines().next().unwrap();
        let stones: Vec<usize> = usize_list(start_line).unwrap().1;
        Ok(PlutonianStones { stones })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_11.txt").expect("Cannot open input file");
    let stones: PlutonianStones = s.parse().unwrap();
    println!(
        "Part1: After blinking 25 times, there are {} stones",
        stones.blink(25)
    );
    println!(
        "Part2: After blinking 75 times, there are {} stones",
        stones.blink(75)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "125 17";

    #[test]
    fn part_1_test_1() {
        let stones: PlutonianStones = EXAMPLE_1.parse().unwrap();
        assert_eq!(stones.blink(6), 22);
    }

    #[test]
    fn part_1_test_2() {
        let stones: PlutonianStones = EXAMPLE_1.parse().unwrap();
        assert_eq!(stones.blink(25), 55312);
    }
}
