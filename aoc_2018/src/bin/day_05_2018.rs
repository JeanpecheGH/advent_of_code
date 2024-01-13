use std::str::FromStr;

struct Polymer {
    formula: String,
}

impl Polymer {
    fn compute(&self) -> (usize, usize) {
        let v = Self::reaction(self.formula.chars(), None);

        let min = ('a'..='z')
            .map(|r| Self::reaction(v.iter().copied(), Some(r)).len())
            .min()
            .unwrap();

        (v.len(), min)
    }
    fn react(a: char, b: char) -> bool {
        a != b && a.to_lowercase().eq(b.to_lowercase())
    }
    fn reaction<I>(mut it: I, remove: Option<char>) -> Vec<char>
    where
        I: Iterator<Item = char>,
    {
        let mut store: Vec<char> = Vec::new();
        let mut left: char = it.next().unwrap();
        while remove.iter().copied().eq(left.to_lowercase()) {
            left = it.next().unwrap();
        }

        while let Some(right) = it.next() {
            if remove.iter().copied().eq(right.to_lowercase()) {
                //Skip this value
            } else if Self::react(left, right) {
                if !store.is_empty() {
                    left = store.pop().unwrap();
                } else {
                    left = it.next().unwrap();
                }
            } else {
                store.push(left);
                left = right;
            }
        }
        store.push(left);
        store
    }
}

impl FromStr for Polymer {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let formula: String = s.lines().next().unwrap().to_string();
        Ok(Polymer { formula })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_05.txt").expect("Cannot open input file");
    let polymer: Polymer = s.parse().unwrap();

    let (reacted, shortest) = polymer.compute();

    println!("Part1: After reaction, the polymer contains {reacted} unit");
    println!("Part2: The shortest polymer obtainable when removing a specific unit type contains {shortest} units");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "dabAcCaCBAcCcaDA";

    #[test]
    fn part_1() {
        let polymer: Polymer = EXAMPLE_1.parse().unwrap();
        assert_eq!(polymer.compute(), (10, 4));
    }
}
