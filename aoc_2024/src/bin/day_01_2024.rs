use nom::bytes::complete::tag;
use nom::sequence::separated_pair;
use nom::IResult;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;

struct Locations {
    pairs: Vec<(usize, usize)>,
}

impl Locations {
    fn solve(&self) -> (usize, usize) {
        let mut left: Vec<usize> = self.pairs.iter().map(|&(a, _)| a).collect();
        let mut right: Vec<usize> = self.pairs.iter().map(|&(_, b)| b).collect();

        left.sort_unstable();
        right.sort_unstable();

        (
            Self::distance(&left, &right),
            Self::similarity(&left, &right),
        )
    }
    fn distance(left: &[usize], right: &[usize]) -> usize {
        left.iter().zip(right).map(|(&a, &b)| a.abs_diff(b)).sum()
    }

    //The lists are sorted, we can pop the right when we find numbers that are too low
    fn similarity(left: &[usize], right: &[usize]) -> usize {
        fn nb_equal(n: &usize, right: &mut Vec<usize>) -> usize {
            let mut count = 0;
            while let Some(r) = right.pop() {
                match r.cmp(n) {
                    Ordering::Less => (),
                    Ordering::Equal => count += 1,
                    Ordering::Greater => {
                        right.push(r);
                        break;
                    }
                }
            }
            count
        }
        let mut rev_right: Vec<usize> = right.iter().rev().copied().collect();

        left.iter().map(|n| n * nb_equal(n, &mut rev_right)).sum()
    }
}

impl FromStr for Locations {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (usize, usize)> {
            let (s, (a, b)) = separated_pair(parse_usize, tag("   "), parse_usize)(s)?;

            Ok((s, (a, b)))
        }
        let pairs: Vec<(usize, usize)> = s.lines().map(|l| parse_pair(l).unwrap().1).collect();

        Ok(Locations { pairs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_01.txt").expect("Cannot open input file");
    let locations: Locations = s.parse().unwrap();

    let (dist, sim) = locations.solve();

    println!("Part1: The total distance between the lists is {}", dist);
    println!("Part2: The similarity score of the lists is {}", sim);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "3   4
4   3
2   5
1   3
3   9
3   3
";
    #[test]
    fn test() {
        let locations: Locations = EXAMPLE_1.parse().unwrap();
        assert_eq!(locations.solve(), (11, 31));
    }
}
