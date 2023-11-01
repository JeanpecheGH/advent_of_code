use crate::Step::{Advance, Left, Right};
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::str::FromStr;
use util::coord::Pos;
use util::intcode::IntCode;
use util::orientation::Dir;

enum Step {
    Left,
    Right,
    Advance(usize),
}

impl Display for Step {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Left => write!(f, "L"),
            Right => write!(f, "R"),
            Advance(n) => write!(f, "{n}"),
        }
    }
}

struct Path {
    current: usize,
    steps: Vec<Step>,
}

impl Path {
    fn path_string(&self) -> String {
        self.steps
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    }
}

impl Path {
    fn advance(&mut self) {
        self.current += 1;
    }

    fn stop_advance(&mut self) {
        if self.current > 0 {
            let step = Advance(self.current);
            self.current = 0;
            self.steps.push(step);
        }
    }

    fn turn_left(&mut self) {
        self.stop_advance();
        let step = Left;
        self.steps.push(step);
    }

    fn turn_right(&mut self) {
        self.stop_advance();
        let step = Right;
        self.steps.push(step);
    }
}

#[derive(Copy, Clone)]
struct Robot {
    pos: Pos,
    dir: Dir,
}

impl Robot {
    fn from_pos_and_pixel(Pos(x, y): Pos, pixel: &char) -> Self {
        let dir: Dir = match pixel {
            '^' => Dir::North,
            '>' => Dir::East,
            'v' => Dir::South,
            _ => Dir::West,
        };
        Robot {
            pos: Pos(x, y),
            dir,
        }
    }
}

struct Scaffolding {
    scaffolds: HashSet<Pos>,
    robot: Robot,
}

impl Scaffolding {
    fn from_code(mut code: IntCode) -> Self {
        code.compute(&mut Vec::new());
        let output = code.output;
        let chars: String = output
            .iter()
            .map(|n| char::from_u32(*n as u32).unwrap())
            .collect();

        //Print the grid
        //println!("{}", chars);

        let pixels_with_pos: Vec<(Pos, char)> = chars
            .lines()
            .map(|line| line.to_string().chars().collect::<Vec<char>>())
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .map(|(x, pixel)| (Pos(x + 1, y + 1), pixel)) //Add 1 to avoid "0" coords
                    .filter(|(_, pix)| *pix != '.') //Remove empty pixels
                    .collect::<Vec<(Pos, char)>>()
            })
            .collect();

        //Extract the robot
        let (robot_pos, robot_pixel): &(Pos, char) =
            pixels_with_pos.iter().find(|(_, pix)| *pix != '#').unwrap();
        let robot: Robot = Robot::from_pos_and_pixel(*robot_pos, robot_pixel);

        let scaffolds: HashSet<Pos> = pixels_with_pos.into_iter().map(|(pos, _)| pos).collect();

        Scaffolding { scaffolds, robot }
    }

    fn alignement_parameters_sum(&self) -> usize {
        self.scaffolds
            .iter()
            .filter_map(|&Pos(x, y)| {
                let ngbs: Vec<Pos> = Pos(x, y).neighbours();
                if ngbs.iter().all(|ngb| self.scaffolds.contains(ngb)) {
                    Some((x - 1) * (y - 1)) // Substract the 1 we added to avoid "0" coords
                } else {
                    None
                }
            })
            .sum()
    }

    fn robot_path(&self) -> String {
        fn move_ahead(Pos(x, y): Pos, dir: &Dir) -> Pos {
            match dir {
                Dir::North => Pos(x, y - 1),
                Dir::East => Pos(x + 1, y),
                Dir::South => Pos(x, y + 1),
                Dir::West => Pos(x - 1, y),
            }
        }
        fn move_left(pos: Pos, dir: &Dir) -> Pos {
            move_ahead(pos, &dir.turn_left())
        }
        fn move_right(pos: Pos, dir: &Dir) -> Pos {
            move_ahead(pos, &dir.turn_right())
        }
        let mut moving_robot: Robot = self.robot;
        let mut path: Path = Path {
            current: 0,
            steps: Vec::new(),
        };
        loop {
            //Move ahead
            let ahead: Pos = move_ahead(moving_robot.pos, &moving_robot.dir);
            if self.scaffolds.contains(&ahead) {
                path.advance();
                moving_robot.pos = ahead;
            } else {
                //Move right
                let right: Pos = move_right(moving_robot.pos, &moving_robot.dir);
                if self.scaffolds.contains(&right) {
                    path.turn_right();
                    moving_robot.dir = moving_robot.dir.turn_right();
                } else {
                    //Move left
                    let left: Pos = move_left(moving_robot.pos, &moving_robot.dir);
                    if self.scaffolds.contains(&left) {
                        path.turn_left();
                        moving_robot.dir = moving_robot.dir.turn_left();
                    } else {
                        path.stop_advance();
                        break;
                    }
                }
            }
        }
        path.path_string()
    }
}

struct Movement {
    routine: String,
    function_a: String,
    function_b: String,
    function_c: String,
}

impl Movement {
    fn to_input(&self) -> Vec<isize> {
        let input: String = format!(
            "{}\n{}\n{}\n{}\nn\n",
            self.routine, self.function_a, self.function_b, self.function_c
        );
        input.as_bytes().iter().map(|b| *b as isize).collect()
    }
}

impl FromStr for Movement {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn has_pattern(path: String, acc: Vec<String>) -> Option<Vec<String>> {
            let mut fun_len = 20;
            while fun_len > 0 {
                let mut rest = path.clone();
                let slice: String = rest[..fun_len].to_string();
                let last: char = slice.chars().last().unwrap();
                if last.is_ascii_digit() && rest[fun_len..].contains(&slice) {
                    rest = rest.replace(&slice, "").replace(",,", ",");
                    rest = rest.strip_prefix(',').unwrap_or(&rest).to_string();
                    rest = rest.strip_suffix(',').unwrap_or(&rest).to_string();

                    let mut new_acc: Vec<String> = acc.clone();
                    new_acc.push(slice);
                    //End condition
                    if acc.len() == 2 {
                        if rest.is_empty() {
                            return Some(new_acc);
                        }
                    } else if let Some(v) = has_pattern(rest.clone(), new_acc) {
                        return Some(v);
                    }
                }
                fun_len -= 1;
            }
            None
        }

        if let Some(f) = has_pattern(s.to_string(), Vec::new()) {
            let routine = s
                .replace(&f[0], "A")
                .replace(&f[1], "B")
                .replace(&f[2], "C");

            let mut iter = f.into_iter();

            Ok(Movement {
                routine,
                function_a: iter.next().unwrap(),
                function_b: iter.next().unwrap(),
                function_c: iter.next().unwrap(),
            })
        } else {
            Err(())
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_17.txt").expect("Cannot open input file");
    let mut code: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut code_2: IntCode = code.clone();
    code.compute(&mut Vec::new());

    let scaffolding: Scaffolding = Scaffolding::from_code(code);

    //scaffolding.print();

    println!(
        "Part1: The sum of the alignement parameters is {}",
        scaffolding.alignement_parameters_sum()
    );

    println!(
        "\nThe vacuum robot full path is:\n{}",
        scaffolding.robot_path()
    );
    let p = scaffolding.robot_path();
    let movements: Movement = Movement::from_str(&p).unwrap();

    //Change first op to "2"
    code_2.ops[0] = 2;
    code_2.compute(&mut movements.to_input());
    println!(
        "Part2: The vacuum robot collected {} dust",
        code_2.output.last().unwrap()
    );
    println!("Computing time: {:?}", now.elapsed());
}
