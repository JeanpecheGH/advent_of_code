use std::collections::HashSet;
use util::orientation::Dir;

struct Position {
    visited: HashSet<(i16, i16)>,
    dir: Dir,
    x: i16,
    y: i16,
    found_hq: bool,
}

impl Position {
    fn turn_left(&mut self) {
        self.dir = self.dir.turn_left()
    }
    fn turn_right(&mut self) {
        self.dir = self.dir.turn_right()
    }

    fn advance_one(&mut self) {
        match self.dir {
            Dir::North => self.y += 1,
            Dir::East => self.x += 1,
            Dir::South => self.y -= 1,
            Dir::West => self.x -= 1,
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
        dir: Dir::North,
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
