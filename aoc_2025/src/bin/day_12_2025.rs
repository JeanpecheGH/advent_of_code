use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::char;
use nom::character::complete::space1;
use nom::multi::separated_list1;
use nom::sequence::terminated;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Clone, Debug)]
struct Region {
    width: usize,
    length: usize,
    quantities: Vec<usize>,
}

impl Region {
    fn solved(&self) -> bool {
        // No optimization at all, each shape can take 9 squares
        let needed_space: usize = self.quantities.iter().sum::<usize>() * 9;
        let true_width: usize = self.width - self.width % 3;
        let true_length: usize = self.length - self.length % 3;
        let easy_space: usize = true_width * true_length;
        needed_space <= easy_space
    }
}

impl FromStr for Region {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_region(s: &str) -> IResult<&str, Region> {
            let (s, width) = terminated(parse_usize, char('x')).parse(s)?;
            let (s, length) = terminated(parse_usize, tag(": ")).parse(s)?;
            let (s, quantities) = separated_list1(space1, parse_usize).parse(s)?;

            Ok((
                s,
                Region {
                    width,
                    length,
                    quantities,
                },
            ))
        }

        Ok(parse_region(s).unwrap().1)
    }
}

struct TreeFarm {
    regions: Vec<Region>,
}

impl TreeFarm {
    fn fillable_regions(&self) -> usize {
        self.regions.iter().filter(|region| region.solved()).count()
    }
}

impl FromStr for TreeFarm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = split_blocks(s);
        // The shapes are useless to solve this problem, don't parse
        let regions: Vec<Region> = blocks[6].lines().map(|s| s.parse().unwrap()).collect();

        Ok(TreeFarm { regions })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_12.txt").expect("Cannot open input file");
    let farm: TreeFarm = s.parse().unwrap();

    println!(
        "Part1: {} regions can fit all the presents (without the need for any optimization) ",
        farm.fillable_regions()
    );
    println!("Computing time: {:?}", now.elapsed());
}
