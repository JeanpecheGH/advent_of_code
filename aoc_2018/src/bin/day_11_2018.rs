use util::coord::{Pos, Pos3};

const SIZE: usize = 300;

struct Fuel {
    summed_grid: Vec<Vec<isize>>,
}

impl Fuel {
    fn from(serial_number: isize) -> Self {
        let mut summed_grid: Vec<Vec<isize>> = Vec::new();

        for y in 0..SIZE {
            let mut row: Vec<isize> = Vec::new();
            let mut summed_row: isize = 0;
            #[allow(clippy::needless_range_loop)]
            for x in 0..SIZE {
                summed_row += Self::power_level(serial_number, Pos(x + 1, y + 1));
                if y > 0 {
                    row.push(summed_row + summed_grid[y - 1][x])
                } else {
                    row.push(summed_row)
                }
            }
            summed_grid.push(row);
        }

        Self { summed_grid }
    }

    fn power_level(serial_number: isize, Pos(x, y): Pos) -> isize {
        let rack_id: isize = x as isize + 10;
        let mut power: isize = rack_id * y as isize;
        power += serial_number;
        power *= rack_id;
        power /= 100;
        power %= 10;
        power -= 5;

        power
    }

    fn square_power(&self, Pos(x, y): Pos, size: usize) -> isize {
        match (x, y) {
            (0, 0) => self.summed_grid[y + size - 1][x + size - 1],
            (_, 0) => {
                self.summed_grid[y + size - 1][x + size - 1] - self.summed_grid[y + size - 1][x - 1]
            }
            (0, _) => {
                self.summed_grid[y + size - 1][x + size - 1] - self.summed_grid[y - 1][x + size - 1]
            }
            _ => {
                self.summed_grid[y + size - 1][x + size - 1] + self.summed_grid[y - 1][x - 1]
                    - self.summed_grid[y + size - 1][x - 1]
                    - self.summed_grid[y - 1][x + size - 1]
            }
        }
    }

    fn best_block_of_size(&self, size: usize) -> (Pos, isize) {
        let mut best: isize = isize::MIN;
        let mut best_pos: Pos = Pos(0, 0);
        for y in 0..=(SIZE - size) {
            for x in 0..(SIZE - size) {
                let block = self.square_power(Pos(x, y), size);
                if block > best {
                    best = block;
                    best_pos = Pos(x + 1, y + 1)
                }
            }
        }

        (best_pos, best)
    }

    fn best_block_all_sizes(&self) -> (Pos3, isize) {
        let mut best: isize = isize::MIN;
        let mut best_pos: Pos3 = Pos3(0, 0, 0);

        for i in 2..=SIZE {
            let (Pos(x, y), b) = self.best_block_of_size(i);
            if b > best {
                best = b;
                best_pos = Pos3(x, y, i)
            }
        }
        (best_pos, best)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let fuel: Fuel = Fuel::from(3628);
    let (Pos(x, y), power) = fuel.best_block_of_size(3);
    println!(
        "Part1: The 3x3 square with the largest power is in [{x},{y}], with a power of {power}"
    );
    let (Pos3(x, y, size), power) = fuel.best_block_all_sizes();
    println!("Part2: The square with the largest power is a {size}x{size} square in [{x},{y}], with a power of {power}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    use util::coord::Pos;
    #[test]
    fn part_1_test_1() {
        assert_eq!(Fuel::power_level(8, Pos(3, 5)), 4);
    }
    #[test]
    fn part_1_test_2() {
        assert_eq!(Fuel::power_level(57, Pos(122, 79)), -5);
    }
    #[test]
    fn part_1_test_3() {
        assert_eq!(Fuel::power_level(39, Pos(217, 196)), 0);
    }
    #[test]
    fn part_1_test_4() {
        assert_eq!(Fuel::power_level(71, Pos(101, 153)), 4);
    }
    #[test]
    fn part_1_test_5() {
        let fuel: Fuel = Fuel::from(18);
        assert_eq!(fuel.best_block_of_size(3), (Pos(33, 45), 29));
    }
    #[test]
    fn part_1_test_6() {
        let fuel: Fuel = Fuel::from(42);
        assert_eq!(fuel.best_block_of_size(3), (Pos(21, 61), 30));
    }
    #[test]
    fn part_2_test_1() {
        let fuel: Fuel = Fuel::from(18);
        assert_eq!(fuel.best_block_all_sizes(), (Pos3(90, 269, 16), 113));
    }
    #[test]
    fn part_2_test_2() {
        let fuel: Fuel = Fuel::from(42);
        assert_eq!(fuel.best_block_all_sizes(), (Pos3(232, 251, 12), 119));
    }
}
