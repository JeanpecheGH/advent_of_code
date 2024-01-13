use std::str::FromStr;

struct Polymer {
    formula: String,
}

impl Polymer {
    fn reduction(&self) -> (usize, usize) {
        let v = Self::reaction(self.formula.chars().map(|c| c as u8));

        let min = (b'a'..=b'z')
            .map(|r| Self::reaction(v.iter().copied().filter(|&c| c | 32 != r)).len())
            .min()
            .unwrap();

        (v.len(), min)
    }
    fn reaction(it: impl Iterator<Item = u8>) -> Vec<u8> {
        let mut store: Vec<u8> = Vec::new();

        for r in it {
            match store.last() {
                Some(&l) if l ^ r == 32 => {
                    store.pop();
                }
                _ => store.push(r),
            }
        }
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

    let (reacted, shortest) = polymer.reduction();

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
        assert_eq!(polymer.reduction(), (10, 4));
    }
}
