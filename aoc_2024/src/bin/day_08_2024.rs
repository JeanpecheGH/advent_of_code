use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::str::FromStr;
use util::coord::PosI;

struct Antennas {
    antennas: FxHashMap<char, Vec<PosI>>,
    max_x: isize,
    max_y: isize,
}

impl Antennas {
    fn nb_antinodes(&self, harmonics: bool) -> usize {
        fn is_inbound(PosI(x, y): PosI, max_x: isize, max_y: isize) -> bool {
            (0..max_x).contains(&x) && (0..max_y).contains(&y)
        }
        fn anti(
            &PosI(x, y): &PosI,
            &PosI(w, z): &PosI,
            max_x: isize,
            max_y: isize,
            harmonics: bool,
        ) -> Vec<PosI> {
            let x_diff: isize = x - w;
            let y_diff: isize = y - z;
            let pos_diff: PosI = PosI(x_diff, y_diff);

            let mut v: Vec<PosI> = Vec::new();

            if harmonics {
                let mut p = PosI(x, y);
                v.push(p);

                p = p.add(pos_diff);
                while is_inbound(p, max_x, max_y) {
                    v.push(p);
                    p = p.add(pos_diff);
                }

                p = PosI(x, y).sub(pos_diff);
                while is_inbound(p, max_x, max_y) {
                    v.push(p);
                    p = p.sub(pos_diff);
                }
            } else {
                let p: PosI = PosI(x, y).add(pos_diff);
                if is_inbound(p, max_x, max_y) {
                    v.push(p);
                }
                let p: PosI = PosI(x, y).sub(pos_diff).sub(pos_diff);
                if is_inbound(p, max_x, max_y) {
                    v.push(p);
                }
            }

            v
        }

        let antinodes: FxHashSet<PosI> = self
            .antennas
            .iter()
            .flat_map(|(_, v)| {
                v.iter()
                    .combinations(2)
                    // Compute all antinodes for a given pair of same-frequency antennas
                    .flat_map(|pair| anti(pair[0], pair[1], self.max_x, self.max_y, harmonics))
            })
            .collect();

        antinodes.len()
    }
}

impl FromStr for Antennas {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut antennas: FxHashMap<char, Vec<PosI>> = FxHashMap::default();
        let lines: Vec<&str> = s.lines().collect();
        let max_y = lines.len() as isize;
        let max_x = lines[0].len() as isize;

        lines.iter().enumerate().for_each(|(y, l)| {
            l.chars()
                .enumerate()
                .filter(|&(_, c)| c != '.')
                .for_each(|(x, c)| {
                    antennas
                        .entry(c)
                        .or_default()
                        .push(PosI(x as isize, y as isize))
                })
        });

        Ok(Antennas {
            antennas,
            max_x,
            max_y,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_08.txt").expect("Cannot open input file");
    let antennas: Antennas = s.parse().unwrap();
    println!(
        "Part1: There are {} antinodes",
        antennas.nb_antinodes(false)
    );
    println!(
        "Part2: Taking resonant harmonics into account, there are {} antinodes",
        antennas.nb_antinodes(true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............
";

    #[test]
    fn part_1() {
        let antennas: Antennas = EXAMPLE_1.parse().unwrap();
        assert_eq!(antennas.nb_antinodes(false), 14);
    }

    #[test]
    fn part_2() {
        let antennas: Antennas = EXAMPLE_1.parse().unwrap();
        assert_eq!(antennas.nb_antinodes(true), 34);
    }
}
