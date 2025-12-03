use fxhash::FxHashSet;
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::sequence::separated_pair;
use nom::IResult;
use nom::Parser;
use std::ops::RangeInclusive;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;

#[derive(Debug, Clone)]
struct HydrothermalLine {
    start: Pos,
    end: Pos,
}

impl HydrothermalLine {
    fn is_vertical(&self) -> bool {
        self.start.0 == self.end.0
    }
    fn is_horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn x_range(&self) -> Box<dyn Iterator<Item = usize>>
    where
        RangeInclusive<usize>: Iterator<Item = usize> + DoubleEndedIterator,
    {
        if self.start.0 < self.end.0 {
            Box::new(self.start.0..=self.end.0)
        } else {
            Box::new((self.end.0..=self.start.0).rev())
        }
    }

    fn y_range(&self) -> Box<dyn Iterator<Item = usize>>
    where
        RangeInclusive<usize>: Iterator<Item = usize> + DoubleEndedIterator,
    {
        if self.start.1 < self.end.1 {
            Box::new(self.start.1..=self.end.1)
        } else {
            Box::new((self.end.1..=self.start.1).rev())
        }
    }

    fn points(&self) -> Vec<Pos> {
        if self.is_vertical() {
            self.y_range().map(|y| Pos(self.start.0, y)).collect()
        } else if self.is_horizontal() {
            self.x_range().map(|x| Pos(x, self.start.1)).collect()
        } else {
            self.x_range()
                .zip(self.y_range())
                .map(|(x, y)| Pos(x, y))
                .collect()
        }
    }
}

impl FromStr for HydrothermalLine {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos(s: &str) -> IResult<&str, Pos> {
            let (s, (a, b)) = separated_pair(parse_usize, char(','), parse_usize).parse(s)?;

            Ok((s, Pos(a, b)))
        }

        fn parse_line(s: &str) -> IResult<&str, HydrothermalLine> {
            let (s, (start, end)) = separated_pair(parse_pos, tag(" -> "), parse_pos).parse(s)?;

            Ok((s, HydrothermalLine { start, end }))
        }

        Ok(parse_line(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct HydrothermalVents {
    vents: Vec<HydrothermalLine>,
}

impl HydrothermalVents {
    fn overlap(&self, with_diag: bool) -> usize {
        let mut plot_set: FxHashSet<Pos> = FxHashSet::default();
        let mut overlap_set: FxHashSet<Pos> = FxHashSet::default();

        for v in &self.vents {
            if with_diag || (v.is_vertical() || v.is_horizontal()) {
                v.points().into_iter().for_each(|p| {
                    if !plot_set.insert(p) {
                        overlap_set.insert(p);
                    }
                })
            }
        }

        overlap_set.len()
    }
}

impl FromStr for HydrothermalVents {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vents: Vec<HydrothermalLine> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(HydrothermalVents { vents })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_05.txt").expect("Cannot open input file");
    let vents: HydrothermalVents = s.parse().unwrap();

    println!(
        "Part1: When taking horizontal and vertical lines, {} points are overlapping",
        vents.overlap(false)
    );
    println!(
        "Part2: When adding the diagonal lines, {} points are overlapping",
        vents.overlap(true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2
";

    #[test]
    fn part_1() {
        let vents: HydrothermalVents = EXAMPLE_1.parse().unwrap();
        assert_eq!(5, vents.overlap(false));
    }
    #[test]
    fn part_2() {
        let vents: HydrothermalVents = EXAMPLE_1.parse().unwrap();
        assert_eq!(12, vents.overlap(true));
    }
}
