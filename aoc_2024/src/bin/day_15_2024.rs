use fxhash::FxHashSet;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;
use util::split_blocks;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum WideTile {
    Wall,
    LeftBox,
    RightBox,
    Robot,
    Empty,
}

impl WideTile {
    fn from_char(c: char) -> Vec<WideTile> {
        match c {
            '#' => vec![WideTile::Wall, WideTile::Wall],
            'O' => vec![WideTile::LeftBox, WideTile::RightBox],
            '@' => vec![WideTile::Robot, WideTile::Empty],
            _ => vec![WideTile::Empty, WideTile::Empty],
        }
    }
}
struct WideWarehouse {
    grid: Vec<Vec<WideTile>>,
    start: Pos,
    robot_moves: Vec<Dir>,
}

impl WideWarehouse {
    fn pos_in_front(Pos(x, y): Pos, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos(x, y - 1),
            Dir::East => Pos(x + 1, y),
            Dir::South => Pos(x, y + 1),
            Dir::West => Pos(x - 1, y),
        }
    }

    fn move_box(grid: &mut [Vec<WideTile>], Pos(x, y): Pos, dir: Dir) {
        let Pos(i, j) = Self::pos_in_front(Pos(x, y), dir);
        grid[j][i] = grid[y][x];
        grid[y][x] = WideTile::Empty;
    }

    fn gps_coordinates(&self) -> usize {
        let mut grid: Vec<Vec<WideTile>> = self.grid.clone();
        let mut pos: Pos = self.start;

        for &m in &self.robot_moves {
            if m == Dir::West || m == Dir::East {
                // Right and left moves are simple
                let mut to_move: Vec<Pos> = Vec::new();
                let mut in_front: Pos = Self::pos_in_front(pos, m);
                let mut tile_in_front: WideTile = grid[in_front.1][in_front.0];
                while tile_in_front == WideTile::LeftBox || tile_in_front == WideTile::RightBox {
                    to_move.push(in_front);
                    in_front = Self::pos_in_front(in_front, m);
                    tile_in_front = grid[in_front.1][in_front.0];
                }
                if tile_in_front != WideTile::Wall {
                    // Move the robot
                    let in_front: Pos = Self::pos_in_front(pos, m);
                    pos = in_front;
                    // Move every box in the column
                    for &p in to_move.iter().rev() {
                        Self::move_box(&mut grid, p, m);
                    }
                }
            } else {
                // Up and down moves can move multiple column of boxes
                let mut to_move: Vec<FxHashSet<Pos>> = Vec::new();
                let mut in_front: Vec<Pos> = vec![Self::pos_in_front(pos, m)];
                let mut tiles_in_front: Vec<WideTile> =
                    in_front.iter().map(|&Pos(x, y)| grid[y][x]).collect();
                while (tiles_in_front.contains(&WideTile::LeftBox)
                    || tiles_in_front.contains(&WideTile::RightBox))
                    && !tiles_in_front.contains(&WideTile::Wall)
                {
                    //Add the part of boxes that are on the side but will be pushed nonetheless
                    let row_to_push: FxHashSet<Pos> = in_front
                        .iter()
                        .flat_map(|&Pos(x, y)| {
                            if grid[y][x] == WideTile::LeftBox {
                                vec![Pos(x, y), Pos(x + 1, y)]
                            } else if grid[y][x] == WideTile::RightBox {
                                vec![Pos(x, y), Pos(x - 1, y)]
                            } else {
                                Vec::new()
                            }
                        })
                        .collect();
                    in_front = row_to_push
                        .iter()
                        .map(|&p| Self::pos_in_front(p, m))
                        .collect();
                    to_move.push(row_to_push);
                    tiles_in_front = in_front.iter().map(|&Pos(x, y)| grid[y][x]).collect();
                }
                if !tiles_in_front.contains(&WideTile::Wall) {
                    // Move the robot
                    pos = Self::pos_in_front(pos, m);
                    // Move every box part of the column
                    for row in to_move.iter().rev() {
                        for &p in row.iter() {
                            Self::move_box(&mut grid, p, m);
                        }
                    }
                }
            }
        }

        grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &tile)| {
                        if tile == WideTile::LeftBox {
                            100 * y + x
                        } else {
                            0
                        }
                    })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for WideWarehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = split_blocks(s);

        let mut grid: Vec<Vec<WideTile>> = blocks[0]
            .lines()
            .map(|l| l.chars().flat_map(WideTile::from_char).collect())
            .collect();

        let mut start: Pos = Pos(0, 0);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == WideTile::Robot {
                    start = Pos(x, y);
                    grid[y][x] = WideTile::Empty;
                }
            }
        }

        let robot_moves: Vec<Dir> = blocks[1]
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| Dir::from_char(c).unwrap())
                    .collect::<Vec<Dir>>()
            })
            .collect();
        Ok(WideWarehouse {
            grid,
            start,
            robot_moves,
        })
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
}

