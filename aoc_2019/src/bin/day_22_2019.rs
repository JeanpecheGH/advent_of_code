use std::collections::HashMap;
use std::str::FromStr;
use util::chinese_remainders::bezout_triplet;

#[derive(Copy, Clone, Debug)]
enum Technique {
    Deal,
    DealWithIncrement(u128),
    Cut(i128),
}

impl FromStr for Technique {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        if words[0] == "cut" {
            let cut_size: i128 = words[1].parse().unwrap();
            Ok(Technique::Cut(cut_size))
        } else if words[1] == "with" {
            let increment: u128 = words[3].parse().unwrap();
            Ok(Technique::DealWithIncrement(increment))
        } else {
            Ok(Technique::Deal)
        }
    }
}

struct BetterDeck {
    size: u128,
    start: u128,
    increment: u128,
    shuffler: Vec<Technique>,
    cache: HashMap<(u128, u128), i128>,
}

impl BetterDeck {
    fn new(size: u128, shuffler_lines: &str) -> BetterDeck {
        let shuffler: Vec<Technique> = shuffler_lines.lines().map(|l| l.parse().unwrap()).collect();
        BetterDeck {
            size,
            start: 0,
            increment: 1,
            shuffler,
            cache: HashMap::new(),
        }
    }

    fn cached_bezout_coeff(&mut self, a: u128, b: u128) -> i128 {
        let entry = self
            .cache
            .entry((a, b))
            .or_insert_with(|| bezout_triplet(a, b).1);
        *entry
    }

    fn pos_of(&mut self, card: u128) -> u128 {
        // We're looking for the position which satisfies
        // start + increment*pos = card mod size
        let rest = if card > self.start {
            card - self.start
        } else {
            card + self.size - self.start
        };

        let u: i128 = self.cached_bezout_coeff(self.increment, self.size);
        if u > 0 {
            (u as u128 * rest) % self.size
        } else {
            ((self.size as i128 + u) as u128 * rest) % self.size
        }
    }

    fn card_at(&self, pos: u128) -> u128 {
        (self.start + self.increment * pos) % self.size
    }

    #[allow(dead_code)]
    fn to_vec(&self) -> Vec<u128> {
        let mut n = self.start;
        let mut v: Vec<u128> = vec![n];
        for _ in 1..self.size {
            n += self.increment;
            n %= self.size;
            v.push(n);
        }
        v
    }

    fn generate_cut_and_deal(&mut self) -> (Technique, Technique) {
        //Every shuffle can be simplfied to a single cut and a single deal
        //Since a deal does not move the first card, the cut is obvious
        let cut: Technique = Technique::Cut(self.start as i128);

        //For the deal, we want a deal that satisfies
        // the reverse of deck_inc * deal_inc = 1 mod size

        let u: i128 = self.cached_bezout_coeff(self.increment, self.size);
        let inc: u128 = if u > 0 {
            u as u128
        } else {
            (self.size as i128 + u) as u128
        };
        let deal: Technique = Technique::DealWithIncrement(inc);

        (cut, deal)
    }

    //Shuffling section
    fn multi_shuffle(&mut self, n: u128) {
        let binary: String = format!("{n:b}");
        let mut cache: Vec<(Technique, Technique)> = Vec::new();
        for _ in 0..binary.len() {
            self.shuffle();
            let pair: (Technique, Technique) = self.generate_cut_and_deal();
            cache.push(pair);
            self.shuffler = vec![pair.0, pair.1];
        }
        cache.reverse();
        // At this point, we already shuffled up to the most significant bit
        // We remove it and shuffle up to the least significant bit
        binary[1..].chars().enumerate().for_each(|(n, c)| {
            if c == '1' {
                let pair = cache[n + 1];
                self.shuffler = vec![pair.0, pair.1];
                self.shuffle()
            }
        });
    }

    fn shuffle(&mut self) {
        for tech in self.shuffler.clone() {
            match tech {
                Technique::Deal => self.deal(),
                Technique::DealWithIncrement(inc) => self.deal_with_increment(inc),
                Technique::Cut(size) => self.cut(size),
            }
        }
    }

    fn cut(&mut self, size: i128) {
        //increments stays the same, we shift the start
        if size > 0 {
            self.start = (self.start + size as u128 * self.increment) % self.size;
        } else {
            let inc = self.size - self.increment;
            self.start = (self.start + (-size) as u128 * inc) % self.size
        }
    }

    fn deal(&mut self) {
        //We're reversing the deck
        //increment is inversed
        self.increment = self.size - self.increment;
        // the end is the new start
        self.start = (self.start + self.increment) % self.size;
    }

    fn deal_with_increment(&mut self, inc: u128) {
        //Start stays unchanged
        // new_increment * inc = old_increment mod size

        let u: i128 = self.cached_bezout_coeff(inc, self.size);
        self.increment = if u > 0 {
            (u as u128 * self.increment) % self.size
        } else {
            ((self.size as i128 + u) as u128 * self.increment) % self.size
        }
    }
}
fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_22.txt").expect("Cannot open input file");
    let mut deck: BetterDeck = BetterDeck::new(10007, &s);
    deck.shuffle();

    println!("Part1: The card 2019 is at position {}", deck.pos_of(2019));

    //Part 2
    let size: u128 = 119_315_717_514_047;
    let nb_shuffle: u128 = 101_741_582_076_661;

    let mut bigger_deck: BetterDeck = BetterDeck::new(119_315_717_514_047, &s);
    bigger_deck.multi_shuffle(101_741_582_076_661);
    println!(
        "Part2: After shuffling a {} cards deck {} times, the card at position 2020 is {}",
        size,
        nb_shuffle,
        bigger_deck.card_at(2020)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "deal with increment 7
deal into new stack
deal into new stack";

    const EXAMPLE_2: &str = "cut 6
deal with increment 7
deal into new stack";

    const EXAMPLE_3: &str = "deal with increment 7
deal with increment 9
cut -2";

    const EXAMPLE_4: &str = "deal into new stack
cut -2
deal with increment 7
cut 8
cut -4
deal with increment 7
cut 3
deal with increment 9
deal with increment 3
cut -1";

    #[test]
    fn example_1() {
        let result: Vec<u128> = vec![0, 3, 6, 9, 2, 5, 8, 1, 4, 7];
        let mut deck: BetterDeck = BetterDeck::new(10, EXAMPLE_1);
        deck.shuffle();
        assert_eq!(result, deck.to_vec());
    }

    #[test]
    fn example_2() {
        let result: Vec<u128> = vec![3, 0, 7, 4, 1, 8, 5, 2, 9, 6];
        let mut deck: BetterDeck = BetterDeck::new(10, EXAMPLE_2);
        deck.shuffle();
        assert_eq!(result, deck.to_vec());
    }

    #[test]
    fn example_3() {
        let result: Vec<u128> = vec![6, 3, 0, 7, 4, 1, 8, 5, 2, 9];
        let mut deck: BetterDeck = BetterDeck::new(10, EXAMPLE_3);
        deck.shuffle();
        assert_eq!(result, deck.to_vec());
    }

    #[test]
    fn example_4() {
        let result: Vec<u128> = vec![9, 2, 5, 8, 1, 4, 7, 0, 3, 6];
        let mut deck: BetterDeck = BetterDeck::new(10, EXAMPLE_4);
        deck.shuffle();
        assert_eq!(result, deck.to_vec());
    }
}
