use std::str::FromStr;

#[derive(Debug)]
enum Action {
    North(isize),
    South(isize),
    East(isize),
    West(isize),
    Left(isize),
    Right(isize),
    Forward(isize),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars: Vec<char> = s.chars().collect();
        let value: isize = s[1..].parse().unwrap();
        match chars[0] {
            'N' => Ok(Action::North(value)),
            'S' => Ok(Action::South(value)),
            'E' => Ok(Action::East(value)),
            'W' => Ok(Action::West(value)),
            'L' => Ok(Action::Left(value)),
            'R' => Ok(Action::Right(value)),
            'F' => Ok(Action::Forward(value)),
            _ => Err(()),
        }
    }
}

enum Dir {
    North,
    South,
    East,
    West,
}

struct Boat {
    x: isize,
    y: isize,
    dir: Dir,
}

impl Boat {
    fn move_boat(&mut self, action: &Action) {
        match action {
            Action::North(n) => self.y += *n,
            Action::South(n) => self.y -= *n,
            Action::East(n) => self.x += *n,
            Action::West(n) => self.x -= *n,
            Action::Left(n) => {
                (0..n / 90).for_each(|_| self.rotate_left());
            }
            Action::Right(n) => {
                (0..n / 90).for_each(|_| self.rotate_right());
            }
            Action::Forward(n) => {
                self.move_forward(*n);
            }
        }
    }

    fn move_forward(&mut self, n: isize) {
        match &self.dir {
            Dir::North => self.y += n,
            Dir::South => self.y -= n,
            Dir::East => self.x += n,
            Dir::West => self.x -= n,
        }
    }
    fn rotate_left(&mut self) {
        match &self.dir {
            Dir::North => self.dir = Dir::West,
            Dir::West => self.dir = Dir::South,
            Dir::South => self.dir = Dir::East,
            Dir::East => self.dir = Dir::North,
        }
    }

    fn rotate_right(&mut self) {
        match &self.dir {
            Dir::North => self.dir = Dir::East,
            Dir::West => self.dir = Dir::North,
            Dir::South => self.dir = Dir::West,
            Dir::East => self.dir = Dir::South,
        }
    }

    fn dist(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

struct WaypointBoat {
    x: isize,
    y: isize,
    w_x: isize,
    w_y: isize,
}

impl WaypointBoat {
    fn move_boat(&mut self, action: &Action) {
        match action {
            Action::North(n) => self.w_y += *n,
            Action::South(n) => self.w_y -= *n,
            Action::East(n) => self.w_x += *n,
            Action::West(n) => self.w_x -= *n,
            Action::Left(n) => {
                (0..n / 90).for_each(|_| self.rotate_left());
            }
            Action::Right(n) => {
                (0..n / 90).for_each(|_| self.rotate_right());
            }
            Action::Forward(n) => {
                self.move_to_waypoint(*n);
            }
        }
    }

    fn move_to_waypoint(&mut self, times: isize) {
        self.x += self.w_x * times;
        self.y += self.w_y * times;
    }

    fn rotate_left(&mut self) {
        (self.w_x, self.w_y) = (-self.w_y, self.w_x)
    }

    fn rotate_right(&mut self) {
        (self.w_x, self.w_y) = (self.w_y, -self.w_x)
    }

    fn dist(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_12.txt").expect("Cannot open input file");

    let actions: Vec<Action> = s.lines().map(|s| s.parse().unwrap()).collect();

    let mut boat = Boat {
        x: 0,
        y: 0,
        dir: Dir::East,
    };
    actions.iter().for_each(|ac| {
        boat.move_boat(ac);
    });
    println!(
        "Part1: The boat travelled {} units in Manhattan distance",
        boat.dist()
    );

    let mut wp_boat = WaypointBoat {
        x: 0,
        y: 0,
        w_x: 10,
        w_y: 1,
    };
    actions.iter().for_each(|ac| {
        wp_boat.move_boat(ac);
    });
    println!(
        "Part2: The waypoint directed boat travelled {} units in Manhattan distance",
        wp_boat.dist()
    );
}
