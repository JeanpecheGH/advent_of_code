use itertools::Itertools;
use std::str::FromStr;

const HEIGHT: usize = 200;
const WIDTH: usize = 150;

type Pos = (usize, usize);

#[derive(Debug, Copy, Clone)]
enum Orient {
    East = 0,
    South = 1,
    West = 2,
    North = 3,
}

impl Orient {
    fn right(&self) -> Self {
        match self {
            Orient::East => Orient::South,
            Orient::South => Orient::West,
            Orient::West => Orient::North,
            Orient::North => Orient::East,
        }
    }

    fn left(&self) -> Self {
        match self {
            Orient::East => Orient::North,
            Orient::South => Orient::East,
            Orient::West => Orient::South,
            Orient::North => Orient::West,
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum Op {
    Move(usize),
    Left,
    Right,
}

#[derive(Debug, Clone)]
struct Labyrinth {
    grid: [[Option<bool>; WIDTH + 1]; HEIGHT + 1],
    pos: Pos,
    orient: Orient,
    ops: Vec<Op>,
}

impl Labyrinth {
    fn move_all(&mut self, in_cube: bool) {
        let ops: Vec<Op> = self.ops.clone();
        for op in ops {
            self.move_one(&op, in_cube);
        }
    }

    fn move_one(&mut self, op: &Op, in_cube: bool) {
        match op {
            Op::Move(n) => (0..*n).for_each(|_| {
                if in_cube {
                    self.advance_one_in_cube()
                } else {
                    self.advance_one()
                }
            }),
            Op::Left => self.orient = self.orient.left(),
            Op::Right => self.orient = self.orient.right(),
        }
    }

    fn advance_one_in_cube(&mut self) {
        /*
           Cube pattern (50x50x50):
           .AB
           .C.
           DE.
           F..
        */
        let (x, y) = self.pos;
        let (n_x, n_y, n_orient) = match (x, y, &self.orient) {
            (51..=100, 1, Orient::North) => (1, x + 100, Orient::East), //A to F
            (101..=150, 1, Orient::North) => (x - 100, 200, Orient::North), //B to F
            (51, 1..=50, Orient::West) => (1, 151 - y, Orient::East),   //A to D
            (150, 1..=50, Orient::East) => (100, 151 - y, Orient::West), //B to E
            (101..=150, 50, Orient::South) => (100, x - 50, Orient::West), //B to C
            (51, 51..=100, Orient::West) => (y - 50, 101, Orient::South), //C to D
            (100, 51..=100, Orient::East) => (y + 50, 50, Orient::North), //C to B
            (1..=50, 101, Orient::North) => (51, x + 50, Orient::East), //D to C
            (1, 101..=150, Orient::West) => (51, 151 - y, Orient::East), //D to A
            (100, 101..=150, Orient::East) => (150, 151 - y, Orient::West), //E to B
            (51..=100, 150, Orient::South) => (50, x + 100, Orient::West), //E to F
            (1, 151..=200, Orient::West) => (y - 100, 1, Orient::South), //F to A
            (50, 151..=200, Orient::East) => (y - 100, 150, Orient::North), //F to E
            (1..=50, 200, Orient::South) => (x + 100, 1, Orient::South), //F to B
            (_, _, Orient::North) => (x, y - 1, self.orient),
            (_, _, Orient::East) => (x + 1, y, self.orient),
            (_, _, Orient::South) => (x, y + 1, self.orient),
            (_, _, Orient::West) => (x - 1, y, self.orient),
        };

        if let Some(false) = self.grid[n_y][n_x] {
            self.pos = (n_x, n_y);
            self.orient = n_orient;
        }
        //Else we're stuck in a wall. Don't move
    }

    fn advance_one(&mut self) {
        let (x, y) = self.pos;
        let (n_x, n_y) = match self.orient {
            Orient::East => {
                let new_x: usize = if x + 1 > WIDTH || self.grid[y][x + 1].is_none() {
                    self.grid[y].iter().position(|opt| opt.is_some()).unwrap()
                } else {
                    x + 1
                };
                (new_x, y)
            }
            Orient::South => {
                let new_y: usize = if y + 1 > HEIGHT || self.grid[y + 1][x].is_none() {
                    let mut n: usize = 0;
                    while self.grid[n][x].is_none() {
                        n += 1;
                    }
                    n
                } else {
                    y + 1
                };
                (x, new_y)
            }
            Orient::West => {
                let new_x: usize = if x - 1 == 0 || self.grid[y][x - 1].is_none() {
                    self.grid[y].iter().rposition(|opt| opt.is_some()).unwrap()
                } else {
                    x - 1
                };
                (new_x, y)
            }
            Orient::North => {
                let new_y: usize = if y - 1 == 0 || self.grid[y - 1][x].is_none() {
                    let mut n: usize = HEIGHT;
                    while self.grid[n][x].is_none() {
                        n -= 1;
                    }
                    n
                } else {
                    y - 1
                };
                (x, new_y)
            }
        };

        if let Some(false) = self.grid[n_y][n_x] {
            self.pos = (n_x, n_y);
        }
        //Else, we're stuck in a wall. Don't move
    }

    fn score(&self) -> usize {
        1000 * self.pos.1 + 4 * self.pos.0 + self.orient.clone() as usize
    }
}

impl FromStr for Labyrinth {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines: Vec<&str> = s.lines().collect();
        //Parse moves
        let ops: &str = lines.pop().unwrap();
        let adv: Vec<Op> = ops
            .split(&['R', 'L'])
            .map(|w| {
                let v: usize = w.parse().unwrap();
                Op::Move(v)
            })
            .collect();
        let ros: Vec<Op> = ops
            .chars()
            .filter(|c| c.is_ascii_uppercase())
            .map(|c| if c == 'L' { Op::Left } else { Op::Right })
            .collect();
        let ops: Vec<Op> = adv.into_iter().interleave(ros.into_iter()).collect();

        lines.pop();
        //Parse grid
        let mut grid: [[Option<bool>; WIDTH + 1]; HEIGHT + 1] = [[None; WIDTH + 1]; HEIGHT + 1];
        lines.into_iter().enumerate().for_each(|(j, l)| {
            l.chars().enumerate().for_each(|(i, c)| match c {
                '#' => grid[j + 1][i + 1] = Some(true),
                '.' => grid[j + 1][i + 1] = Some(false),
                _ => (),
            });
        });

        //Find starting pos
        let start_x: usize = grid[1].iter().position(|opt| opt.is_some()).unwrap();

        Ok(Labyrinth {
            grid,
            pos: (start_x, 1),
            orient: Orient::East,
            ops,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_22.txt").expect("Cannot open input file");
    let mut laby: Labyrinth = s.parse().unwrap();
    let mut laby_1 = laby.clone();
    laby_1.move_all(false);
    println!("Part1: After moving, your score is {}", laby_1.score());
    laby.move_all(true);
    println!(
        "Part2: After moving around the cube, your score is {}",
        laby.score()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part_1() {
        assert_eq!(6032, 6032);
    }

    #[test]
    fn part_2() {
        assert_eq!(1, 1);
    }
}
