use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Debug)]
enum Track {
    Horizontal,
    Vertical,
    Cross,
    Slash,
    Antislash,
    Empty,
}

impl Track {
    fn from(c: char) -> Self {
        match c {
            '-' => Track::Horizontal,
            '|' => Track::Vertical,
            '+' => Track::Cross,
            '/' => Track::Slash,
            '\\' => Track::Antislash,
            _ => Track::Empty,
        }
    }
}

#[derive(Debug)]
enum NextCross {
    Left,
    Right,
    Straight,
}

impl NextCross {
    fn next(&self) -> NextCross {
        match self {
            NextCross::Left => NextCross::Straight,
            NextCross::Right => NextCross::Left,
            NextCross::Straight => NextCross::Right,
        }
    }
}

#[derive(Debug)]
struct Cart {
    pos: Pos,
    dir: Dir,
    turn_status: NextCross,
}

impl Cart {
    fn from(c: char, pos: Pos) -> Option<Self> {
        let dir_opt: Option<Dir> = match c {
            '^' => Some(Dir::North),
            'v' => Some(Dir::South),
            '<' => Some(Dir::West),
            '>' => Some(Dir::East),
            _ => None,
        };

        dir_opt.map(|dir| Cart {
            pos,
            dir,
            turn_status: NextCross::Left,
        })
    }

    fn in_front(&self) -> Pos {
        let Pos(x, y) = self.pos;
        match self.dir {
            Dir::North => Pos(x, y - 1),
            Dir::East => Pos(x + 1, y),
            Dir::South => Pos(x, y + 1),
            Dir::West => Pos(x - 1, y),
        }
    }

    fn advance(&mut self, t: &Track) {
        match self.dir {
            Dir::North => self.pos.1 -= 1,
            Dir::East => self.pos.0 += 1,
            Dir::South => self.pos.1 += 1,
            Dir::West => self.pos.0 -= 1,
        }
        match (t, &self.turn_status) {
            (Track::Cross, NextCross::Left) => {
                self.dir = self.dir.turn_left();
                self.turn_status = self.turn_status.next();
            }
            (Track::Cross, NextCross::Right) => {
                self.dir = self.dir.turn_right();
                self.turn_status = self.turn_status.next();
            }
            (Track::Cross, NextCross::Straight) => {
                self.turn_status = self.turn_status.next();
            }
            (Track::Slash, _) => {
                self.dir = match self.dir {
                    Dir::North => Dir::East,
                    Dir::East => Dir::North,
                    Dir::South => Dir::West,
                    Dir::West => Dir::South,
                }
            }
            (Track::Antislash, _) => {
                self.dir = match self.dir {
                    Dir::North => Dir::West,
                    Dir::East => Dir::South,
                    Dir::South => Dir::East,
                    Dir::West => Dir::North,
                }
            }
            //Keep going straight
            _ => (),
        }
    }
}

#[derive(Debug)]
struct TracksCircuit {
    grid: Vec<Vec<Track>>,
    carts: Vec<Cart>,
}

impl TracksCircuit {
    fn run_carts(&mut self) -> (Pos, Pos) {
        let mut collisions: Vec<Pos> = Vec::new();
        while self.carts.len() > 1 {
            self.carts
                .sort_by(|a, b| a.pos.1.cmp(&b.pos.1).then(a.pos.0.cmp(&b.pos.0)));
            let col = self.tick();
            self.carts.retain(|cart| !col.contains(&cart.pos));
            collisions.extend(col);
        }
        (
            collisions.first().copied().unwrap(),
            self.carts.first().map(|c| c.pos).unwrap_or(Pos(0, 0)),
        )
    }
    fn tick(&mut self) -> Vec<Pos> {
        let mut collisions: Vec<Pos> = Vec::new();

        let n: usize = self.carts.len();
        for i in 0..n {
            if collisions.contains(&self.carts[i].pos) {
                continue;
            }

            let Pos(x, y) = self.carts[i].in_front();
            self.carts[i].advance(&self.grid[y][x]);
            let p: Pos = self.carts[i].pos;

            for j in 0..n {
                if i != j && p == self.carts[j].pos {
                    collisions.push(p)
                }
            }
        }
        collisions
    }
}

impl FromStr for TracksCircuit {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut carts: Vec<Cart> = Vec::new();

        let grid: Vec<Vec<Track>> = s
            .lines()
            .enumerate()
            .map(|(y, row)| {
                row.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        if let Some(cart) = Cart::from(c, Pos(x, y)) {
                            let dir: Dir = cart.dir;
                            carts.push(cart);
                            match dir {
                                Dir::North | Dir::South => Track::Vertical,
                                Dir::East | Dir::West => Track::Horizontal,
                            }
                        } else {
                            Track::from(c)
                        }
                    })
                    .collect()
            })
            .collect();

        Ok(TracksCircuit { grid, carts })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_13.txt").expect("Cannot open input file");
    let mut circuit: TracksCircuit = s.parse().unwrap();
    let (Pos(x, y), Pos(i, j)): (Pos, Pos) = circuit.run_carts();

    println!("Part1: The first crash occurs in [{x},{y}]");
    println!("Part2: The last cart is in [{i},{j}]");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = r#"/->-\        
|   |  /----\
| /-+--+-\  |
| | |  | v  |
\-+-/  \-+--/
  \------/   
"#;

    const EXAMPLE_2: &str = r#"/>-<\  
|   |  
| /<+-\
| | | v
\>+</ |
  |   ^
  \<->/"#;

    #[test]
    fn part_1() {
        let mut circuit: TracksCircuit = EXAMPLE_1.parse().unwrap();
        assert_eq!(circuit.run_carts().0, Pos(7, 3));
    }

    #[test]
    fn part_2() {
        let mut circuit: TracksCircuit = EXAMPLE_2.parse().unwrap();
        assert_eq!(circuit.run_carts().1, Pos(6, 4));
    }
}
