use std::collections::HashSet;
use std::str::FromStr;

struct ScratchCard {
    winning: HashSet<usize>,
    numbers: Vec<usize>,
}

impl ScratchCard {
    fn points(&self) -> usize {
        match self.nb_match() {
            0 => 0,
            n => 2_usize.pow((n - 1) as u32),
        }
    }

    fn nb_match(&self) -> usize {
        self.numbers
            .iter()
            .filter(|n| self.winning.contains(*n))
            .count()
    }
}

impl FromStr for ScratchCard {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (winning, numbers) = s.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning: HashSet<usize> = winning
            .split_whitespace()
            .map(|segment| segment.parse().unwrap())
            .collect();
        let numbers: Vec<usize> = numbers
            .split_whitespace()
            .map(|segment| segment.parse().unwrap())
            .collect();

        Ok(ScratchCard { winning, numbers })
    }
}

struct CardPile {
    cards: Vec<ScratchCard>,
}

impl CardPile {
    fn points(&self) -> usize {
        self.cards.iter().map(|c| c.points()).sum()
    }

    fn total_scratchcards(&self) -> usize {
        let size: usize = self.cards.len();
        let mut nb_cards: Vec<usize> = vec![1; size];

        for (i, card) in self.cards.iter().enumerate() {
            let nb: usize = nb_cards[i];
            let nb_match: usize = card.nb_match();

            //Add "nb" cards to the next cards, depending on the number of match
            (i + 1..=i + nb_match)
                .filter(|&id| id < size)
                .for_each(|id| nb_cards[id] += nb);
        }

        nb_cards.into_iter().sum()
    }
}

impl FromStr for CardPile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cards: Vec<ScratchCard> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(CardPile { cards })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_04.txt").expect("Cannot open input file");
    let pile: CardPile = s.parse().unwrap();

    println!(
        "Part1: The pile of ScratchCards is worth {} points",
        pile.points()
    );
    println!(
        "Part2: After scratching all the ScratchCards, you end up with {} ScratchCards",
        pile.total_scratchcards()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11
";

    #[test]
    fn part_1() {
        let pile: CardPile = EXAMPLE_1.parse().unwrap();
        assert_eq!(pile.points(), 13);
    }
    #[test]
    fn part_2() {
        let pile: CardPile = EXAMPLE_1.parse().unwrap();
        assert_eq!(pile.total_scratchcards(), 30);
    }
}
