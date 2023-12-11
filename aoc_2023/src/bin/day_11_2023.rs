use itertools::Itertools;
use std::str::FromStr;
use util::coord::Pos;

struct Cosmos {
    galaxies: Vec<Vec<bool>>,
}

impl Cosmos {
    fn short_path_sum(&self, expansion: usize) -> usize {
        //Get all galaxies coord
        let coords: Vec<Pos> = self
            .galaxies
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &is_gal)| if is_gal { Some(Pos(x, y)) } else { None })
                    .collect::<Vec<Pos>>()
            })
            .collect();

        //Get empty columns
        let empty_col: Vec<usize> = (0..self.galaxies[0].len())
            .filter(|&col| coords.iter().all(|&Pos(x, _)| x != col))
            .collect();
        //emtpy rows
        let empty_row: Vec<usize> = (0..self.galaxies.len())
            .filter(|&row| coords.iter().all(|&Pos(_, y)| y != row))
            .collect();

        coords
            .into_iter()
            .map(|Pos(x, y)| {
                //Expand coordinates
                let x_expand: usize = empty_col.iter().filter(|&&col| col < x).count();
                let y_expand: usize = empty_row.iter().filter(|&&row| row < y).count();
                Pos(
                    x + x_expand * (expansion - 1),
                    y + y_expand * (expansion - 1),
                )
            })
            .combinations(2)
            .map(|pair| pair[0].distance(pair[1])) //Compute pair distance
            .sum()
    }
}

impl FromStr for Cosmos {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let galaxies: Vec<Vec<bool>> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect::<Vec<bool>>())
            .collect();
        Ok(Cosmos { galaxies })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_11.txt").expect("Cannot open input file");
    let cosmos: Cosmos = s.parse().unwrap();

    println!(
        "Part1: After 2 years, the sum of distances is {}",
        cosmos.short_path_sum(2)
    );
    println!(
        "Part2: After 1000000 years, the sum of distances is {}",
        cosmos.short_path_sum(1000000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....
";

    #[test]
    fn part_1() {
        let cosmos: Cosmos = EXAMPLE_1.parse().unwrap();
        assert_eq!(cosmos.short_path_sum(2), 374);
    }
    #[test]
    fn part_2_test_1() {
        let cosmos: Cosmos = EXAMPLE_1.parse().unwrap();
        assert_eq!(cosmos.short_path_sum(10), 1030);
    }
    #[test]
    fn part_2_test_2() {
        let cosmos: Cosmos = EXAMPLE_1.parse().unwrap();
        assert_eq!(cosmos.short_path_sum(100), 8410);
    }
}
