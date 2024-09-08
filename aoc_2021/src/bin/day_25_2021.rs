use std::str::FromStr;
use util::coord::Pos;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SeaCucumber {
    East,
    South,
    None,
}
impl SeaCucumber {
    fn from_char(c: char) -> Self {
        match c {
            '>' => SeaCucumber::East,
            'v' => SeaCucumber::South,
            _ => SeaCucumber::None,
        }
    }

    fn move_to(&self, Pos(x, y): Pos, max: usize) -> Pos {
        match self {
            SeaCucumber::East => Pos((x + 1) % max, y),
            SeaCucumber::South => Pos(x, (y + 1) % max),
            SeaCucumber::None => Pos(x, y),
        }
    }
}

#[derive(Debug, Clone)]
struct SeaCucumberHerd {
    grid: Vec<Vec<SeaCucumber>>,
}

impl SeaCucumberHerd {
    fn last_move(&self) -> usize {
        let mut current_grid: Vec<Vec<SeaCucumber>> = self.grid.clone();
        let x_max: usize = current_grid[0].len();
        let y_max: usize = current_grid.len();
        let mut step: usize = 0;
        let mut nb_move: usize = usize::MAX;

        while nb_move > 0 {
            nb_move = 0;
            step += 1;
            let mut temp_grid: Vec<Vec<SeaCucumber>> = vec![vec![SeaCucumber::None; x_max]; y_max];
            //Move East cucumbers
            current_grid.iter().enumerate().for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == SeaCucumber::East)
                    .for_each(|(x, cucumber)| {
                        let Pos(new_x, new_y) = cucumber.move_to(Pos(x, y), x_max);
                        if current_grid[new_y][new_x] == SeaCucumber::None {
                            nb_move += 1;
                            temp_grid[new_y][new_x] = SeaCucumber::East
                        } else {
                            temp_grid[y][x] = SeaCucumber::East
                        }
                    })
            });

            //Move South cucumbers
            current_grid.iter().enumerate().for_each(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(_, c)| **c == SeaCucumber::South)
                    .for_each(|(x, cucumber)| {
                        let Pos(new_x, new_y) = cucumber.move_to(Pos(x, y), y_max);
                        if current_grid[new_y][new_x] != SeaCucumber::South
                            && temp_grid[new_y][new_x] == SeaCucumber::None
                        {
                            nb_move += 1;
                            temp_grid[new_y][new_x] = SeaCucumber::South
                        } else {
                            temp_grid[y][x] = SeaCucumber::South
                        }
                    })
            });
            current_grid = temp_grid;
        }

        step
    }
}

impl FromStr for SeaCucumberHerd {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<SeaCucumber>> = s
            .lines()
            .map(|l| l.chars().map(SeaCucumber::from_char).collect())
            .collect();
        Ok(SeaCucumberHerd { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_25.txt").expect("Cannot open input file");
    let herd: SeaCucumberHerd = s.parse().unwrap();
    println!(
        "Part1: The sea cucumbers stop moving after {} steps",
        herd.last_move()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "v...>>.vv>
.vv>>.vv..
>>.>v>...v
>>v>>.>.v.
v>v.vv.v..
>.>>..v...
.vv..>.>v.
v.v..>>v.v
....v..v.>
";

    #[test]
    fn part_1() {
        let herd: SeaCucumberHerd = EXAMPLE_1.parse().unwrap();
        assert_eq!(58, herd.last_move());
    }
}
