use std::str::FromStr;
use util::coord::Pos;
use util::split_blocks;

struct AlmanacRange {
    source: usize,
    destination: usize,
    range: usize,
}

impl AlmanacRange {
    fn dest(&self, seed: usize) -> Option<usize> {
        let s: usize = self.source;
        if (s..(s + self.range)).contains(&seed) {
            Some(self.destination + (seed - s))
        } else {
            None
        }
    }

    //Returns an optional transformed range if the input range overlaps with the AlmanacRange
    //Return 0, 1 or 2 ranges in a Vec for parts not overlapping with the AlmanacRange
    fn dest_range(&self, pos: Pos) -> (Option<Pos>, Vec<Pos>) {
        let dest: usize = self.destination;
        match (pos.0, pos.1, self.source, self.source + self.range) {
            //All range is outside (either inferior or superior)
            (start, end, s, e) if end < s || start >= e => (None, vec![pos]),
            //ALl range is inside
            (start, end, s, e) if start >= s && end <= e => {
                (Some(Pos(dest + (start - s), dest + (end - s))), Vec::new())
            }
            //Part before, part inside
            (start, end, s, e) if start < s && end <= e => {
                (Some(Pos(dest, dest + (end - s))), vec![Pos(start, s)])
            }
            //Part inside, part after
            (start, end, s, e) if start >= s && end > e => (
                Some(Pos(dest + (start - s), dest + (e - s))),
                vec![Pos(e, end)],
            ),
            //Part before, part inside, part after
            (start, end, s, e) if start < s && end > e => (
                Some(Pos(dest, dest + (e - s))),
                vec![Pos(start, s), Pos(e, end)],
            ),
            //Should never happen
            _ => (None, Vec::new()),
        }
    }
}

impl FromStr for AlmanacRange {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let values: Vec<usize> = s.split_whitespace().map(|n| n.parse().unwrap()).collect();
        Ok(AlmanacRange {
            source: values[1],
            destination: values[0],
            range: values[2],
        })
    }
}
struct AlmanacMap {
    ranges: Vec<AlmanacRange>,
}

impl AlmanacMap {
    fn dest(&self, seed: usize) -> usize {
        let opt: Option<usize> =
            self.ranges.iter().fold(
                None,
                |acc, range| {
                    if acc.is_some() {
                        acc
                    } else {
                        range.dest(seed)
                    }
                },
            );

        opt.unwrap_or(seed)
    }

    fn dest_vec(&self, ranges: Vec<Pos>) -> Vec<Pos> {
        ranges
            .into_iter()
            .flat_map(|range| self.dest_range(range))
            .collect()
    }

    fn dest_range(&self, seed_range: Pos) -> Vec<Pos> {
        let (mut to_do, mut done): (Vec<Pos>, Vec<Pos>) = self.ranges.iter().fold(
            (vec![seed_range], Vec::new()),
            |(todo_acc, mut done_acc), range| {
                let mut new_todo: Vec<Pos> = Vec::new();
                for todo in todo_acc {
                    let (done_opt, mut not_done) = range.dest_range(todo);
                    if let Some(done) = done_opt {
                        done_acc.push(done);
                    }
                    new_todo.append(&mut not_done)
                }
                (new_todo, done_acc)
            },
        );

        to_do.append(&mut done);
        to_do
    }
}
impl FromStr for AlmanacMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ranges: Vec<AlmanacRange> = s.lines().skip(1).map(|l| l.parse().unwrap()).collect();
        Ok(AlmanacMap { ranges })
    }
}

struct Almanac {
    seeds: Vec<usize>,
    maps: Vec<AlmanacMap>,
}

impl Almanac {
    fn apply_maps(&self, seed: usize) -> usize {
        let mut loc: usize = seed;
        for m in self.maps.iter() {
            loc = m.dest(loc);
        }
        loc
    }
    fn lowest_location(&self) -> usize {
        self.seeds
            .iter()
            .copied()
            .map(|seed| self.apply_maps(seed))
            .min()
            .unwrap()
    }
    fn apply_range_maps(&self, ranges: Vec<Pos>) -> Vec<Pos> {
        let mut locs: Vec<Pos> = ranges;
        for m in self.maps.iter() {
            locs = m.dest_vec(locs);
        }
        locs
    }

    fn lowest_range_location(&self) -> usize {
        self.seeds
            .chunks(2)
            .map(|chunk| vec![Pos(chunk[0], chunk[0] + chunk[1])])
            .map(|v| self.apply_range_maps(v))
            .flat_map(|v| v.into_iter().map(|pos| pos.0).collect::<Vec<usize>>())
            .min()
            .unwrap()
    }
}

impl FromStr for Almanac {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<&str> = split_blocks(s);
        let seeds: Vec<usize> = blocks[0]
            .split_once(':')
            .unwrap()
            .1
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        let maps: Vec<AlmanacMap> = (1..=7).map(|n| blocks[n].parse().unwrap()).collect();

        Ok(Almanac { seeds, maps })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_05.txt").expect("Cannot open input file");
    let almanac: Almanac = s.parse().unwrap();

    println!(
        "Part1: The lowest location number for the given seeds is {}",
        almanac.lowest_location()
    );
    println!(
        "Part2: The lowest location number for the given range of seeds is {}",
        almanac.lowest_range_location()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4
";

    #[test]
    fn part_1() {
        let almanac: Almanac = EXAMPLE_1.parse().unwrap();
        assert_eq!(almanac.lowest_location(), 35);
    }
    #[test]
    fn part_2() {
        let almanac: Almanac = EXAMPLE_1.parse().unwrap();
        assert_eq!(almanac.lowest_range_location(), 46);
    }
}
