use itertools::Itertools;
use std::str::FromStr;

struct Cups {
    cups: Vec<u32>,
    max: u32,
}

impl Cups {
    fn n_moves(&mut self, n: usize) {
        for _ in 0..n {
            self.one_move();
        }
    }

    fn next(&self, n: u32, except: &[u32]) -> u32 {
        let mut s: u32 = if n == 1 { self.max } else { n - 1 };
        while except.contains(&s) {
            s = self.next(s, except);
        }
        s
    }

    fn one_move(&mut self) {
        let head = self.cups[0];
        self.cups.rotate_left(1);
        let search: u32 = self.next(head, &self.cups[0..3]);
        let pos: usize = self
            .cups
            .iter()
            .position(|c| *c == search)
            .map(|pos| pos + 1)
            .unwrap();
        self.cups[..pos].rotate_left(3);
    }

    fn extend_to(&mut self, n: u32) {
        self.cups.extend((self.max + 1)..=n);
        self.max = n;
    }

    fn one_in_front(&mut self) {
        let pos: usize = self.cups.iter().position(|c| *c == 1).unwrap();
        self.cups.rotate_left(pos);
    }

    fn labels(&mut self) -> String {
        self.one_in_front();
        self.cups[1..].iter().join("")
    }

    fn product(&mut self) -> usize {
        self.one_in_front();
        println!("{} {} {}", self.cups[0], self.cups[1], self.cups[2]);
        self.cups[1] as usize * self.cups[2] as usize
    }
}

impl FromStr for Cups {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cups: Vec<u32> = s.chars().map(|c| c.to_digit(10).unwrap()).collect();
        let max: u32 = cups.iter().max().copied().unwrap();
        Ok(Cups { cups, max })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let input = "962713854";
    let mut cups: Cups = input.parse().unwrap();
    cups.n_moves(100);
    println!(
        "Part1: After doing 100 moves, the labels after cup 1 are {}",
        cups.labels()
    );
    cups = input.parse().unwrap();
    cups.extend_to(1_000_000);
    cups.n_moves(10_000_000);
    println!(
        "Part2: After doing 10000000 moves, the two labels after cup 1 multiply to {}",
        cups.product()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "389125467";

    #[test]
    fn test_1_part_1() {
        let mut cups: Cups = INPUT.parse().unwrap();
        cups.n_moves(10);
        assert_eq!(cups.labels(), "92658374");
    }

    #[test]
    fn test_2_part_1() {
        let mut cups: Cups = INPUT.parse().unwrap();
        cups.n_moves(100);
        assert_eq!(cups.labels(), "67384529");
    }

    #[test]
    fn test_part_2() {
        let mut cups: Cups = INPUT.parse().unwrap();
        cups.extend_to(1_000_000);
        cups.n_moves(10_000_000);
        assert_eq!(cups.product(), 149245887792);
    }
}
