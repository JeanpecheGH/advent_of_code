use std::cmp::max;
use std::collections::HashMap;
use std::str::FromStr;
use util::coord::Pos3;

const MAX: Pos3 = Pos3(12, 13, 14);

struct Game {
    draws: Vec<Pos3>,
}

impl Game {
    fn is_possible(&self, Pos3(r, g, b): Pos3) -> bool {
        self.draws
            .iter()
            .all(|&Pos3(i, j, k)| i <= r && j <= g && k <= b)
    }

    fn min_possible_set(&self) -> Pos3 {
        self.draws
            .iter()
            .fold(Pos3(0, 0, 0), |Pos3(r, g, b), &Pos3(i, j, k)| {
                Pos3(max(r, i), max(g, j), max(b, k))
            })
    }

    fn power(&self) -> usize {
        let Pos3(r, g, b) = self.min_possible_set();
        r * g * b
    }
}

impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn rgb(s: &str) -> Pos3 {
            let map: HashMap<&str, usize> = s
                .split(',')
                .map(|pair| {
                    let split: Vec<&str> = pair.split_whitespace().collect();
                    let value: usize = split[0].parse().unwrap();
                    let color: &str = split[1];
                    (color, value)
                })
                .collect();

            Pos3(
                map.get("red").copied().unwrap_or(0),
                map.get("green").copied().unwrap_or(0),
                map.get("blue").copied().unwrap_or(0),
            )
        }
        let draws: Vec<Pos3> = s.split_once(':').unwrap().1.split(';').map(rgb).collect();

        Ok(Game { draws })
    }
}
struct Conundrum {
    games: Vec<Game>,
}

impl Conundrum {
    fn possible_games(&self, max: Pos3) -> usize {
        self.games
            .iter()
            .enumerate()
            .filter_map(|(id, g)| {
                if g.is_possible(max) {
                    Some(id + 1)
                } else {
                    None
                }
            })
            .sum()
    }

    fn games_powers_sum(&self) -> usize {
        self.games.iter().map(|g| g.power()).sum()
    }
}

impl FromStr for Conundrum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let games: Vec<Game> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Conundrum { games })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_02.txt").expect("Cannot open input file");
    let con: Conundrum = s.parse().unwrap();

    println!(
        "Part1: The sum of the IDs of the games that are possible is {}",
        con.possible_games(MAX)
    );
    println!(
        "Part2: The total power of the games played is {}",
        con.games_powers_sum()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
";

    #[test]
    fn part_1() {
        let con: Conundrum = EXAMPLE_1.parse().unwrap();
        assert_eq!(con.possible_games(MAX), 8);
    }
    #[test]
    fn part_2() {
        let con: Conundrum = EXAMPLE_1.parse().unwrap();
        assert_eq!(con.games_powers_sum(), 2286);
    }
}
