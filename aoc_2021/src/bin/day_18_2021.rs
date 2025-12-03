use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
enum SnailNumber {
    Node(Box<SnailNumber>, Box<SnailNumber>),
    Leave(usize),
}

impl SnailNumber {
    fn add(&mut self, other: SnailNumber) {
        *self = SnailNumber::Node(Box::new(self.clone()), Box::new(other));
        self.reduce();
    }

    fn value(&self) -> usize {
        match self {
            SnailNumber::Node(_, _) => 0,
            SnailNumber::Leave(v) => *v,
        }
    }

    fn split(&mut self) -> bool {
        match self {
            //Split right only if left didn't
            SnailNumber::Node(l, r) => l.split() || r.split(),
            SnailNumber::Leave(v) if *v > 9 => {
                let d = *v / 2;
                *self = SnailNumber::Node(
                    Box::new(SnailNumber::Leave(d)),
                    Box::new(SnailNumber::Leave(*v - d)),
                );
                true
            }
            _ => false,
        }
    }

    fn explode(&mut self, depth: usize) -> (bool, (usize, usize)) {
        match self {
            SnailNumber::Node(l, r) if depth == 5 => {
                let val_l: usize = l.value();
                let val_r: usize = r.value();

                *self = SnailNumber::Leave(0);
                (true, (val_l, val_r))
            }
            SnailNumber::Node(l, r) => {
                let (explode_left, (val_l, val_r)) = l.explode(depth + 1);
                if explode_left {
                    r.add_left(val_r);
                    (true, (val_l, 0))
                } else {
                    let (explode_right, (val_l, val_r)) = r.explode(depth + 1);
                    if explode_right {
                        l.add_right(val_l);
                        (true, (0, val_r))
                    } else {
                        (false, (0, 0))
                    }
                }
            }
            SnailNumber::Leave(_) => (false, (0, 0)),
        }
    }

    fn add_left(&mut self, add: usize) {
        match self {
            SnailNumber::Node(l, _) => l.add_left(add),
            SnailNumber::Leave(v) => *v += add,
        }
    }

    fn add_right(&mut self, add: usize) {
        match self {
            SnailNumber::Node(_, r) => r.add_right(add),
            SnailNumber::Leave(v) => *v += add,
        }
    }

    fn reduce(&mut self) {
        while self.explode(1).0 || self.split() {
            //Nothing, the work is done in the condition
        }
    }
    fn magnitude(&self) -> usize {
        match self {
            SnailNumber::Node(l, r) => 3 * l.magnitude() + 2 * r.magnitude(),
            SnailNumber::Leave(v) => *v,
        }
    }

    #[allow(dead_code)]
    fn print(&self, root: bool) {
        match self {
            SnailNumber::Node(l, r) => {
                print!("[");
                l.print(false);
                print!(",");
                r.print(false);
                print!("]");
            }
            SnailNumber::Leave(v) => print!("{v}"),
        }
        if root {
            println!();
        }
    }
}

impl FromStr for SnailNumber {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_number(s: &str) -> IResult<&str, SnailNumber> {
            if let Ok((s, v)) = parse_usize(s) {
                Ok((s, SnailNumber::Leave(v)))
            } else {
                let (s, _) = char('[')(s)?;
                let (s, (left, right)) =
                    separated_pair(parse_number, char(','), parse_number).parse(s)?;
                let (s, _) = char(']')(s)?;
                Ok((s, SnailNumber::Node(Box::new(left), Box::new(right))))
            }
        }
        Ok(parse_number(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct Snailfish {
    numbers: Vec<SnailNumber>,
}

impl Snailfish {
    fn sum_magnitude(&self) -> usize {
        self.numbers
            .clone()
            .into_iter()
            .reduce(|mut acc, other| {
                acc.add(other.clone());
                acc
            })
            .unwrap()
            .magnitude()
    }

    fn largest_magnitude(&self) -> usize {
        let mut max_magnitude: usize = 0;

        for i in 0..self.numbers.len() {
            for j in 0..self.numbers.len() {
                if i != j {
                    let mut left: SnailNumber = self.numbers[i].clone();
                    let right: SnailNumber = self.numbers[j].clone();
                    left.add(right);
                    if left.magnitude() > max_magnitude {
                        max_magnitude = left.magnitude();
                    }
                }
            }
        }
        max_magnitude
    }
}

impl FromStr for Snailfish {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let numbers: Vec<SnailNumber> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Snailfish { numbers })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_18.txt").expect("Cannot open input file");
    let snailfish: Snailfish = s.parse().unwrap();
    println!(
        "Part1: The magnitude of the sum of all numbers is {}",
        snailfish.sum_magnitude()
    );
    println!(
        "Part2: The largest magnitude of the sum of 2 differents numbers is {}",
        snailfish.largest_magnitude()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]
[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]
[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]
[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]
[7,[5,[[3,8],[1,4]]]]
[[2,[2,2]],[8,[8,1]]]
[2,9]
[1,[[[9,3],9],[[9,0],[0,7]]]]
[[[5,[7,4]],7],1]
[[[[4,2],2],6],[8,7]]
";

    const EXAMPLE_2: &str = "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]
[[[5,[2,8]],4],[5,[[9,9],0]]]
[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]
[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]
[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]
[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]
[[[[5,4],[7,7]],8],[[8,3],8]]
[[9,3],[[9,9],[6,[4,9]]]]
[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]
[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]
";

    #[test]
    fn part_1_test_1() {
        let snailfish: Snailfish = EXAMPLE_1.parse().unwrap();
        assert_eq!(3488, snailfish.sum_magnitude());
    }

    #[test]
    fn part_1_test_2() {
        let snailfish: Snailfish = EXAMPLE_2.parse().unwrap();
        assert_eq!(4140, snailfish.sum_magnitude());
    }
    #[test]
    fn part_2() {
        let snailfish: Snailfish = EXAMPLE_2.parse().unwrap();
        assert_eq!(3993, snailfish.largest_magnitude());
    }
}
