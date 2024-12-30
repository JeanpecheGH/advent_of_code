use std::str::FromStr;

use util::split_blocks;

struct LocksAndKeys {
    keys: Vec<usize>,
    locks: Vec<usize>,
}
impl LocksAndKeys {
    fn nb_fit(&self) -> usize {
        self.keys
            .iter()
            .map(|&key| self.locks.iter().filter(|&&lock| key & lock == 0).count())
            .sum()
    }
}

impl FromStr for LocksAndKeys {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = split_blocks(s);
        let (keys, locks): (Vec<usize>, Vec<usize>) =
            blocks
                .iter()
                .fold((Vec::new(), Vec::new()), |(mut keys, mut pins), block| {
                    let mut it = block.as_bytes().iter();
                    let key = it.next().unwrap() == &b'#';
                    let value = it
                        .skip(5) // Skip the first line
                        .take(35) // Take the 5 next lines including CRs & LFs
                        .fold(0, |acc, &c| acc * 2 + (c == b'#') as usize);
                    if key {
                        keys.push(value);
                    } else {
                        pins.push(value);
                    }
                    (keys, pins)
                });

        Ok(LocksAndKeys { keys, locks })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_25.txt").expect("Cannot open input file");
    let locks_and_keys: LocksAndKeys = s.parse().unwrap();
    println!(
        "Part1: {} key/lock pairs fit together",
        locks_and_keys.nb_fit()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####
";

    #[test]
    fn part_1_test_1() {
        let locks_and_keys: LocksAndKeys = EXAMPLE_1.parse().unwrap();
        assert_eq!(locks_and_keys.nb_fit(), 3);
    }
}
