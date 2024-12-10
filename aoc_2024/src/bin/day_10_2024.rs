use fxhash::FxHashSet;
use std::str::FromStr;
use util::coord::Pos;

struct TrailMap {
    grid: Vec<Vec<usize>>,
    max_x: usize,
    max_y: usize,
}

impl TrailMap {
    fn height_at(&self, &Pos(x, y): &Pos) -> usize {
        self.grid[y][x]
    }

    fn nb_trails(&self, start: Pos, tops: bool) -> usize {
        let mut nb_trails: usize = 0;

        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        let mut current_paths: Vec<Pos> = vec![start];

        while let Some(pos) = current_paths.pop() {
            let h: usize = self.height_at(&pos);
            if h == 9 {
                nb_trails += 1;
            } else {
                pos.neighbours_safe(self.max_x, self.max_y)
                    .iter()
                    .filter(|&n_pos| {
                        self.height_at(n_pos) == h + 1 && (!tops || visited.insert(*n_pos))
                    })
                    .for_each(|&n_pos| current_paths.push(n_pos))
            }
        }

        nb_trails
    }

    fn trailheads(&self) -> Vec<Pos> {
        self.grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, &height)| height == 0)
                    .map(|(x, _)| Pos(x, y))
                    .collect::<Vec<Pos>>()
            })
            .collect()
    }

    fn score(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&pos| self.nb_trails(pos, true))
            .sum()
    }

    fn rating(&self) -> usize {
        self.trailheads()
            .iter()
            .map(|&pos| self.nb_trails(pos, false))
            .sum()
    }
}

impl FromStr for TrailMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();

        let max_x: usize = grid[0].len();
        let max_y: usize = grid.len();
        Ok(TrailMap { grid, max_x, max_y })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_10.txt").expect("Cannot open input file");
    let trail: TrailMap = s.parse().unwrap();
    println!(
        "Part1: The sum of the scores of all trailheads is {}",
        trail.score()
    );
    println!(
        "Part2: The sum of the ratings of all trailheads is {}",
        trail.rating()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732
";

    #[test]
    fn part_1() {
        let trail: TrailMap = EXAMPLE_1.parse().unwrap();
        assert_eq!(trail.score(), 36);
    }

    #[test]
    fn part_2() {
        let trail: TrailMap = EXAMPLE_1.parse().unwrap();
        assert_eq!(trail.rating(), 81);
    }
}
