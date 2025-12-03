use nom::bytes::complete::tag;
use nom::character::complete::anychar;
use nom::sequence::preceded;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct DiceGame {
    player_0: usize,
    player_1: usize,
}

impl DiceGame {
    fn deterministic_game_score(&self) -> usize {
        fn roll_three_dice(dice: &mut usize, nb_rolls: &mut usize) -> usize {
            let mut rolls: usize = 0;
            for _ in 0..3 {
                *dice = *dice % 100 + 1;
                rolls += *dice;
            }
            *nb_rolls += 3;
            rolls
        }
        let mut scores: Vec<usize> = vec![0, 0];
        let mut position: Vec<usize> = vec![self.player_0, self.player_1];
        let mut dice: usize = 0;
        let mut nb_rolls: usize = 0;
        let mut current_player: usize = 0;

        while scores.iter().all(|s| *s < 1000) {
            let rolls = roll_three_dice(&mut dice, &mut nb_rolls);
            position[current_player] = (position[current_player] + rolls - 1) % 10 + 1;
            scores[current_player] += position[current_player];
            current_player = (current_player + 1) % 2
        }

        scores.iter().min().unwrap() * nb_rolls
    }

    fn dirac_game_score(&self) -> usize {
        //All the rolls possible and their distribution
        let rolls: Vec<(usize, usize)> =
            vec![(3, 1), (4, 3), (5, 6), (6, 7), (7, 6), (8, 3), (9, 1)];
        //4 dimension state game. [P1 score][P2 score][P1 pos][P2 pos]
        let mut games: Vec<Vec<Vec<Vec<usize>>>> = vec![vec![vec![vec![0; 10]; 10]; 21]; 21];
        let mut wins: Vec<usize> = vec![0, 0];
        let mut current_player: usize = 0;

        //Starting position
        games[0][0][self.player_0 - 1][self.player_1 - 1] = 1;

        //Loop while all games are not finished
        while !games.iter().all(|score_2| {
            score_2
                .iter()
                .all(|pos_1| pos_1.iter().all(|pos_2| pos_2.iter().all(|n| *n == 0)))
        }) {
            let mut temp_games: Vec<Vec<Vec<Vec<usize>>>> =
                vec![vec![vec![vec![0; 10]; 10]; 21]; 21];
            for score_1 in 0..21 {
                for score_2 in 0..21 {
                    for pos_1 in 0..10 {
                        for pos_2 in 0..10 {
                            //Get number of games currently in that state
                            let nb_games = games[score_1][score_2][pos_1][pos_2];
                            if nb_games > 0 {
                                if current_player == 0 {
                                    rolls.iter().for_each(|(value, number)| {
                                        let new_pos = (pos_1 + value) % 10 + 1;
                                        let new_score = score_1 + new_pos;
                                        if new_score >= 21 {
                                            //Add wins for player 0
                                            wins[current_player] += nb_games * number;
                                        } else {
                                            //Add the number of games created to the new position
                                            temp_games[new_score][score_2][new_pos - 1][pos_2] +=
                                                nb_games * number;
                                        }
                                    });
                                } else {
                                    rolls.iter().for_each(|(value, number)| {
                                        let new_pos = (pos_2 + value) % 10 + 1;
                                        let new_score = score_2 + new_pos;
                                        if new_score >= 21 {
                                            //Add wins for player 0
                                            wins[current_player] += nb_games * number;
                                        } else {
                                            //Add the number of games created to the new position
                                            temp_games[score_1][new_score][pos_1][new_pos - 1] +=
                                                nb_games * number;
                                        }
                                    });
                                }
                            }
                        }
                    }
                }
            }

            current_player = (current_player + 1) % 2;
            games = temp_games;
        }

        wins.into_iter().max().unwrap()
    }
}

impl FromStr for DiceGame {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_player(s: &str) -> IResult<&str, usize> {
            preceded(
                tag("Player "),
                preceded(anychar, preceded(tag(" starting position: "), parse_usize)),
            )
            .parse(s)
        }
        let players: Vec<usize> = s.lines().map(|l| parse_player(l).unwrap().1).collect();
        Ok(DiceGame {
            player_0: players[0],
            player_1: players[1],
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_21.txt").expect("Cannot open input file");
    let game: DiceGame = s.parse().unwrap();
    println!(
        "Part1: The score for a game with a deterministic game is {}",
        game.deterministic_game_score()
    );
    println!(
        "Part2: The score for a game with a Dirac game is {}",
        game.dirac_game_score()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Player 1 starting position: 4
Player 2 starting position: 8
";

    #[test]
    fn part_1() {
        let game: DiceGame = EXAMPLE_1.parse().unwrap();
        assert_eq!(739_785, game.deterministic_game_score());
    }

    #[test]
    fn part_2() {
        let game: DiceGame = EXAMPLE_1.parse().unwrap();
        assert_eq!(444_356_092_776_315, game.dirac_game_score());
    }
}