impl Tile {
    pub fn from_char(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            'O' => Tile::Box,
            '@' => Tile::Robot,
            _ => Tile::Empty,
        }
    }
}
struct Warehouse {
    grid: Vec<Vec<Tile>>,
    start: Pos,
    robot_moves: Vec<Dir>,
}
impl Warehouse {
    fn pos_in_front(Pos(x, y): Pos, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos(x, y - 1),
            Dir::East => Pos(x + 1, y),
            Dir::South => Pos(x, y + 1),
            Dir::West => Pos(x - 1, y),
        }
    }

    fn gps_coordinates(&self) -> usize {
        let mut grid: Vec<Vec<Tile>> = self.grid.clone();
        let mut pos: Pos = self.start;

        for &m in &self.robot_moves {
            let mut boxes: Vec<Pos> = Vec::new();
            let mut in_front: Pos = Self::pos_in_front(pos, m);
            let mut tile_in_front: Tile = grid[in_front.1][in_front.0];
            while tile_in_front == Tile::Box {
                boxes.push(in_front);
                in_front = Self::pos_in_front(in_front, m);
                tile_in_front = grid[in_front.1][in_front.0];
            }
            if tile_in_front != Tile::Wall {
                // Move the robot
                let in_front: Pos = Self::pos_in_front(pos, m);
                pos = in_front;
                // Remove the first box in front
                grid[in_front.1][in_front.0] = Tile::Empty;
                // Add a box in the empty space after the last box
                if let Some(last_box) = boxes.pop() {
                    let in_front_last_box: Pos = Self::pos_in_front(last_box, m);
                    grid[in_front_last_box.1][in_front_last_box.0] = Tile::Box;
                }
            }
        }

        grid.iter()
            .enumerate()
            .map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .map(|(x, &tile)| if tile == Tile::Box { 100 * y + x } else { 0 })
                    .sum::<usize>()
            })
            .sum()
    }
}

impl FromStr for Warehouse {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks = split_blocks(s);

        let mut grid: Vec<Vec<Tile>> = blocks[0]
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect())
            .collect();

        let mut start: Pos = Pos(0, 0);
        for y in 0..grid.len() {
            for x in 0..grid[0].len() {
                if grid[y][x] == Tile::Robot {
                    start = Pos(x, y);
                    grid[y][x] = Tile::Empty;
                }
            }
        }

        let robot_moves: Vec<Dir> = blocks[1]
            .lines()
            .flat_map(|l| {
                l.chars()
                    .map(|c| Dir::from_char(c).unwrap())
                    .collect::<Vec<Dir>>()
            })
            .collect();
        Ok(Warehouse {
            grid,
            start,
            robot_moves,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_15.txt").expect("Cannot open input file");
    let warehouse: Warehouse = s.parse().unwrap();
    println!(
        "Part1: The sum of all GPS coordinates in the warehouse is {}",
        warehouse.gps_coordinates()
    );
    let wide_warehouse: WideWarehouse = s.parse().unwrap();
    println!(
        "Part2: The sum of all GPS coordinates in the wide warehouse is {}",
        wide_warehouse.gps_coordinates()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    const EXAMPLE_2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn part_1_test_1() {
        let warehouse: Warehouse = EXAMPLE_1.parse().unwrap();
        assert_eq!(warehouse.gps_coordinates(), 2028);
    }
    #[test]
    fn part_1_test_2() {
        let warehouse: Warehouse = EXAMPLE_2.parse().unwrap();
        assert_eq!(warehouse.gps_coordinates(), 10092);
    }

    #[test]
    fn part_2_test_1() {
        let wide_warehouse: WideWarehouse = EXAMPLE_2.parse().unwrap();
        assert_eq!(wide_warehouse.gps_coordinates(), 9021);
    }
}
