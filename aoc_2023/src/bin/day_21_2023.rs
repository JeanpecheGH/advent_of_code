use std::collections::HashSet;
use std::str::FromStr;
use util::coord::Pos;

const SIZE: usize = 131;
const HALF: usize = 65;
struct Garden {
    start: Pos,
    plots: Vec<Vec<bool>>,
    max_x: usize,
    max_y: usize,
}

impl Garden {
    fn is_plot_at(&self, Pos(x, y): Pos) -> bool {
        self.plots[y][x]
    }

    fn plots_for_start(&self, start: Pos) -> Vec<usize> {
        // println!("--- {:?} ---", start);
        let mut plots_by_step: Vec<usize> = vec![1];

        let mut evens: HashSet<Pos> = HashSet::from_iter(vec![start]);
        let mut odds: HashSet<Pos> = HashSet::new();

        let mut current_plots: HashSet<Pos> = evens.clone();
        let mut i: usize = 1;
        while !current_plots.is_empty() {
            // if current_plots.len() < 5 {
            //     println!("{i}: {:?}", current_plots);
            // }
            current_plots = current_plots
                .into_iter()
                .flat_map(|p| p.neighbours_safe(SIZE, SIZE))
                .filter(|&p| {
                    if self.is_plot_at(p) {
                        if i % 2 == 0 {
                            evens.insert(p)
                        } else {
                            odds.insert(p)
                        }
                    } else {
                        false
                    }
                })
                .collect();
            if current_plots.is_empty() {
                break;
            }
            if i % 2 == 0 {
                plots_by_step.push(evens.len());
            } else {
                plots_by_step.push(odds.len());
            }
            i += 1;
        }
        // println!("{:?}", plots_by_step);
        plots_by_step
    }

    fn infinite_plots_in_steps(&self, steps: usize) -> usize {
        //Compute the way the garden is traveled, for each possible start point
        let mut center: Vec<usize> = self.plots_for_start(self.start);
        let top_left: Vec<usize> = self.plots_for_start(Pos(0, 0));
        let top: Vec<usize> = self.plots_for_start(Pos(HALF, 0));
        let top_right: Vec<usize> = self.plots_for_start(Pos(SIZE - 1, 0));
        let left: Vec<usize> = self.plots_for_start(Pos(0, HALF));
        let right: Vec<usize> = self.plots_for_start(Pos(SIZE - 1, HALF));
        let bottom_left: Vec<usize> = self.plots_for_start(Pos(0, SIZE - 1));
        let bottom: Vec<usize> = self.plots_for_start(Pos(HALF, SIZE - 1));
        let bottom_right: Vec<usize> = self.plots_for_start(Pos(SIZE - 1, SIZE - 1));

        //Travels starting on a corner/center takes even steps
        //Travels starting on an edge takes odd steps

        let even: usize = center.pop().unwrap();
        let odd: usize = center.pop().unwrap();
        //Nb of full squares
        let div: usize = steps / SIZE;
        //Nb of steps on the flat sides
        let sides_n: usize = SIZE - 1;
        //Nb of steps on the corner sides
        let corner_n: usize = HALF - 1;
        let corner_inside_n: usize = sides_n + HALF;
        let odd_square: usize = div * div * even;
        let even_square: usize = (div - 1) * (div - 1) * odd;
        let corner_sides: usize = div
            * (top_left[corner_n]
                + top_right[corner_n]
                + bottom_left[corner_n]
                + bottom_right[corner_n]);
        let corner_inside: usize = (div - 1)
            * (top_left[corner_inside_n]
                + top_right[corner_inside_n]
                + bottom_left[corner_inside_n]
                + bottom_right[corner_inside_n]);
        let flat_sides: usize = top[sides_n] + left[sides_n] + right[sides_n] + bottom[sides_n];

        let sum: usize = odd_square + even_square + corner_sides + corner_inside + flat_sides;
        sum
    }
    fn plots_in_steps_2(&self, start: Pos, steps: usize) -> usize {
        let mut evens: HashSet<Pos> = HashSet::from_iter(vec![start]);
        let mut odds: HashSet<Pos> = HashSet::new();

        let mut current_plots: HashSet<Pos> = evens.clone();
        for i in 1..=steps {
            current_plots = current_plots
                .into_iter()
                .flat_map(|p| p.neighbours_safe(self.max_x, self.max_y))
                .filter(|&p| {
                    if self.is_plot_at(p) {
                        if i % 2 == 0 {
                            evens.insert(p)
                        } else {
                            odds.insert(p)
                        }
                    } else {
                        false
                    }
                })
                .collect();
        }

        if steps % 2 == 0 {
            evens.len()
        } else {
            odds.len()
        }
    }
    fn plots_in_steps(&self, steps: usize) -> usize {
        let mut evens: HashSet<Pos> = HashSet::from_iter(vec![self.start]);
        let mut odds: HashSet<Pos> = HashSet::new();

        let mut current_plots: HashSet<Pos> = evens.clone();
        for i in 1..=steps {
            current_plots = current_plots
                .into_iter()
                .flat_map(|p| p.neighbours_safe(self.max_x, self.max_y))
                .filter(|&p| {
                    if self.is_plot_at(p) {
                        if i % 2 == 0 {
                            evens.insert(p)
                        } else {
                            odds.insert(p)
                        }
                    } else {
                        false
                    }
                })
                .collect();
        }

        if steps % 2 == 0 {
            evens.len()
        } else {
            odds.len()
        }
    }
}

impl FromStr for Garden {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut start: Pos = Pos(0, 0);
        let plots: Vec<Vec<bool>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if c == 'S' {
                            start = Pos(x, y);
                        }
                        c != '#'
                    })
                    .collect::<Vec<bool>>()
            })
            .collect();
        let max_x: usize = plots[0].len();
        let max_y: usize = plots.len();
        Ok(Garden {
            start,
            plots,
            max_x,
            max_y,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_21.txt").expect("Cannot open input file");
    let garden: Garden = s.parse().unwrap();
    println!("Part1: {}", garden.plots_in_steps(64));
    println!("Part2: {}", garden.infinite_plots_in_steps(26501365));
    assert_eq!(596734624269210, garden.infinite_plots_in_steps(26501365));
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "...........
.....###.#.
.###.##..#.
..#.#...#..
....#.#....
.##..S####.
.##..#...#.
.......##..
.##.#.####.
.##..##.##.
...........
";

    #[test]
    fn part_1() {
        let garden: Garden = EXAMPLE_1.parse().unwrap();
        assert_eq!(garden.plots_in_steps(6), 16);
    }
}
