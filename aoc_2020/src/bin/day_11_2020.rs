use itertools::Itertools;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

struct WaitingArea {
    seats: Vec<Vec<Seat>>,
}

impl WaitingArea {
    fn next(&self, first_part: bool) -> Self {
        let max_x: usize = self.seats[0].len();
        let max_y: usize = self.seats.len();
        let mut new_seats = vec![vec![Seat::Floor; max_x]; max_y];
        for (j, row) in self.seats.iter().enumerate() {
            for (i, &seat) in row.iter().enumerate() {
                let nb_occupied = self.nb_occupied_neighbours((i as isize, j as isize), first_part);

                let limit: usize = if first_part { 4 } else { 5 };

                new_seats[j][i] = match (seat, nb_occupied) {
                    (Seat::Floor, _) => Seat::Floor,
                    (Seat::Empty, 0) => Seat::Occupied,
                    (Seat::Occupied, n) if n < limit => Seat::Occupied,
                    _ => Seat::Empty,
                }
            }
        }
        WaitingArea { seats: new_seats }
    }
    fn nb_occupied_neighbours(&self, node: Node, stop_at_first: bool) -> usize {
        let max_x: usize = self.seats[0].len();
        let max_y: usize = self.seats.len();
        if stop_at_first {
            Self::neighbours(node, max_x as isize, max_y as isize)
                .iter()
                .map(|&(x, y)| self.seats[y as usize][x as usize])
                .filter(|&seat| seat == Seat::Occupied)
                .count()
        } else {
            self.distant_neighbours(node, max_x as isize, max_y as isize)
        }
    }

    fn distant_neighbours(&self, node: Node, max_x: isize, max_y: isize) -> usize {
        let directions: Vec<Node> = (-1..=1)
            .cartesian_product(-1..=1)
            .filter(|&(x, y)| x != 0 || y != 0)
            .collect();
        directions
            .iter()
            .filter_map(|&(x, y)| {
                let mut n: Node = (node.0 + x, node.1 + y);
                while n.0 >= 0
                    && n.0 < max_x
                    && n.1 >= 0
                    && n.1 < max_y
                    && self.seats[n.1 as usize][n.0 as usize] == Seat::Floor
                {
                    n = (n.0 + x, n.1 + y);
                }
                if n.0 >= 0
                    && n.0 < max_x
                    && n.1 >= 0
                    && n.1 < max_y
                    && self.seats[n.1 as usize][n.0 as usize] == Seat::Occupied
                {
                    Some(1)
                } else {
                    None
                }
            })
            .count()
    }

    fn neighbours(node: Node, max_x: isize, max_y: isize) -> Vec<Node> {
        (node.0 - 1..=node.0 + 1)
            .cartesian_product(node.1 - 1..=node.1 + 1)
            .filter(|&(x, y)| {
                x >= 0 && x < max_x && y >= 0 && y < max_y && (x != node.0 || y != node.1)
            })
            .collect()
    }

    fn equal(&self, other: &WaitingArea) -> bool {
        self.seats.iter().enumerate().all(|(j, row)| {
            row.iter()
                .enumerate()
                .all(|(i, seat)| *seat == other.seats[j][i])
        })
    }

    fn nb_occupied(&self) -> usize {
        self.seats
            .iter()
            .map(|row| row.iter().filter(|seat| **seat == Seat::Occupied).count())
            .sum()
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.seats.iter() {
            for &seat in row.iter() {
                let c = match seat {
                    Seat::Floor => '.',
                    Seat::Empty => 'L',
                    Seat::Occupied => '#',
                };
                print!("{c}");
            }
            println!();
        }
        println!();
    }
}

type Node = (isize, isize);

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_11.txt").expect("Cannot open input file");

    let mut waiting_area = WaitingArea {
        seats: parse_seats(s.clone()),
    };
    loop {
        let new_area = waiting_area.next(true);
        if waiting_area.equal(&new_area) {
            break;
        }
        waiting_area = new_area;
    }
    println!(
        "Part1: {} seats are occupied at equilibrium",
        waiting_area.nb_occupied()
    );

    //Part 2
    let mut waiting_area_2 = WaitingArea {
        seats: parse_seats(s),
    };
    loop {
        let new_area_2 = waiting_area_2.next(false);
        if waiting_area_2.equal(&new_area_2) {
            break;
        }
        waiting_area_2 = new_area_2;
    }
    println!(
        "Part2: {} seats are occupied at equilibrium",
        waiting_area_2.nb_occupied()
    );
}

fn parse_seats(s: String) -> Vec<Vec<Seat>> {
    s.lines()
        .map(|s| {
            s.chars()
                .map(|c| match c {
                    'L' => Seat::Empty,
                    _ => Seat::Floor,
                })
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_CASE: &str = "L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";

    #[test]
    fn test_part1() {
        let mut waiting_area = WaitingArea {
            seats: parse_seats(TEST_CASE.to_string()),
        };
        loop {
            let new_area = waiting_area.next(true);
            if waiting_area.equal(&new_area) {
                break;
            }
            waiting_area = new_area;
        }
        assert_eq!(waiting_area.nb_occupied(), 37);
    }

    #[test]
    fn test_part2() {
        let mut waiting_area = WaitingArea {
            seats: parse_seats(TEST_CASE.to_string()),
        };
        loop {
            let new_area = waiting_area.next(false);
            if waiting_area.equal(&new_area) {
                break;
            }
            waiting_area = new_area;
        }
        assert_eq!(waiting_area.nb_occupied(), 26);
    }
}
