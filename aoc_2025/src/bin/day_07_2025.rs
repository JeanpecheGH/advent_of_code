use fxhash::{FxHashMap, FxHashSet};
use std::str::FromStr;

struct TachyonManifold {
    source: usize,
    splitters: Vec<FxHashSet<usize>>,
}

impl TachyonManifold {
    fn process(&self) -> (usize, usize) {
        let mut beams: FxHashMap<usize, usize> = FxHashMap::default();
        beams.insert(self.source, 1);
        let mut nb_splits: usize = 0;

        for row in &self.splitters {
            let mut new_beams: FxHashMap<usize, usize> = FxHashMap::default();
            for (b, nb) in beams {
                if row.contains(&b) {
                    *new_beams.entry(b - 1).or_default() += nb;
                    *new_beams.entry(b + 1).or_default() += nb;
                    nb_splits += 1;
                } else {
                    *new_beams.entry(b).or_default() += nb;
                }
            }
            beams = new_beams;
        }
        (nb_splits, beams.values().sum())
    }
}

impl FromStr for TachyonManifold {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();

        let source: usize = lines[0].chars().position(|c| c == 'S').unwrap();
        let splitters: Vec<FxHashSet<usize>> = lines[1..]
            .iter()
            .map(|l| {
                l.chars()
                    .enumerate()
                    .filter_map(|(i, c)| if c == '^' { Some(i) } else { None })
                    .collect()
            })
            .filter(|set: &FxHashSet<usize>| !set.is_empty())
            .collect();

        Ok(TachyonManifold { source, splitters })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_07.txt").expect("Cannot open input file");
    let manifold: TachyonManifold = s.parse().unwrap();

    let (nb_splits, nb_timelines) = manifold.process();
    println!("Part1: The beam will be split {} times", nb_splits);
    println!("Part2: {} timelines will be created", nb_timelines);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............
";
    #[test]
    fn test_1() {
        let manifold: TachyonManifold = EXAMPLE_1.parse().unwrap();
        assert_eq!(manifold.process(), (21, 40));
    }
}
