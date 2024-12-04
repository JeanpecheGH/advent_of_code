use std::str::FromStr;

const XMAS: [char; 4] = ['X', 'M', 'A', 'S'];
const SAMX: [char; 4] = ['S', 'A', 'M', 'X'];

struct WordSearch {
    grid: Vec<Vec<char>>,
}

impl WordSearch {
    fn nb_horizontal(&self) -> usize {
        self.grid
            .iter()
            .map(|row| {
                row.windows(4)
                    .filter(|block| block.eq(&XMAS) || block.eq(&SAMX))
                    .count()
            })
            .sum()
    }

    fn nb_vertical(&self) -> usize {
        let mut count: usize = 0;
        for x in 0..self.grid[0].len() {
            for y in 0..self.grid.len() - 3 {
                if (self.grid[y][x] == 'X'
                    && self.grid[y + 1][x] == 'M'
                    && self.grid[y + 2][x] == 'A'
                    && self.grid[y + 3][x] == 'S')
                    || (self.grid[y][x] == 'S'
                        && self.grid[y + 1][x] == 'A'
                        && self.grid[y + 2][x] == 'M'
                        && self.grid[y + 3][x] == 'X')
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn nb_diag(&self) -> usize {
        let mut count: usize = 0;
        for x in 0..self.grid[0].len() - 3 {
            for y in 0..self.grid.len() - 3 {
                if (self.grid[y][x] == 'X'
                    && self.grid[y + 1][x + 1] == 'M'
                    && self.grid[y + 2][x + 2] == 'A'
                    && self.grid[y + 3][x + 3] == 'S')
                    || (self.grid[y][x] == 'S'
                        && self.grid[y + 1][x + 1] == 'A'
                        && self.grid[y + 2][x + 2] == 'M'
                        && self.grid[y + 3][x + 3] == 'X')
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn nb_other_diag(&self) -> usize {
        let mut count: usize = 0;
        for x in 0..self.grid[0].len() - 3 {
            for y in 3..self.grid.len() {
                if (self.grid[y][x] == 'X'
                    && self.grid[y - 1][x + 1] == 'M'
                    && self.grid[y - 2][x + 2] == 'A'
                    && self.grid[y - 3][x + 3] == 'S')
                    || (self.grid[y][x] == 'S'
                        && self.grid[y - 1][x + 1] == 'A'
                        && self.grid[y - 2][x + 2] == 'M'
                        && self.grid[y - 3][x + 3] == 'X')
                {
                    count += 1;
                }
            }
        }
        count
    }

    fn nb_cross_mas(&self) -> usize {
        let mut count: usize = 0;
        for y in 0..self.grid.len() - 2 {
            for x in 0..self.grid[0].len() - 2 {
                if (self.grid[y + 1][x + 1] == 'A')
                    && ((self.grid[y][x] == 'M' && self.grid[y + 2][x + 2] == 'S')
                        || (self.grid[y][x] == 'S' && self.grid[y + 2][x + 2] == 'M'))
                    && ((self.grid[y + 2][x] == 'M' && self.grid[y][x + 2] == 'S')
                        || (self.grid[y + 2][x] == 'S' && self.grid[y][x + 2] == 'M'))
                {
                    count += 1;
                }
            }
        }
        count
    }
    fn nb_xmas(&self) -> usize {
        self.nb_horizontal() + self.nb_vertical() + self.nb_diag() + self.nb_other_diag()
    }
}

impl FromStr for WordSearch {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s.lines().map(|l| l.chars().collect()).collect();

        Ok(WordSearch { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_04.txt").expect("Cannot open input file");
    let search: WordSearch = s.parse().unwrap();

    println!("Part1: The word XMAS appears {} times", search.nb_xmas());
    println!(
        "Part2: The X-MAS pattern appears {} times",
        search.nb_cross_mas()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
";
    #[test]
    fn part_1() {
        let search: WordSearch = EXAMPLE_1.parse().unwrap();
        assert_eq!(search.nb_xmas(), 18);
    }
    #[test]
    fn part_2() {
        let search: WordSearch = EXAMPLE_1.parse().unwrap();
        assert_eq!(search.nb_cross_mas(), 9);
    }
}
