use crate::basic_parser::parse_usize;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::IResult;
use nom::Parser;
use std::fmt::Write;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct KnotHash {
    lengths: Vec<u8>,
}

impl KnotHash {
    pub fn from_usize_list(l: &[usize]) -> KnotHash {
        KnotHash {
            lengths: l.iter().map(|n| *n as u8).collect(),
        }
    }
    pub fn new(s: &str) -> KnotHash {
        let lengths: Vec<u8> = s.chars().map(|c| c as u8).collect();
        KnotHash { lengths }
    }

    pub fn weak_hash(&self, len: usize) -> usize {
        let mut v: Vec<usize> = (0..len).collect();

        let mut total_rot: usize = 0;

        for (skip, &l) in self.lengths.iter().enumerate() {
            let slice: &mut [usize] = &mut v[0..l as usize];
            slice.reverse();
            let rot: usize = (l as usize + skip) % len;
            total_rot = (total_rot + rot) % len;
            v.rotate_left(rot);
        }

        //Rotate the vector back to its initial position
        v.rotate_left(len - total_rot);

        v[0] * v[1]
    }

    pub fn hash_vec(&self) -> Vec<u8> {
        let mut lengths: Vec<u8> = self.lengths.clone();
        lengths.extend(vec![17, 31, 73, 47, 23]);

        let mut v: Vec<u8> = (0..256).map(|n| n as u8).collect();

        let mut skip: usize = 0;
        let mut total_rot: usize = 0;

        for _ in 0..64 {
            for &l in &lengths {
                let slice: &mut [u8] = &mut v[0..l as usize];
                slice.reverse();
                let rot: usize = (l as usize + skip) % 256;
                total_rot = (total_rot + rot) % 256;
                v.rotate_left(rot);
                skip += 1;
            }
        }

        //Rotate the vector back to its initial position
        v.rotate_right(total_rot);

        //v is the sparse hash, densify it
        v.chunks(16)
            .map(|c| c.iter().copied().reduce(|a, b| a ^ b).unwrap())
            .collect()
    }

    pub fn hash(&self) -> String {
        self.hash_vec()
            .into_iter()
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
            separated_list1(char(','), parse_usize).parse(s)
        }

        let lengths: Vec<usize> = s
            .lines()
            .next()
            .map(|l| parse_lengths(l).unwrap().1)
            .unwrap();
        Ok(KnotHash::from_usize_list(&lengths))
    }
}
