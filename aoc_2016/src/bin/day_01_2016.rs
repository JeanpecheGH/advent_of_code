use std::collections::HashSet;

enum Direction {
    N,
    E,
    S,
    W,
}

struct Position {
    visited: HashSet<(i16, i16)>,
    dir: Direction,
    x: i16,
    y: i16,
    found_hq: bool,
}

impl Position {
    fn turn_left(&mut self) {
        self.dir = match self.dir {
            Direction::N => Direction::W,
            Direction::E => Direction::N,
            Direction::S => Direction::E,
            Direction::W => Direction::S,
        }
    }
    fn turn_right(&mut self) {
        self.dir = match self.dir {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }

    fn advance_one(&mut self) {
        match self.dir {
            Direction::N => self.y += 1,
            Direction::E => self.x += 1,
            Direction::S => self.y -= 1,
            Direction::W => self.x -= 1,
        };
        if !self.visited.insert((self.x, self.y)) && !self.found_hq {
            println!(
                "Part2: The Easter Bunny HQ is at {} blocks",
                self.x.abs() + self.y.abs()
            );
            self.found_hq = true;
        }
    }

    fn advance(&mut self, n: u16) {
        (0..n).for_each(|_| self.advance_one())
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_01.txt").expect("Cannot open input file");
    let s = s.lines().next().unwrap();

    let mut pos: Position = Position {
        visited: HashSet::new(),
        dir: Direction::N,
        x: 0,
        y: 0,
        found_hq: false,
    };

    s.split(", ")
        .map(|subs| {
            let mut chars = subs.chars();
            (
                chars.next().unwrap(),
                chars.as_str().parse::<u16>().unwrap(),
            )
        })
        .for_each(|(rot, adv)| {
            match rot {
                'R' => pos.turn_right(),
                'L' => pos.turn_left(),
                _ => (),
            }
            pos.advance(adv);
        });

    println!(
        "Part1: The destination is at {} blocks",
        pos.x.abs() + pos.y.abs()
    );
}
