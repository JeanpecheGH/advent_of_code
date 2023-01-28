struct Round {
    you: i16,
    result: i16,
}
impl Round {
    fn from_plays(opp: i16, you: i16) -> Self {
        let result: i16 = ((you - opp) % 3 + 4) % 3;
        Round { you, result }
    }
    fn from_result(opp: i16, result: i16) -> Self {
        let you: i16 = (opp + (result - 1) + 3) % 3;
        Round { you, result }
    }

    fn score(&self) -> i16 {
        self.result * 3 + self.you + 1
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_02.txt").expect("Cannot open input file");

    let pairs: Vec<(i16, i16)> = s
        .lines()
        .map(|s| {
            let chars: Vec<u8> = s.bytes().collect();
            ((chars[0] - b'A') as i16, (chars[2] - b'X') as i16)
        })
        .collect();

    let part1_score: i16 = pairs
        .iter()
        .map(|&(opp, you)| Round::from_plays(opp, you).score())
        .sum();
    println!("Part1: When summing the rounds, the total score is {part1_score}");
    let part2_score: i16 = pairs
        .iter()
        .map(|&(opp, result)| Round::from_result(opp, result).score())
        .sum();
    println!("Part2: When following the actual strategy, the score is {part2_score}");
}
