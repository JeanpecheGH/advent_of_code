use std::cmp::Ordering;
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn from_sets_size(a: usize, b: usize) -> HandType {
        match (a, b) {
            (1, _) => HandType::FiveOfAKind,
            (2, 1) => HandType::FourOfAKind,
            (2, 2) => HandType::FullHouse,
            (3, 1) => HandType::ThreeOfAKind,
            (3, 2) => HandType::TwoPair,
            (4, _) => HandType::OnePair,
            (_, _) => HandType::HighCard,
        }
    }

    fn from_cards(cards: &[Card]) -> HandType {
        let (first, second): (HashSet<Card>, HashSet<Card>) = cards.iter().fold(
            (HashSet::new(), HashSet::new()),
            |(mut first, mut second), &card| {
                if !first.insert(card) {
                    let _ = second.insert(card);
                }
                (first, second)
            },
        );
        Self::from_sets_size(first.len(), second.len())
    }

    fn from_joker_cards(cards: &[JokerCard]) -> HandType {
        let (first, second, has_jokers): (HashSet<JokerCard>, HashSet<JokerCard>, bool) =
            cards.iter().fold(
                (HashSet::new(), HashSet::new(), false),
                |(mut first, mut second, jokers), &card| {
                    if card == JokerCard::Joker {
                        (first, second, true)
                    } else {
                        if !first.insert(card) {
                            let _ = second.insert(card);
                        }
                        (first, second, jokers)
                    }
                },
            );
        let a: usize = if first.is_empty() { 1 } else { first.len() };
        let b: usize = if second.is_empty() && has_jokers {
            1
        } else {
            second.len()
        };
        Self::from_sets_size(a, b)
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum JokerCard {
    Zero,
    Joker,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Queen,
    King,
    Ace,
}

impl JokerCard {
    fn from_char(c: char) -> JokerCard {
        match c {
            '2' => JokerCard::Two,
            '3' => JokerCard::Three,
            '4' => JokerCard::Four,
            '5' => JokerCard::Five,
            '6' => JokerCard::Six,
            '7' => JokerCard::Seven,
            '8' => JokerCard::Eight,
            '9' => JokerCard::Nine,
            'T' => JokerCard::Ten,
            'J' => JokerCard::Joker,
            'Q' => JokerCard::Queen,
            'K' => JokerCard::King,
            'A' => JokerCard::Ace,
            _ => JokerCard::Zero, //Should not happen
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd, Debug)]
enum Card {
    Zero,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Card {
        match c {
            '2' => Card::Two,
            '3' => Card::Three,
            '4' => Card::Four,
            '5' => Card::Five,
            '6' => Card::Six,
            '7' => Card::Seven,
            '8' => Card::Eight,
            '9' => Card::Nine,
            'T' => Card::Ten,
            'J' => Card::Jack,
            'Q' => Card::Queen,
            'K' => Card::King,
            'A' => Card::Ace,
            _ => Card::Zero, //Should not happen
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand<T: Ord> {
    hand_type: HandType,
    cards: Vec<T>,
    bid: usize,
}

impl<T: Ord> Ord for Hand<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.hand_type
            .cmp(&other.hand_type)
            .then(self.cards.cmp(&other.cards))
    }
}

impl<T: Ord> PartialOrd for Hand<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Hand<Card> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid: usize = bid.parse().unwrap();
        let cards: Vec<Card> = cards.chars().map(Card::from_char).collect();
        let hand_type: HandType = HandType::from_cards(&cards);

        Ok(Hand {
            hand_type,
            cards,
            bid,
        })
    }
}

impl FromStr for Hand<JokerCard> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').unwrap();
        let bid: usize = bid.parse().unwrap();
        let cards: Vec<JokerCard> = cards.chars().map(JokerCard::from_char).collect();
        let hand_type: HandType = HandType::from_joker_cards(&cards);

        Ok(Hand {
            hand_type,
            cards,
            bid,
        })
    }
}

struct Hands<T: Ord> {
    hands: Vec<Hand<T>>,
}

impl<T: Ord> Hands<T> {
    fn total_winnings(&mut self) -> usize {
        self.hands.sort();
        self.hands
            .iter()
            .enumerate()
            .map(|(rank, hand)| (rank + 1) * hand.bid)
            .sum()
    }
}
impl FromStr for Hands<Card> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<Hand<Card>> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Hands { hands })
    }
}

impl FromStr for Hands<JokerCard> {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let hands: Vec<Hand<JokerCard>> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Hands { hands })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_07.txt").expect("Cannot open input file");
    let mut hands: Hands<Card> = s.parse().unwrap();

    println!("Part1: The total winnings are {}", hands.total_winnings());
    let mut hands: Hands<JokerCard> = s.parse().unwrap();
    println!(
        "Part2: Using Jokers instead of Jacks, the total winnings are now {}",
        hands.total_winnings()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

    #[test]
    fn part_1() {
        let mut hands: Hands<Card> = EXAMPLE_1.parse().unwrap();
        assert_eq!(hands.total_winnings(), 6440);
    }
    #[test]
    fn part_2() {
        let mut hands: Hands<JokerCard> = EXAMPLE_1.parse().unwrap();
        assert_eq!(hands.total_winnings(), 5905);
    }
}
