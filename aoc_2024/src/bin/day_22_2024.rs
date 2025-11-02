use std::str::FromStr;

use bitvec::bitvec;
use rayon::prelude::*;

struct MonkeyMarket {
    numbers: Vec<isize>,
}
impl MonkeyMarket {
    fn next(secret: isize) -> isize {
        let mut s: isize = secret;
        s = ((s * 64) ^ s) % 16777216;
        s = ((s / 32) ^ s) % 16777216;
        ((s * 2048) ^ s) % 16777216
    }
    fn next_n(secret: isize, n: usize) -> Vec<isize> {
        let mut s: isize = secret;
        let mut values: Vec<isize> = vec![secret];
        for _ in 0..n {
            s = MonkeyMarket::next(s);
            values.push(s);
        }
        values
    }

    fn index((a, b, c, d): (isize, isize, isize, isize)) -> usize {
        ((a + 9) as usize) * 19usize.pow(3)
            + ((b + 9) as usize) * 19usize.pow(2)
            + ((c + 9) as usize) * 19usize.pow(1)
            + (d + 9) as usize
    }

    fn max_bananas(cache: &mut [isize], values: &[isize]) {
        let changes: Vec<isize> = values
            .windows(2)
            .map(|pair| (pair[1] % 10) - (pair[0] % 10))
            .collect();

        let mut seen = bitvec![0; 19usize.pow(4)];
        for (i, quad) in changes.windows(4).enumerate() {
            let idx: usize = MonkeyMarket::index((quad[0], quad[1], quad[2], quad[3]));
            if !seen[idx] {
                seen.set(idx, true);
                cache[idx] += values[i + 4] % 10;
            }
        }
    }

    fn iterate(&self, times: usize) -> (isize, isize) {
        let all_values: Vec<Vec<isize>> = self
            .numbers
            .par_iter()
            .map(|&n| MonkeyMarket::next_n(n, times))
            .collect();
        let sum_last: isize = all_values.par_iter().map(|n| n.last().unwrap()).sum();

        let mut banana_cache: Vec<isize> = vec![0; 19usize.pow(4)];
        all_values
            .iter()
            .for_each(|n| MonkeyMarket::max_bananas(&mut banana_cache, n));
        let max_bananas: isize = banana_cache.into_iter().max().unwrap();

        (sum_last, max_bananas)
    }
}

impl FromStr for MonkeyMarket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<isize> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(MonkeyMarket { numbers })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_22.txt").expect("Cannot open input file");
    let market: MonkeyMarket = s.parse().unwrap();
    let (sum, max_bananas) = market.iterate(2000);
    println!("Part1: The sum of all the 2000th generated secrets is {sum}");
    println!("Part2: At most, you can get {max_bananas} bananas");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1
10
100
2024
";

    const EXAMPLE_2: &str = "1
2
3
2024
";

    #[test]
    fn hash_test() {
        let mut s = 123;
        s = MonkeyMarket::next(s);
        assert_eq!(s, 15887950);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 16495136);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 527345);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 704524);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 1553684);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 12683156);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 11100544);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 12249484);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 7753432);
        s = MonkeyMarket::next(s);
        assert_eq!(s, 5908254);
    }

    #[test]
    fn part_1() {
        let market: MonkeyMarket = EXAMPLE_1.parse().unwrap();
        assert_eq!(market.iterate(2000).0, 37327623);
    }

    #[test]
    fn part_2() {
        let market: MonkeyMarket = EXAMPLE_2.parse().unwrap();
        assert_eq!(market.iterate(2000).1, 23);
    }
}
