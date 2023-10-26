use itertools::MinMaxResult::MinMax;
use itertools::{Itertools, MinMaxResult};
use std::collections::HashSet;
use util::intcode::IntCode;
use util::orientation::Dir;

type Pos = (isize, isize);
type Err = ();

struct Position {
    pos: Pos,
    orient: Dir,
}

impl Position {
    fn turn(&mut self, n: isize) {
        if n == 0 {
            self.turn_left();
        } else {
            self.turn_right();
        }
        self.advance();
    }

    fn turn_left(&mut self) {
        self.orient = self.orient.turn_left()
    }

    fn turn_right(&mut self) {
        self.orient = self.orient.turn_right()
    }

    fn advance(&mut self) {
        let (x, y): (isize, isize) = match &self.orient {
            Dir::North => (0, -1),
            Dir::East => (1, 0),
            Dir::South => (0, 1),
            Dir::West => (-1, 0),
        };
        self.pos = (self.pos.0 + x, self.pos.1 + y);
    }
}

struct Robot {
    position: Position,
    painted: HashSet<Pos>,
    white: HashSet<Pos>,
    code: IntCode,
}

impl Robot {
    fn from_code(code: IntCode, start_on_white: bool) -> Self {
        let position = Position {
            pos: (0, 0),
            orient: Dir::North,
        };
        let mut white: HashSet<Pos> = HashSet::new();
        if start_on_white {
            white.insert((0, 0));
        }
        Robot {
            position,
            painted: HashSet::new(),
            white,
            code,
        }
    }

    fn paint_one(&mut self) -> Result<(), Err> {
        let pos: Pos = self.position.pos;
        let input: isize = self.white.contains(&pos) as isize;
        self.code.compute(&mut vec![input]);
        let turn: isize = self.code.output.pop().ok_or(())?;
        let color: isize = self.code.output.pop().ok_or(())?;
        self.painted.insert(pos);
        if color == 1 {
            self.white.insert(pos);
        } else {
            self.white.remove(&pos);
        }
        self.position.turn(turn);
        Ok(())
    }

    fn paint(&mut self) {
        loop {
            if self.paint_one().is_err() {
                break;
            }
        }
    }

    fn nb_painted(&self) -> usize {
        self.painted.len()
    }

    fn print(&self) {
        let min_max_x: MinMaxResult<isize> = self.white.iter().map(|pos| pos.0).minmax();
        let min_max_y: MinMaxResult<isize> = self.white.iter().map(|pos| pos.1).minmax();
        let (min_x, max_x): (isize, isize) = if let MinMax(a, b) = min_max_x {
            (a, b)
        } else {
            (0, 0)
        };
        let (min_y, max_y): (isize, isize) = if let MinMax(a, b) = min_max_y {
            (a, b)
        } else {
            (0, 0)
        };

        for y in min_y..=max_y {
            for x in min_x..=max_x {
                let s: &str = if self.white.contains(&(x, y)) {
                    "██"
                } else {
                    "  "
                };
                print!("{s}");
            }
            println!();
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_11.txt").expect("Cannot open input file");
    let code: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut robot = Robot::from_code(code.clone(), false);
    robot.paint();
    println!(
        "Part1: The robot painted over {} panels",
        robot.nb_painted()
    );
    let mut robot = Robot::from_code(code, true);
    robot.paint();
    println!("Part2: Your read the Registration Identifier on your spacecraft ");
    robot.print();
    println!("Computing time: {:?}", now.elapsed());
}
