use std::str::FromStr;

#[derive(Debug, Clone)]
struct GarbageStream {
    stream: Vec<char>,
}

impl GarbageStream {
    fn scores(&self) -> (usize, usize) {
        let mut score: usize = 0;
        let mut removed_garbage: usize = 0;
        let mut depth: usize = 0;
        let mut in_garbage: bool = false;

        let mut it = self.stream.iter();

        while let Some(c) = it.next() {
            match (c, in_garbage) {
                ('{', false) => depth += 1,
                ('}', false) => {
                    score += depth;
                    depth -= 1;
                }
                ('<', false) => in_garbage = true,
                ('>', true) => in_garbage = false,
                ('!', true) => {
                    let _ = it.next();
                }
                (_, true) => removed_garbage += 1,
                (_, false) => (),
            }
        }
        (score, removed_garbage)
    }
}

impl FromStr for GarbageStream {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let stream: Vec<char> = s.lines().next().map(|l| l.chars()).unwrap().collect();
        Ok(GarbageStream { stream })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_09.txt").expect("Cannot open input file");
    let stream: GarbageStream = s.parse().unwrap();
    let (score, garbage) = stream.scores();
    println!("Part1: The total group score is {score}");
    println!("Part2: We removed {garbage} garbage characters");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "{}";
    const EXAMPLE_2: &str = "{{{}}}";
    const EXAMPLE_3: &str = "{{},{}}";
    const EXAMPLE_4: &str = "{{{},{},{{}}}}";
    const EXAMPLE_5: &str = "{<a>,<a>,<a>,<a>}";
    const EXAMPLE_6: &str = "{{<ab>},{<ab>},{<ab>},{<ab>}}";
    const EXAMPLE_7: &str = "{{<!!>},{<!!>},{<!!>},{<!!>}}";
    const EXAMPLE_8: &str = "{{<a!>},{<a!>},{<a!>},{<ab>}}";
    const EXAMPLE_9: &str = "<{o\"i!a,<{i<a>";

    #[test]
    fn part_1_test_1() {
        let stream: GarbageStream = EXAMPLE_1.parse().unwrap();
        assert_eq!(1, stream.scores().0);
    }
    #[test]
    fn part_1_test_2() {
        let stream: GarbageStream = EXAMPLE_2.parse().unwrap();
        assert_eq!(6, stream.scores().0);
    }
    #[test]
    fn part_1_test_3() {
        let stream: GarbageStream = EXAMPLE_3.parse().unwrap();
        assert_eq!(5, stream.scores().0);
    }
    #[test]
    fn part_1_test_4() {
        let stream: GarbageStream = EXAMPLE_4.parse().unwrap();
        assert_eq!(16, stream.scores().0);
    }
    #[test]
    fn part_1_test_5() {
        let stream: GarbageStream = EXAMPLE_5.parse().unwrap();
        assert_eq!(1, stream.scores().0);
    }
    #[test]
    fn part_1_test_6() {
        let stream: GarbageStream = EXAMPLE_6.parse().unwrap();
        assert_eq!(9, stream.scores().0);
    }
    #[test]
    fn part_1_test_7() {
        let stream: GarbageStream = EXAMPLE_7.parse().unwrap();
        assert_eq!(9, stream.scores().0);
    }
    #[test]
    fn part_1_test_8() {
        let stream: GarbageStream = EXAMPLE_8.parse().unwrap();
        assert_eq!(3, stream.scores().0);
    }
    #[test]
    fn part_2_test_1() {
        let stream: GarbageStream = EXAMPLE_9.parse().unwrap();
        assert_eq!(10, stream.scores().1);
    }
}
