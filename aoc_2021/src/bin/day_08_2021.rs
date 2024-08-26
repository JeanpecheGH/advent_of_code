use fxhash::FxHashMap;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct Display {
    dict: Vec<Vec<char>>,
    numbers: Vec<Vec<char>>,
}

impl Display {
    fn build_number_map(&self) -> FxHashMap<Vec<char>, usize> {
        let mut remaining_numbers: Vec<Vec<char>> = self.dict.clone();
        let mut digit_map: FxHashMap<usize, Vec<char>> = FxHashMap::default();

        //The only number with 2 segments is 1
        if let Some((i, one)) = remaining_numbers.iter().find_position(|n| n.len() == 2) {
            digit_map.insert(1, one.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //The only number with 3 segments is 7
        if let Some((i, seven)) = remaining_numbers.iter().find_position(|n| n.len() == 3) {
            digit_map.insert(7, seven.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //The only number with 4 segments is 4
        if let Some((i, four)) = remaining_numbers.iter().find_position(|n| n.len() == 4) {
            digit_map.insert(4, four.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //The only number with 7 segments is 8
        if let Some((i, eight)) = remaining_numbers.iter().find_position(|n| n.len() == 7) {
            digit_map.insert(8, eight.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //3 has five segments and contains both segments of 1
        if let Some((i, three)) = remaining_numbers.iter().find_position(|n| {
            n.len() == 5 && digit_map.get(&1).unwrap().iter().all(|c| n.contains(c))
        }) {
            digit_map.insert(3, three.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //6 has six segments and doesn't contain both segments of 1
        if let Some((i, six)) = remaining_numbers.iter().find_position(|n| {
            n.len() == 6 && !digit_map.get(&1).unwrap().iter().all(|c| n.contains(c))
        }) {
            digit_map.insert(6, six.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //Build the difference between 4 and 1, four_minus_one
        let mut four_minus_one = digit_map.get(&4).unwrap().clone();
        four_minus_one.retain(|c| !digit_map.get(&1).unwrap().contains(c));

        //0 has six segments and doesn't contain both segments of four_minus_one
        if let Some((i, zero)) = remaining_numbers
            .iter()
            .find_position(|n| n.len() == 6 && !four_minus_one.iter().all(|c| n.contains(c)))
        {
            digit_map.insert(0, zero.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //The only remaining number with six segment is 9
        if let Some((i, nine)) = remaining_numbers.iter().find_position(|n| n.len() == 6) {
            digit_map.insert(9, nine.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //5 is the only remaining number containing both segments of four_minus_one
        if let Some((i, five)) = remaining_numbers
            .iter()
            .find_position(|n| four_minus_one.iter().all(|c| n.contains(c)))
        {
            digit_map.insert(5, five.iter().copied().sorted().collect());
            remaining_numbers.swap_remove(i);
        }
        //2 is the last number remaining
        if let Some(mut two) = remaining_numbers.pop() {
            two.sort();
            digit_map.insert(2, two);
        }

        digit_map.into_iter().map(|(k, v)| (v, k)).collect()
    }
    fn solve(&self) -> (usize, usize) {
        let digit_map: FxHashMap<Vec<char>, usize> = self.build_number_map();
        let easy_numbers: [usize; 4] = [1, 4, 7, 8];

        self.numbers.iter().fold((0, 0), |(mut easy, mut sum), n| {
            let sorted: Vec<char> = n.iter().copied().sorted().collect();
            let &v = digit_map.get(&sorted).unwrap();
            if easy_numbers.contains(&v) {
                easy += 1;
            }
            sum = (sum * 10) + v;
            (easy, sum)
        })
    }
}

impl FromStr for Display {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_display(s: &str) -> IResult<&str, Display> {
            let (s, dict) = separated_list1(char(' '), alpha1)(s)?;
            let (s, numbers) = preceded(tag(" | "), separated_list1(char(' '), alpha1))(s)?;

            let dict: Vec<Vec<char>> = dict.into_iter().map(|w| w.chars().collect()).collect();
            let numbers: Vec<Vec<char>> =
                numbers.into_iter().map(|w| w.chars().collect()).collect();

            Ok((s, Display { dict, numbers }))
        }

        Ok(parse_display(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct SevenSegment {
    displays: Vec<Display>,
}

impl SevenSegment {
    fn solve(&self) -> (usize, usize) {
        self.displays
            .iter()
            .map(|d| d.solve())
            .fold((0, 0), |(easy, sum), (e, s)| (easy + e, sum + s))
    }
}

impl FromStr for SevenSegment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let displays: Vec<Display> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(SevenSegment { displays })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_08.txt").expect("Cannot open input file");
    let display: SevenSegment = s.parse().unwrap();

    let (easy_digits, total_sum) = display.solve();
    println!("Part1: The numbers 1, 4, 7 and 8 can be found {easy_digits} times");
    println!("Part2: The total sum of all displayed numbers is {total_sum}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str =
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce
";

    #[test]
    fn part_1() {
        let display: SevenSegment = EXAMPLE_1.parse().unwrap();
        assert_eq!((26, 61229), display.solve());
    }
}
