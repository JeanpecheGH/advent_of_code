use fxhash::FxHashMap;
use nom::bytes::complete::{tag, take};
use nom::character::complete::anychar;
use nom::multi::many1;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::VecDeque;
use std::str::FromStr;
use util::basic_parser::title;

#[derive(Debug)]
struct PotsRow {
    pots: VecDeque<bool>,
    start_pot: isize,
    generation: usize,
    rules: FxHashMap<usize, bool>,
}

impl PotsRow {
    fn pots_sum(&self) -> isize {
        self.pots
            .iter()
            .enumerate()
            .filter_map(|(n, &b)| {
                if b {
                    Some(n as isize + self.start_pot)
                } else {
                    None
                }
            })
            .sum()
    }

    fn n_generations(&mut self, n: usize) -> isize {
        for _ in 0..n {
            self.next_generation();
        }
        self.pots_sum()
    }

    fn generations_until_stable(&mut self, n: usize) -> isize {
        let mut cache = self.pots.clone();
        let mut start_pot: isize = self.start_pot;
        self.next_generation();

        while !cache.eq(&self.pots) {
            cache.clone_from(&self.pots);
            start_pot = self.start_pot;
            self.next_generation();
        }
        //After the pattern is stable, the plants will start "shifting" in a given direction
        let move_speed: isize = self.start_pot - start_pot;
        //The final sum will be the current sum to which we can add the sum due to shifting until we reach the final generation
        let current_sum: isize = self.pots_sum();
        let nb_plants: usize = self.pots.iter().filter(|&&b| b).count();
        let move_sum: isize =
            nb_plants as isize * move_speed * (n as isize - self.generation as isize);
        current_sum + move_sum
    }

    fn next_generation(&mut self) {
        let l: usize = self.pots.len();
        //We add enough empty pots to the left in order to compute the leftmost pot
        let add_back: usize = match (self.pots[l - 1], self.pots[l - 2], self.pots[l - 3]) {
            (true, _, _) => 3,
            (_, true, _) => 2,
            (_, _, true) => 1,
            _ => 0,
        };
        (0..add_back).for_each(|_| self.pots.push_back(false));

        //We add enough empty pots to the right in order to compute the rightmost pot
        let add_front: usize = match (self.pots[0], self.pots[1], self.pots[2]) {
            (true, _, _) => 3,
            (_, true, _) => 2,
            (_, _, true) => 1,
            _ => 0,
        };
        (0..add_front).for_each(|_| self.pots.push_front(false));

        //The starting pot is moved to minimize the number of empty pots on the left
        self.start_pot -= add_front as isize - 2;

        self.pots.make_contiguous();
        let left = self.pots.as_slices().0;

        self.pots = left
            .windows(5)
            .map(|five| {
                let hash: usize = five.iter().fold(0, |acc, &b| {
                    let v: usize = if b { 1 } else { 0 };
                    acc * 2 + v
                });
                self.rules.get(&hash).copied().unwrap_or(false)
            })
            .collect();

        self.generation += 1;
    }
}

impl FromStr for PotsRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_initial_state(s: &str) -> IResult<&str, VecDeque<bool>> {
            let (s, pots) = preceded(title, many1(anychar))(s)?;

            let pots: VecDeque<bool> = pots.into_iter().map(|c| c == '#').collect();
            Ok((s, pots))
        }

        fn parse_rule(s: &str) -> IResult<&str, (usize, bool)> {
            let (s, left) = take(5usize)(s)?;
            let (s, right) = preceded(tag(" => "), anychar)(s)?;

            let score: usize = left.chars().fold(0, |acc, c| {
                let p: usize = if c == '#' { 1 } else { 0 };
                acc * 2 + p
            });
            let is_plant: bool = right == '#';

            Ok((s, (score, is_plant)))
        }

        let mut lines = s.lines();
        let pots: VecDeque<bool> = parse_initial_state(lines.next().unwrap()).unwrap().1;
        lines.next();
        let rules: FxHashMap<usize, bool> = lines.map(|l| parse_rule(l).unwrap().1).collect();

        Ok(PotsRow {
            pots,
            start_pot: 0,
            generation: 0,
            rules,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_12.txt").expect("Cannot open input file");
    let mut row: PotsRow = s.parse().unwrap();

    println!("Part1: After 20 generations, the sum of the numbers of all pots which contain a plant is {}", row.n_generations(20));
    println!(
        "Part2: After 50 billion generations, this sum will be {}",
        row.generations_until_stable(50_000_000_000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "initial state: #..#.#..##......###...###

...## => #
..#.. => #
.#... => #
.#.#. => #
.#.## => #
.##.. => #
.#### => #
#.#.# => #
#.### => #
##.#. => #
##.## => #
###.. => #
###.# => #
####. => #
";

    #[test]
    fn part_1() {
        let mut row: PotsRow = EXAMPLE_1.parse().unwrap();
        assert_eq!(row.n_generations(20), 325);
    }
}
