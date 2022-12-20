use std::str::FromStr;

const DECRYPTION_KEY: isize = 811589153;
const TARGETS: &[usize] = &[1000, 2000, 3000];
const MIX_TIMES: usize = 10;

#[derive(Debug)]
struct Numbers {
    indexed: Vec<(usize, isize)>,
}

impl Numbers {
    fn new(list: Vec<isize>) -> Self {
        let indexed: Vec<(usize, isize)> = list.into_iter().enumerate().collect();

        Numbers { indexed }
    }

    fn sum_at(&self, positions: &[usize]) -> isize {
        let len = self.indexed.len();
        let zero_pos: usize = self.indexed.iter().position(|(_, n)| *n == 0).unwrap();
        positions
            .iter()
            .map(|n| self.indexed[(zero_pos + *n) % len].1)
            .sum()
    }

    fn decrypt(&mut self, key: isize) {
        self.indexed = self.indexed.iter().map(|(i, n)| (*i, *n * key)).collect()
    }

    fn solve(&mut self, times: usize, key: isize) {
        self.decrypt(key);
        for _ in 0..times {
            self.mix();
        }
    }

    fn mix(&mut self) {
        let len: usize = self.indexed.len();
        for i in 0..len {
            let old_idx = self.indexed.iter().position(|(idx, _)| *idx == i).unwrap();
            let elem = self.indexed.remove(old_idx);
            let new_idx = ((((old_idx as isize + elem.1) % (len - 1) as isize)
                + (len - 1) as isize)
                % (len - 1) as isize) as usize;
            self.indexed.insert(new_idx, elem);
        }
    }

    #[allow(dead_code)]
    fn print(&self) {
        for &elem in self.indexed.iter() {
            print!("{:?}, ", elem);
        }
        println!();
    }
}

impl FromStr for Numbers {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let list: Vec<isize> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Numbers::new(list))
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_20.txt").expect("Cannot open input file");

    let mut nbs: Numbers = s.parse().unwrap();
    nbs.solve(1, 1);
    println!(
        "Part1: The sum of the three coordinates is {}",
        nbs.sum_at(TARGETS)
    );

    let mut nbs_2: Numbers = s.parse().unwrap();
    nbs_2.solve(MIX_TIMES, DECRYPTION_KEY);
    println!(
        "Part2: After decryption, the sum of the three coordinates is now {}",
        nbs_2.sum_at(TARGETS)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1
2
-3
3
-2
0
4";

    #[test]
    fn part_1() {
        let mut nbs: Numbers = INPUT.parse().unwrap();
        nbs.solve(1, 1);
        let sum = nbs.sum_at(TARGETS);
        assert_eq!(sum, 3);
    }

    #[test]
    fn part_2() {
        let mut nbs: Numbers = INPUT.parse().unwrap();
        nbs.solve(MIX_TIMES, DECRYPTION_KEY);
        let sum = nbs.sum_at(TARGETS);
        assert_eq!(sum, 1623178306);
    }
}
