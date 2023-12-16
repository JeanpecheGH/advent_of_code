use rayon::iter::{IndexedParallelIterator, IntoParallelIterator, ParallelIterator};
use std::str::FromStr;

enum LensOperation {
    Add(String, usize),
    Remove(String),
}

impl FromStr for LensOperation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(label) = s.strip_suffix('-') {
            Ok(LensOperation::Remove(label.to_string()))
        } else {
            let (label, power) = s.split_once('=').unwrap();
            let power: usize = power.parse().unwrap();
            Ok(LensOperation::Add(label.to_string(), power))
        }
    }
}

struct Library {
    ops: Vec<LensOperation>,
}

impl Library {
    fn focusing_power(&self) -> usize {
        let mut boxes: Vec<Vec<(String, usize)>> = vec![Vec::new(); 256];
        for op in self.ops.iter() {
            Self::apply_op(op, &mut boxes);
        }

        let power: usize = boxes
            .into_par_iter()
            .enumerate()
            .map(|(i, b)| {
                b.into_iter()
                    .enumerate()
                    .map(|(j, (_, p))| (i + 1) * (j + 1) * p)
                    .sum::<usize>()
            })
            .sum();
        power
    }

    fn apply_op(op: &LensOperation, boxes: &mut [Vec<(String, usize)>]) {
        match op {
            LensOperation::Add(lbl, pow) => {
                let h: usize = Hasher::hash(lbl);
                if let Some(pos) = boxes[h].iter().position(|(l, _)| lbl == l) {
                    boxes[h][pos] = (lbl.to_string(), *pow);
                } else {
                    boxes[h].push((lbl.to_string(), *pow));
                }
            }
            LensOperation::Remove(lbl) => {
                let h: usize = Hasher::hash(lbl);
                boxes[h].retain(|(l, _)| lbl != l);
            }
        }
    }
}

impl FromStr for Library {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ops: Vec<LensOperation> = s
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.parse::<LensOperation>().unwrap())
            .collect();
        Ok(Library { ops })
    }
}

struct Hasher {
    inputs: Vec<String>,
}

impl Hasher {
    fn hash_sum(&self) -> usize {
        self.inputs.iter().map(|i| Self::hash(i)).sum()
    }

    fn hash(i: &str) -> usize {
        let mut h: usize = 0;

        for c in i.chars() {
            h = ((h + c as usize) * 17) % 256;
        }
        h
    }
}

impl FromStr for Hasher {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let inputs: Vec<String> = s
            .lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.to_string())
            .collect();
        Ok(Hasher { inputs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_15.txt").expect("Cannot open input file");
    let hasher: Hasher = s.parse().unwrap();
    println!("Part1: The sum of the Hashes is {}", hasher.hash_sum());
    let library: Library = s.parse().unwrap();
    println!(
        "Part2: The total focusing power of the lenses is {}",
        library.focusing_power()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_1() {
        let hasher: Hasher = EXAMPLE_1.parse().unwrap();
        assert_eq!(hasher.hash_sum(), 1320);
    }
    #[test]
    fn part_2() {
        let library: Library = EXAMPLE_1.parse().unwrap();
        assert_eq!(library.focusing_power(), 145);
    }
}
