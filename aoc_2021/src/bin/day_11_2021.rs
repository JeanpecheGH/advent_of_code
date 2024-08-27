use std::str::FromStr;
use util::coord::Pos;

#[derive(Debug, Clone)]
struct DumboOctopus {
    octopuses: Vec<Vec<usize>>,
}

impl DumboOctopus {
    fn reset_flashed(&mut self) -> usize {
        let mut nb_flashed: usize = 0;

        for x in 0..10 {
            for y in 0..10 {
                if self.octopuses[y][x] == 10 {
                    self.octopuses[y][x] = 0;
                    nb_flashed += 1;
                }
            }
        }
        nb_flashed
    }

    fn step(&mut self) -> usize {
        for x in 0..10 {
            for y in 0..10 {
                //We only do something if this octopus was not already flashed
                if self.octopuses[y][x] < 10 {
                    self.octopuses[y][x] += 1;
                    //Propagate to neighbours
                    if self.octopuses[y][x] == 10 {
                        let mut ngbs: Vec<Pos> = Pos(x, y).neighbours_diag_safe(10, 10);
                        while let Some(Pos(n_x, n_y)) = ngbs.pop() {
                            //We also only do something if this octopus was not already flashed
                            if self.octopuses[n_y][n_x] < 10 {
                                self.octopuses[n_y][n_x] += 1;
                                if self.octopuses[n_y][n_x] == 10 {
                                    ngbs.extend(Pos(n_x, n_y).neighbours_diag_safe(10, 10))
                                }
                            }
                        }
                    }
                }
            }
        }

        self.reset_flashed()
    }
    fn flashing(&mut self) -> (usize, usize) {
        let mut hundred_steps: usize = 0;
        let mut steps: usize = 0;
        let mut nb_flashes: usize = 0;

        while nb_flashes < 100 {
            nb_flashes = self.step();
            steps += 1;
            if steps <= 100 {
                hundred_steps += nb_flashes;
            }
        }

        (hundred_steps, steps)
    }
}

impl FromStr for DumboOctopus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let octopuses: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        Ok(DumboOctopus { octopuses })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_11.txt").expect("Cannot open input file");
    let mut octopus: DumboOctopus = s.parse().unwrap();

    let (hundred_steps, all_flash) = octopus.flashing();
    println!("Part1: After 100 steps, {hundred_steps} flashes occured",);
    println!("Part2: All octopuses flash together after {all_flash} steps");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526
";

    #[test]
    fn part_1() {
        let mut octopus: DumboOctopus = EXAMPLE_1.parse().unwrap();
        assert_eq!((1656, 195), octopus.flashing());
    }
}
