use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::IResult;
use std::collections::VecDeque;
use std::str::FromStr;
use util::basic_parser::parse_usize;

struct MarbleGame {
    nb_players: usize,
    nb_marbles: usize,
}

impl MarbleGame {
    fn high_score(&self, times: usize) -> usize {
        let mut marbles: VecDeque<usize> = VecDeque::new();
        marbles.push_back(0);
        marbles.push_back(1);
        let mut scores: Vec<usize> = vec![0; self.nb_players];

        for i in 2..=(self.nb_marbles * times) {
            if i % 23 == 0 {
                marbles.rotate_right(7);
                let m = marbles.pop_back().unwrap();
                marbles.rotate_left(1);
                scores[i % self.nb_players] += m + i;
            } else {
                marbles.rotate_left(1);
                marbles.push_back(i);
            }
        }

        scores.into_iter().max().unwrap()
    }
}

impl FromStr for MarbleGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_game(s: &str) -> IResult<&str, MarbleGame> {
            let (s, nb_players) =
                terminated(parse_usize, tag(" players; last marble is worth "))(s)?;
            let (s, nb_marbles) = terminated(parse_usize, tag(" points"))(s)?;

            Ok((
                s,
                MarbleGame {
                    nb_players,
                    nb_marbles,
                },
            ))
        }
        Ok(parse_game(s).unwrap().1)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_09.txt").expect("Cannot open input file");
    let game: MarbleGame = s.parse().unwrap();

    println!("Part1: The winning Elf's score is {}", game.high_score(1));
    println!(
        "Part2: If the last marble is 100 times bigger, the high score is {}",
        game.high_score(100)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "9 players; last marble is worth 25 points";
    const EXAMPLE_2: &str = "10 players; last marble is worth 1618 points";
    const EXAMPLE_3: &str = "13 players; last marble is worth 7999 points";
    const EXAMPLE_4: &str = "17 players; last marble is worth 1104 points";
    const EXAMPLE_5: &str = "21 players; last marble is worth 6111 points";
    const EXAMPLE_6: &str = "30 players; last marble is worth 5807 points";

    #[test]
    fn part_1_test_1() {
        let game: MarbleGame = EXAMPLE_1.parse().unwrap();
        assert_eq!(game.high_score(1), 32);
    }
    #[test]
    fn part_1_test_2() {
        let game: MarbleGame = EXAMPLE_2.parse().unwrap();
        assert_eq!(game.high_score(1), 8317);
    }
    #[test]
    fn part_1_test_3() {
        let game: MarbleGame = EXAMPLE_3.parse().unwrap();
        assert_eq!(game.high_score(1), 146373);
    }
    #[test]
    fn part_1_test_4() {
        let game: MarbleGame = EXAMPLE_4.parse().unwrap();
        assert_eq!(game.high_score(1), 2764);
    }
    #[test]
    fn part_1_test_5() {
        let game: MarbleGame = EXAMPLE_5.parse().unwrap();
        assert_eq!(game.high_score(1), 54718);
    }
    #[test]
    fn part_1_test_6() {
        let game: MarbleGame = EXAMPLE_6.parse().unwrap();
        assert_eq!(game.high_score(1), 37305);
    }
}
