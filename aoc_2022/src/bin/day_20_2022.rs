use std::str::FromStr;

const DECRYPTION_KEY: isize = 811589153;
const TARGETS: &[usize] = &[1000, 2000, 3000];
const MIX_TIMES: usize = 10;

#[derive(Debug)]
struct Numbers {
    list: Vec<isize>,
    indexes: Vec<usize>,
}

impl Numbers {
    fn new(list: Vec<isize>) -> Self {
        let indexes: Vec<usize> = (0..list.len()).collect();

        Numbers { list, indexes }
    }

    fn decrypt(&mut self, key: isize) {
        self.list = self.list.iter().map(|n| *n * key).collect()
    }

    fn mixing_times(&mut self, n: usize) {
        for _ in 0..n {
            self.mixing();
        }
    }

    fn mixing(&mut self) {
        for i in 0..self.list.len() {
            self.mix(i);
        }
    }

    fn mix(&mut self, i: usize) {
        let idx: usize = self.indexes[i];
        let nb: isize = self.list[i];
        let len: usize = self.list.len() - 1;
        let new_idx = (((idx as isize + nb % len as isize) + len as isize) % len as isize) as usize;

        if new_idx > idx {
            for j in 0..self.list.len() {
                if (idx + 1..=new_idx).contains(&self.indexes[j]) {
                    self.indexes[j] -= 1;
                }
            }
        } else {
            for j in 0..self.list.len() {
                if (new_idx..idx).contains(&self.indexes[j]) {
                    self.indexes[j] += 1;
                }
            }
        }
        self.indexes[i] = new_idx;
    }

    fn zero_pos(&self) -> usize {
        let zero_idx = self.list.iter().position(|n| *n == 0).unwrap();
        self.indexes[zero_idx]
    }

    fn n_after_zero(&self, n: usize) -> isize {
        let zero: usize = self.zero_pos();
        let idx: usize = (zero + n) % self.list.len();
        let pos: usize = self.indexes.iter().position(|i| *i == idx).unwrap();
        self.list[pos]
    }

    fn sum_at(&self, positions: &[usize]) -> isize {
        positions.iter().map(|n| self.n_after_zero(*n)).sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for i in 0..self.list.len() {
            let pos = self.indexes.iter().position(|idx| *idx == i).unwrap();
            print!("{}, ", self.list[pos]);
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
    nbs.mixing();
    println!("Part1: {}", nbs.sum_at(TARGETS));

    let mut nbs_2: Numbers = s.parse().unwrap();
    nbs_2.decrypt(DECRYPTION_KEY);
    nbs_2.mixing_times(MIX_TIMES);
    println!("Part2: {}", nbs_2.sum_at(TARGETS));

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
        nbs.mixing();
        let sum = nbs.sum_at(TARGETS);
        assert_eq!(sum, 3);
    }

    #[test]
    fn part_2() {
        let mut nbs: Numbers = INPUT.parse().unwrap();
        nbs.decrypt(DECRYPTION_KEY);
        nbs.mixing_times(MIX_TIMES);
        let sum = nbs.sum_at(TARGETS);
        assert_eq!(sum, 1623178306);
    }
}
