use itertools::Itertools;
use std::str::FromStr;

struct Cups {
    cups: Vec<usize>,
    current: usize,
}

impl Cups {
    fn n_moves(&mut self, n: usize) {
        for _ in 0..n {
            self.one_move();
        }
    }

    fn next(&self, n: usize, except: &[usize]) -> usize {
        let mut s: usize = if n == 0 { self.cups.len() - 1 } else { n - 1 };
        while except.contains(&s) {
            s = self.next(s, except);
        }
        s
    }

    fn one_move(&mut self) {
        let a = self.cups[self.current];
        let b = self.cups[a];
        let c = self.cups[b];
        let new_head = self.cups[c];
        let next: usize = self.next(self.current, &[a, b, c]);

        self.cups[self.current] = new_head;
        self.cups[c] = self.cups[next];
        self.cups[next] = a;
        self.current = new_head;
    }

    fn extend_to(&mut self, n: usize) {
        let max: usize = self.cups.len() - 1;
        for e in self.cups.iter_mut() {
            if *e == self.current {
                *e = max + 1;
            }
        }
        for i in (max + 1)..(n - 1) {
            self.cups.push(i + 1);
        }
        self.cups.push(self.current);
    }

    fn labels(&mut self) -> String {
        let mut idx: usize = 0;
        let labels: Vec<usize> = (0..self.cups.len() - 1)
            .map(|_| {
                idx = self.cups[idx];
                idx + 1
            })
            .collect();
        labels.iter().join("")
    }

    fn product(&mut self) -> usize {
        let a: usize = self.cups[0];
        let b: usize = self.cups[a];
        (a + 1) * (b + 1)
    }
}

impl FromStr for Cups {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let digits: Vec<usize> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize - 1)
            .collect();
        let current: usize = digits.first().copied().unwrap();
        let last: usize = digits.last().copied().unwrap();

        let mut cups: Vec<usize> = vec![0; digits.len()];
        for pair in digits.windows(2) {
            cups[pair[0]] = pair[1];
        }
        cups[last] = current;
        Ok(Self { cups, current })
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
