use nom::IResult;
use nom::Parser;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use std::str::FromStr;
use util::basic_parser::parse_usize;

struct GiftShop {
    ranges: Vec<(usize, usize)>,
}

impl GiftShop {
    fn is_invalid(n: usize, half_only: bool) -> bool {
        fn block_repeat(n: usize, block_size: usize) -> bool {
            let mut rest: usize = n;
            let div: usize = 10usize.pow(block_size as u32);
            let elem: usize = rest % div;
            rest /= div;
            while rest > 0 {
                if (rest % div) != elem {
                    return false;
                }
                rest /= div;
            }
            true
        }

        let size: usize = (n.ilog10() as usize) + 1;
        if half_only {
            size.is_multiple_of(2) && block_repeat(n, size / 2)
        } else {
            (1..=size / 2).any(|i| size.is_multiple_of(i) && block_repeat(n, i))
        }
    }

    fn solve(&self, half_only: bool) -> usize {
        self.ranges
            .iter()
            .map(|&(a, b)| {
                (a..=b)
                    .filter(|&x| GiftShop::is_invalid(x, half_only))
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for GiftShop {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_ranges(s: &str) -> IResult<&str, Vec<(usize, usize)>> {
            separated_list1(
                char(','),
                separated_pair(parse_usize, char('-'), parse_usize),
            )
            .parse(s)
        }

        let ranges: Vec<(usize, usize)> = parse_ranges(s.lines().next().unwrap()).unwrap().1;
        Ok(GiftShop { ranges })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_02.txt").expect("Cannot open input file");
    let shop: GiftShop = s.parse().unwrap();

    println!(
        "Part1: The sum of all the invalid IDs is {}",
        shop.solve(true)
    );
    println!(
        "Part2: Using the new rules, the sum of all the invalid IDs is {}",
        shop.solve(false)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
    #[test]
    fn test() {
        let shop: GiftShop = EXAMPLE_1.parse().unwrap();
        assert_eq!(shop.solve(true), 1227775554);
        assert_eq!(shop.solve(false), 4174379265);
    }
}
