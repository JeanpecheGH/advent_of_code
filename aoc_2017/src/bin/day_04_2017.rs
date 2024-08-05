use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Passphrases {
    words: Vec<Vec<String>>,
}

impl Passphrases {
    fn nb_valid(&self) -> usize {
        self.words
            .iter()
            .filter(|row| row.iter().all_unique())
            .count()
    }

    fn nb_valid_anagram(&self) -> usize {
        self.words
            .iter()
            .filter(|row| {
                row.iter()
                    .map(|w| w.chars().sorted().collect::<Vec<char>>())
                    .all_unique()
            })
            .count()
    }
}

impl FromStr for Passphrases {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<Vec<String>> = s
            .lines()
            .map(|l| l.split(' ').map(|w| w.to_string()).collect::<Vec<String>>())
            .collect();
        Ok(Passphrases { words })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_04.txt").expect("Cannot open input file");
    let pass: Passphrases = s.parse().unwrap();

    println!("Part1: {} passphrases are valid", pass.nb_valid());
    println!(
        "Part2: When forbiding anagrams, only {} passphrases are now valid ",
        pass.nb_valid_anagram()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "aa bb cc dd ee
aa bb cc dd aa
aa bb cc dd aaa";

    const EXAMPLE_2: &str = "abcde fghij
abcde xyz ecdab
a ab abc abd abf abj
iiii oiii ooii oooi oooo
oiii ioii iioi iiio";

    #[test]
    fn part_1() {
        let mut pass: Passphrases = EXAMPLE_1.parse().unwrap();
        assert_eq!(2, pass.nb_valid());
    }

    #[test]
    fn part_2() {
        let mut pass: Passphrases = EXAMPLE_2.parse().unwrap();
        assert_eq!(3, pass.nb_valid_anagram());
    }
}
