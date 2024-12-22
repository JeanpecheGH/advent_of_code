use std::str::FromStr;

use fxhash::{FxHashMap, FxHashSet};

#[derive(Debug, Clone, Copy)]
struct SecretNumber {
    n: isize,
}

impl SecretNumber {
    fn next(&self) -> SecretNumber {
        let mut secret: isize = self.n;
        let mul: isize = secret * 64;
        secret ^= mul;
        secret %= 16777216;
        let div: isize = secret / 32;
        secret ^= div;
        secret %= 16777216;
        let mul_2: isize = secret * 2048;
        secret ^= mul_2;
        secret %= 16777216;

        SecretNumber { n: secret }
    }

    fn next_n(&self, n: isize) -> Vec<isize> {
        let mut secret: SecretNumber = *self;
        let mut values: Vec<isize> = vec![secret.n];
        for _ in 0..n {
            secret = secret.next();
            values.push(secret.n);
        }
        values
    }
}

struct MonkeyMarket {
    numbers: Vec<SecretNumber>,
}
impl MonkeyMarket {
    fn max_bananas(cache: &mut FxHashMap<(isize, isize, isize, isize), isize>, values: &[isize]) {
        let mut key_set: FxHashSet<(isize, isize, isize, isize)> = FxHashSet::default();
        for quint in values.windows(5) {
            let a: isize = (quint[1] % 10) - (quint[0] % 10);
            let b: isize = (quint[2] % 10) - (quint[1] % 10);
            let c: isize = (quint[3] % 10) - (quint[2] % 10);
            let d: isize = (quint[4] % 10) - (quint[3] % 10);
            let key: (isize, isize, isize, isize) = (a, b, c, d);
            if key_set.insert(key) {
                *cache.entry(key).or_default() += quint[4] % 10;
            }
        }
    }

    fn iterate(&self, times: isize) -> (isize, isize) {
        let all_values: Vec<Vec<isize>> = self.numbers.iter().map(|n| n.next_n(times)).collect();
        let sum_last: isize = all_values.iter().map(|n| n.last().unwrap()).sum();

        let mut banana_cache: FxHashMap<(isize, isize, isize, isize), isize> = FxHashMap::default();
        all_values
            .iter()
            .for_each(|n| MonkeyMarket::max_bananas(&mut banana_cache, n));
        let max_bananas: isize = banana_cache.values().copied().max().unwrap();

        (sum_last, max_bananas)
    }
}

impl FromStr for MonkeyMarket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<SecretNumber> = s
            .lines()
            .map(|l| SecretNumber {
                n: l.parse().unwrap(),
            })
            .collect();

        Ok(MonkeyMarket { numbers })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_22.txt").expect("Cannot open input file");
    let market: MonkeyMarket = s.parse().unwrap();
    let (sum, max_bananas) = market.iterate(2000);
    println!("Part1: {sum}");
    println!("Part2: {max_bananas}");
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
        let secret = SecretNumber { n: 123 };
        let next = secret.next();
        assert_eq!(next.n, 15887950);
        let next = next.next();
        assert_eq!(next.n, 16495136);
        let next = next.next();
        assert_eq!(next.n, 527345);
        let next = next.next();
        assert_eq!(next.n, 704524);
        let next = next.next();
        assert_eq!(next.n, 1553684);
        let next = next.next();
        assert_eq!(next.n, 12683156);
        let next = next.next();
        assert_eq!(next.n, 11100544);
        let next = next.next();
        assert_eq!(next.n, 12249484);
        let next = next.next();
        assert_eq!(next.n, 7753432);
        let next = next.next();
        assert_eq!(next.n, 5908254);
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
