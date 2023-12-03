use std::collections::{HashMap, HashSet};
use std::str::FromStr;
use util::coord::PosI;

struct Gondolas {
    engine: Vec<Vec<char>>,
}

impl Gondolas {
    fn sums(&self) -> (usize, usize) {
        //Get the position and value of every symbol
        let ops_map: HashMap<PosI, char> = self.ops_map();

        //Get the list of numbers attached to each symbol
        let parts_map: HashMap<PosI, Vec<usize>> =
            self.parts_map(ops_map.keys().copied().collect::<HashSet<PosI>>());

        //We can sum everyone of these numbers for Part1
        let parts_sum: usize = parts_map.values().map(|v| v.iter().sum::<usize>()).sum();

        //For Part2, we filter the '*' symbols
        let gear_set: HashSet<PosI> = ops_map
            .into_iter()
            .filter_map(|(pos, c)| if c == '*' { Some(pos) } else { None })
            .collect();

        //Filter the parts_map to keep only the pair attached to a '*' and get the product of the 2
        let gears_ratio_sum: usize = parts_map
            .into_iter()
            .filter_map(|(pos, v)| {
                if gear_set.contains(&pos) && v.len() == 2 {
                    Some(v[0] * v[1])
                } else {
                    None
                }
            })
            .sum();

        (parts_sum, gears_ratio_sum)
    }

    fn ops_map(&self) -> HashMap<PosI, char> {
        let mut map: HashMap<PosI, char> = HashMap::new();

        for (y, row) in self.engine.iter().enumerate() {
            for (x, &c) in row.iter().enumerate() {
                match c {
                    '.' => (),
                    _ if c.is_ascii_digit() => (),
                    _ => {
                        let _ = map.insert(PosI(x as isize, y as isize), c);
                    }
                }
            }
        }
        map
    }

    fn parts_map(&self, ops_set: HashSet<PosI>) -> HashMap<PosI, Vec<usize>> {
        let mut parts_map: HashMap<PosI, Vec<usize>> = HashMap::new();

        for (y, row) in self.engine.iter().enumerate() {
            let mut x: usize = 0;
            while x < row.len() {
                let start: usize = x;
                let mut nb: usize = 0;
                while x < row.len() && row[x].is_ascii_digit() {
                    nb = nb * 10 + row[x].to_digit(10).unwrap() as usize;
                    //If this is the end of the number, we skip the following symbol or space
                    x += 1;
                }
                if nb > 0 {
                    let opt: Option<PosI> = (start..x).find_map(|i| {
                        let p: PosI = PosI(i as isize, y as isize);
                        let nbgs: Vec<PosI> = p.neighbours_diag();
                        nbgs.iter().find_map(|pos| {
                            if ops_set.contains(pos) {
                                Some(*pos)
                            } else {
                                None
                            }
                        })
                    });
                    if let Some(part) = opt {
                        let entry = parts_map.entry(part).or_default();
                        entry.push(nb);
                    }
                }
                x += 1;
            }
        }
        parts_map
    }
}

impl FromStr for Gondolas {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let engine: Vec<Vec<char>> = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        Ok(Gondolas { engine })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_03.txt").expect("Cannot open input file");
    let gondolas: Gondolas = s.parse().unwrap();

    let (parts_sum, gear_ratio_sum): (usize, usize) = gondolas.sums();

    println!(
        "Part1: the sum of all of the part numbers in the engine is {}",
        parts_sum
    );
    println!(
        "Part2: the sum of all of the gear ratios in the engine is {}",
        gear_ratio_sum
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..
";

    #[test]
    fn part_1() {
        let gondolas: Gondolas = EXAMPLE_1.parse().unwrap();
        assert_eq!(gondolas.sums(), (4361, 467835));
    }
}
