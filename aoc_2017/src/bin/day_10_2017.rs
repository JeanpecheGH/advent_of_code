use itertools::Itertools;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use std::fmt::Write;
use std::str::FromStr;
use util::basic_parser::parse_usize;

const SIZE: usize = 256;

#[derive(Debug, Clone)]
struct KnotHash {
    lengths: Vec<usize>,
}

impl KnotHash {
    fn hash(&self, len: usize) -> usize {
        let mut v: Vec<usize> = (0..len).collect();

        let mut total_rot: usize = 0;

        for (skip, &l) in self.lengths.iter().enumerate() {
            let slice: &mut [usize] = &mut v[0..l];
            slice.reverse();
            let rot: usize = (l + skip) % len;
            total_rot = (total_rot + rot) % len;
            v.rotate_left(rot);
        }

        //Rotate the vector back to its initial position
        v.rotate_left(len - total_rot);

        v[0] * v[1]
    }

    fn dense_hash(&self) -> String {
        let mut lengths: Vec<u8> = self
            .lengths
            .iter()
            .map(|n| n.to_string())
            .join(",")
            .chars()
            .map(|c| c as u8)
            .collect();
        lengths.extend(vec![17, 31, 73, 47, 23]);

        let mut v: Vec<u8> = (0..SIZE).map(|n| n as u8).collect();

        let mut skip: usize = 0;
        let mut total_rot: usize = 0;

        for _ in 0..64 {
            for &l in &lengths {
                let slice: &mut [u8] = &mut v[0..l as usize];
                slice.reverse();
                let rot: usize = (l as usize + skip) % SIZE;
                total_rot = (total_rot + rot) % SIZE;
                v.rotate_left(rot);
                skip += 1;
            }
        }

        //Rotate the vector back to its initial position
        v.rotate_left(SIZE - total_rot);

        //v is the sparse hash, densify it
        v.chunks(16)
            .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
            .fold(String::new(), |mut output, b| {
                let _ = write!(output, "{b:02x}");
                output
            })
    }
}

impl FromStr for KnotHash {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_lengths(s: &str) -> IResult<&str, Vec<usize>> {
            separated_list1(char(','), parse_usize)(s)
        }

        let lengths: Vec<usize> = s
            .lines()
            .next()
            .map(|l| parse_lengths(l).unwrap().1)
            .unwrap();
        Ok(KnotHash { lengths })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_10.txt").expect("Cannot open input file");
    let khash: KnotHash = s.parse().unwrap();
    println!("Part1: The simple hash method gives {}", khash.hash(SIZE));
    println!(
        "Part2: When using the complete hash method, the result is {}",
        khash.dense_hash()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "3,4,1,5";
    const EXAMPLE_2: &str = "1,2,3";
    const EXAMPLE_3: &str = "1,2,4";

    #[test]
    fn part_1() {
        let khash: KnotHash = EXAMPLE_1.parse().unwrap();
        assert_eq!(12, khash.hash(5));
    }

    #[test]
    fn part_2_test_1() {
        let khash: KnotHash = EXAMPLE_2.parse().unwrap();
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", khash.dense_hash());
    }

    #[test]
    fn part_2_test_2() {
        let khash: KnotHash = EXAMPLE_3.parse().unwrap();
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", khash.dense_hash());
    }
}
