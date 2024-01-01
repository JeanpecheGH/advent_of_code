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

    fn infinite_plots_in_steps(&self, steps: usize) -> usize {
        //Size of subgarden grid
        let div: usize = steps / SIZE;

        //Number of positions in the even gardens
        let even: usize = self.plots_in_steps(self.start, SIZE * 2);
        let even_squares: usize = div * div * even;

        //Number of positions in the odd gardens
        let odd: usize = self.plots_in_steps(self.start, SIZE * 2 + 1);
        let odd_squares: usize = (div - 1) * (div - 1) * odd;

        //Number of positions in the 4 "corner" gardens
        let side_steps: usize = SIZE - 1;
        let left: usize = self.plots_in_steps(Pos(SIZE - 1, HALF), side_steps);
        let right: usize = self.plots_in_steps(Pos(0, HALF), side_steps);
        let top: usize = self.plots_in_steps(Pos(HALF, SIZE - 1), side_steps);
        let bot: usize = self.plots_in_steps(Pos(HALF, 0), side_steps);

        let corner_squares: usize = left + right + top + bot;

        //Number of positions in the small diagonal gardens
        let small_diag_steps: usize = HALF - 1;
        let small_nw: usize = self.plots_in_steps(Pos(SIZE - 1, SIZE - 1), small_diag_steps);
        let small_ne: usize = self.plots_in_steps(Pos(0, SIZE - 1), small_diag_steps);
        let small_sw: usize = self.plots_in_steps(Pos(SIZE - 1, 0), small_diag_steps);
        let small_se: usize = self.plots_in_steps(Pos(0, 0), small_diag_steps);

        let small_diag_squares: usize = div * (small_nw + small_ne + small_sw + small_se);
        //Number of positions in the big diagonal gardens
        let big_diag_steps: usize = SIZE + HALF - 1;

        let big_nw: usize = self.plots_in_steps(Pos(SIZE - 1, SIZE - 1), big_diag_steps);
        let big_ne: usize = self.plots_in_steps(Pos(0, SIZE - 1), big_diag_steps);
        let big_sw: usize = self.plots_in_steps(Pos(SIZE - 1, 0), big_diag_steps);
        let big_se: usize = self.plots_in_steps(Pos(0, 0), big_diag_steps);

        let big_diag_squares: usize = (div - 1) * (big_nw + big_ne + big_sw + big_se);

        odd_squares + even_squares + corner_squares + small_diag_squares + big_diag_squares
    }

    fn plots_in_steps(&self, start: Pos, steps: usize) -> usize {
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
    println!(
        "Part1: In 64 steps, the Elf could reach {} garden plots",
        garden.plots_in_steps(garden.start, 64)
    );
    println!(
        "Part2: In 26501365 steps, the Elf could reach {} garden plots",
        garden.infinite_plots_in_steps(26_501_365)
    );
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
        assert_eq!(garden.plots_in_steps(garden.start, 6), 16);
    }
}
