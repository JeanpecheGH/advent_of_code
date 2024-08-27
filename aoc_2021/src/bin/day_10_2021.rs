use fxhash::FxHashMap;
use itertools::Itertools;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct SyntaxScoring {
    lines: Vec<Vec<char>>,
}

impl SyntaxScoring {
    fn parse_line_syntax(input: &[char], corrupted: bool) -> usize {
        let opens = ['(', '[', '{', '<'];
        let closes = [')', ']', '}', '>'];
        let scores: [usize; 4] = if corrupted {
            [3, 57, 1197, 25137]
        } else {
            [1, 2, 3, 4]
        };
        let pairs: FxHashMap<char, char> = closes.iter().copied().zip(opens).collect();
        let score_pairs: FxHashMap<char, usize> = if corrupted {
            closes.iter().copied().zip(scores).collect()
        } else {
            opens.iter().copied().zip(scores).collect()
        };
        let mut rest: Vec<char> = input.iter().copied().rev().collect();
        let mut buffer: Vec<char> = Vec::new();

        while let Some(c) = rest.pop() {
            if opens.contains(&c) {
                //We open a new block
                buffer.push(c);
            } else if closes.contains(&c) {
                if buffer.last() == pairs.get(&c) {
                    //Closing a well-formed block
                    buffer.pop();
                } else if corrupted {
                    //Returning score for a corrupted line
                    return score_pairs.get(&c).copied().unwrap();
                } else {
                    return 0;
                }
            }
        }

        if corrupted {
            0
        } else {
            buffer
                .iter()
                .rev()
                .fold(0, |acc, c| acc * 5 + score_pairs.get(c).copied().unwrap())
        }
    }
    fn error_score(&self) -> usize {
        self.lines
            .iter()
            .map(|l| SyntaxScoring::parse_line_syntax(l, true))
            .sum()
    }

    fn median_incomplete_score(&self) -> usize {
        let incomplete_scores: Vec<usize> = self
            .lines
            .iter()
            .filter_map(|l| {
                let s: usize = SyntaxScoring::parse_line_syntax(l, false);
                if s > 0 {
                    Some(s)
                } else {
                    None
                }
            })
            .sorted()
            .collect();
        incomplete_scores[incomplete_scores.len() / 2]
    }
}

impl FromStr for SyntaxScoring {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();
        Ok(SyntaxScoring { lines })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_10.txt").expect("Cannot open input file");
    let scoring: SyntaxScoring = s.parse().unwrap();
    println!(
        "Part1: The total syntax error score is {}",
        scoring.error_score()
    );
    println!(
        "Part2: The median incomplete score is {}",
        scoring.median_incomplete_score()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]
";

    #[test]
    fn part_1() {
        let scoring: SyntaxScoring = EXAMPLE_1.parse().unwrap();
        assert_eq!(26397, scoring.error_score());
    }

    #[test]
    fn part_2() {
        let scoring: SyntaxScoring = EXAMPLE_1.parse().unwrap();
        assert_eq!(288957, scoring.median_incomplete_score());
    }
}
