use std::collections::{HashSet, VecDeque};
use std::str::FromStr;

struct Combat {
    deck_1: VecDeque<usize>,
    deck_2: VecDeque<usize>,
    hands: HashSet<(Vec<usize>, Vec<usize>)>,
    ended: bool,
}

impl Combat {
    fn new(deck_1: &[usize], deck_2: &[usize]) -> Self {
        let deck_1: VecDeque<usize> = VecDeque::from(Vec::from(deck_1));
        let deck_2: VecDeque<usize> = VecDeque::from(Vec::from(deck_2));
        let hands = HashSet::new();
        Self {
            deck_1,
            deck_2,
            hands,
            ended: false,
        }
    }

    fn play(&mut self, rec: bool) {
        while self.can_play() {
            self.play_round(rec);
        }
    }

    fn play_round(&mut self, rec: bool) {
        //Register each pair of hands
        if rec {
            let pairs = (
                Vec::from(self.deck_1.clone()),
                Vec::from(self.deck_2.clone()),
            );
            if !self.hands.insert(pairs) {
                self.ended = true;
                return;
            }
        }
        let card_1: usize = self.deck_1.pop_front().unwrap();
        let card_2: usize = self.deck_2.pop_front().unwrap();

        let mini_game: bool = rec && card_1 <= self.deck_1.len() && card_2 <= self.deck_2.len();

        match (mini_game, card_1 > card_2) {
            (false, true) => {
                self.deck_1.push_back(card_1);
                self.deck_1.push_back(card_2);
            }
            (false, false) => {
                self.deck_2.push_back(card_2);
                self.deck_2.push_back(card_1);
            }
            (true, _) => {
                self.deck_1.make_contiguous();
                self.deck_2.make_contiguous();

                let (slice_1, _): (&[usize], &[usize]) = self.deck_1.as_slices();
                let (slice_2, _): (&[usize], &[usize]) = self.deck_2.as_slices();

                let mut mini_game: Self = Self::new(&slice_1[..card_1], &slice_2[..card_2]);
                mini_game.play(rec);
                if mini_game.win() {
                    self.deck_1.push_back(card_1);
                    self.deck_1.push_back(card_2);
                } else {
                    self.deck_2.push_back(card_2);
                    self.deck_2.push_back(card_1);
                }
            }
        }
    }

    fn can_play(&self) -> bool {
        !self.ended && !self.deck_1.is_empty() && !self.deck_2.is_empty()
    }

    fn win(&self) -> bool {
        self.ended || self.deck_2.is_empty()
    }

    fn score(&mut self) -> usize {
        let (winning_deck, _): (&[usize], &[usize]) = if self.deck_1.is_empty() {
            self.deck_2.make_contiguous();
            self.deck_2.as_slices()
        } else {
            self.deck_1.make_contiguous();
            self.deck_1.as_slices()
        };

        winning_deck
            .iter()
            .rev()
            .enumerate()
            .map(|(idx, &card)| (idx + 1) * card)
            .sum()
    }
}

impl FromStr for Combat {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let decks: Vec<&str> = util::split_blocks(s);
        let deck_1: VecDeque<usize> = decks[0]
            .lines()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect();
        let deck_2: VecDeque<usize> = decks[1]
            .lines()
            .skip(1)
            .map(|l| l.parse().unwrap())
            .collect();
        let hands = HashSet::new();
        Ok(Self {
            deck_1,
            deck_2,
            hands,
            ended: false,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_22.txt").expect("Cannot open input file");
    let mut game: Combat = s.parse().unwrap();
    game.play(false);
    println!("Part1: The winning player score is {}", game.score());
    let mut rec_game: Combat = s.parse().unwrap();
    rec_game.play(true);
    println!(
        "Part2: The winning player score is now {}",
        rec_game.score()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";

    const INPUT_2: &str = "Player 1:
43
19

Player 2:
2
29
14";

    #[test]
    fn test_part_1() {
        let mut game: Combat = INPUT.parse().unwrap();
        game.play(false);
        assert_eq!(game.score(), 306);
    }

    #[test]
    fn test_part_2() {
        let mut game: Combat = INPUT.parse().unwrap();
        game.play(true);
        assert_eq!(game.score(), 291);
    }

    #[test]
    fn test_part_2_loop() {
        let mut game: Combat = INPUT_2.parse().unwrap();
        game.play(true);
        assert_eq!(game.score(), 105);
    }
}
