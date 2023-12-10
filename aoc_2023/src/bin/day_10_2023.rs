use std::collections::HashSet;
use std::str::FromStr;
use util::coord::PosI;
use util::orientation::Dir;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum Pipe {
    Ground,
    Start,
    Join(Dir, Dir),
}

impl Pipe {
    fn from_char(c: char) -> Pipe {
        match c {
            'S' => Pipe::Start,
            '|' => Pipe::Join(Dir::North, Dir::South),
            '-' => Pipe::Join(Dir::West, Dir::East),
            'L' => Pipe::Join(Dir::North, Dir::East),
            'J' => Pipe::Join(Dir::North, Dir::West),
            'F' => Pipe::Join(Dir::South, Dir::East),
            '7' => Pipe::Join(Dir::South, Dir::West),
            _ => Pipe::Ground,
        }
    }
}

struct PipeMaze {
    pipes: Vec<Vec<Pipe>>,
}

impl PipeMaze {
    fn pipes_in_loop(&self, start: PosI) -> HashSet<PosI> {
        let mut pipes_in_loop: HashSet<PosI> = HashSet::from([start]);
        let mut current: Option<PosI> = Some(start);
        while let Some(curr) = current {
            let ngbs = curr.neighbours();
            current = ngbs
                .into_iter()
                .find(|&ngb| self.connects_to(curr, ngb) && pipes_in_loop.insert(ngb));
        }
        pipes_in_loop
    }

    fn start(&self) -> PosI {
        self.pipes
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, pipe)| {
                    if *pipe == Pipe::Start {
                        Some(PosI(x as isize, y as isize))
                    } else {
                        None
                    }
                })
            })
            .unwrap()
    }

    fn pipe_from_start(&self, start: PosI) -> Pipe {
        let PosI(x, y) = start;
        let up: PosI = PosI(x, y - 1);
        let down: PosI = PosI(x, y + 1);
        let left: PosI = PosI(x - 1, y);
        let right: PosI = PosI(x + 1, y);

        match (
            self.connects_to(up, start),
            self.connects_to(down, start),
            self.connects_to(left, start),
            self.connects_to(right, start),
        ) {
            (true, true, false, false) => Pipe::Join(Dir::North, Dir::South),
            (false, false, true, true) => Pipe::Join(Dir::West, Dir::East),
            (true, false, true, false) => Pipe::Join(Dir::North, Dir::West),
            (true, false, false, true) => Pipe::Join(Dir::North, Dir::East),
            (false, true, true, false) => Pipe::Join(Dir::South, Dir::West),
            (false, true, false, true) => Pipe::Join(Dir::South, Dir::East),
            _ => panic!("Impossible pipe"),
        }
    }

    fn connects_to(&self, from: PosI, to: PosI) -> bool {
        let PosI(x, y) = from;
        if x >= 0 && x < self.pipes[0].len() as isize && y >= 0 && y < self.pipes.len() as isize {
            let pipe: Pipe = self.pipes[y as usize][x as usize];

            match pipe {
                Pipe::Join(a, b) => {
                    let diff_x: isize = from.0 - to.0;
                    let diff_y: isize = from.1 - to.1;
                    let dir: Dir = match (diff_x, diff_y) {
                        (1, 0) => Dir::West,
                        (-1, 0) => Dir::East,
                        (0, 1) => Dir::North,
                        (0, -1) => Dir::South,
                        _ => panic!("Impossible neighbour {:?} {:?}", from, to),
                    };
                    dir == a || dir == b
                }
                Pipe::Start => self.connects_to(to, from),
                _ => false,
            }
        } else {
            false
        }
    }

    fn compute(&self) -> (usize, usize) {
        let pipes_in_loop: HashSet<PosI> = self.pipes_in_loop(self.start());
        //Find the most top/left 'F' in the loop and start going East following the loop.
        //From there, we know "inside" is to our right
        let first_f: PosI = self
            .pipes
            .iter()
            .enumerate()
            .find_map(|(y, row)| {
                row.iter().enumerate().find_map(|(x, _)| {
                    let pos: PosI = PosI(x as isize, y as isize);
                    if pipes_in_loop.contains(&pos) {
                        Some(pos)
                    } else {
                        None
                    }
                })
            })
            .unwrap();

        //Find all the "inside" tiles directly touching the loop (we touch them with our right hand)
        let mut curr: PosI = PosI(first_f.0 + 1, first_f.1);
        let mut dir = Dir::West;
        let mut touching_inside: HashSet<PosI> = HashSet::new();

        while curr != first_f {
            let (new_curr, new_dir, insides) = self.next_and_inside_neighbours(curr, dir);
            insides.into_iter().for_each(|p| {
                if !pipes_in_loop.contains(&p) {
                    touching_inside.insert(p);
                }
            });
            curr = new_curr;
            dir = new_dir;
        }

        //We have to count all the inside tiles not touching the loop too
        let mut to_flood: Vec<PosI> = touching_inside.into_iter().collect();
        let mut flooded: HashSet<PosI> = HashSet::new();

        while let Some(p) = to_flood.pop() {
            flooded.insert(p);
            p.neighbours().into_iter().for_each(|ngb| {
                if !pipes_in_loop.contains(&ngb) && !flooded.contains(&ngb) {
                    to_flood.push(ngb)
                }
            })
        }

        (pipes_in_loop.len() / 2, flooded.len())
    }

    fn next_and_inside_neighbours(
        &self,
        PosI(x, y): PosI,
        from_dir: Dir,
    ) -> (PosI, Dir, Vec<PosI>) {
        let pipe: Pipe = self.pipes[y as usize][x as usize];

        let pipe: Pipe = if pipe == Pipe::Start {
            self.pipe_from_start(PosI(x, y))
        } else {
            pipe
        };

        match (pipe, from_dir) {
            //Horizontal pipes
            (Pipe::Join(Dir::West, Dir::East), Dir::West) => {
                (PosI(x + 1, y), Dir::West, vec![PosI(x, y + 1)])
            }
            (Pipe::Join(Dir::West, Dir::East), Dir::East) => {
                (PosI(x - 1, y), Dir::East, vec![PosI(x, y - 1)])
            }
            //Vertical pipes
            (Pipe::Join(Dir::North, Dir::South), Dir::North) => {
                (PosI(x, y + 1), Dir::North, vec![PosI(x - 1, y)])
            }
            (Pipe::Join(Dir::North, Dir::South), Dir::South) => {
                (PosI(x, y - 1), Dir::South, vec![PosI(x + 1, y)])
            }
            //F pipes
            (Pipe::Join(Dir::South, Dir::East), Dir::South) => {
                (PosI(x + 1, y), Dir::West, Vec::new())
            }
            (Pipe::Join(Dir::South, Dir::East), Dir::East) => (
                PosI(x, y + 1),
                Dir::North,
                vec![PosI(x, y - 1), PosI(x - 1, y)],
            ),
            //7 pipes
            (Pipe::Join(Dir::South, Dir::West), Dir::South) => (
                PosI(x - 1, y),
                Dir::East,
                vec![PosI(x, y - 1), PosI(x + 1, y)],
            ),
            (Pipe::Join(Dir::South, Dir::West), Dir::West) => {
                (PosI(x, y + 1), Dir::North, Vec::new())
            }
            //L pipes
            (Pipe::Join(Dir::North, Dir::East), Dir::North) => (
                PosI(x + 1, y),
                Dir::West,
                vec![PosI(x, y + 1), PosI(x - 1, y)],
            ),
            (Pipe::Join(Dir::North, Dir::East), Dir::East) => {
                (PosI(x, y - 1), Dir::South, Vec::new())
            }
            //J pipes
            (Pipe::Join(Dir::North, Dir::West), Dir::North) => {
                (PosI(x - 1, y), Dir::East, Vec::new())
            }

            (Pipe::Join(Dir::North, Dir::West), Dir::West) => (
                PosI(x, y - 1),
                Dir::South,
                vec![PosI(x, y + 1), PosI(x + 1, y)],
            ),
            _ => panic!(
                "Impossible, {:?} cannot reach {:?} at {:?}",
                from_dir,
                pipe,
                PosI(x, y)
            ),
        }
    }
}

impl FromStr for PipeMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pipes: Vec<Vec<Pipe>> = s
            .lines()
            .map(|l| l.chars().map(Pipe::from_char).collect::<Vec<Pipe>>())
            .collect();
        Ok(PipeMaze { pipes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_10.txt").expect("Cannot open input file");
    let maze: PipeMaze = s.parse().unwrap();

    let (half_loop, inside_area) = maze.compute();
    assert_eq!(half_loop, 6828);
    assert_eq!(inside_area, 459);

    println!(
        "Part1: It takes {} steps to get to the furthers point from the start",
        half_loop
    );
    println!("Part2: {} tiles are enclosed by the loop", inside_area);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "-L|F7
7S-7|
L|7||
-L-J|
L|-JF
";

    const EXAMPLE_2: &str = "7-F7-
.FJ|7
SJLL7
|F--J
LJ.LJ
";

    const EXAMPLE_3: &str = "..........
.S------7.
.|F----7|.
.||....||.
.||....||.
.|L-7F-J|.
.|..||..|.
.L--JL--J.
..........
";

    const EXAMPLE_4: &str = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...
";

    const EXAMPLE_5: &str = "FF7FSF7F7F7F7F7F---7
L|LJ||||||||||||F--J
FL-7LJLJ||||||LJL-77
F--JF--7||LJLJ7F7FJ-
L---JF-JLJ.||-FJLJJ7
|F|F-JF---7F7-L7L|7|
|FFJF7L7F-JF7|JL---7
7-L-JL7||F7|L7F-7F7|
L.L7LFJ|||||FJL7||LJ
L7JLJL-JLJLJL--JLJ.L
";

    #[test]
    fn part_1_test_1() {
        let maze: PipeMaze = EXAMPLE_1.parse().unwrap();
        assert_eq!(maze.compute().0, 4);
    }
    #[test]
    fn part_1_test_2() {
        let maze: PipeMaze = EXAMPLE_2.parse().unwrap();
        assert_eq!(maze.compute().0, 8);
    }

    #[test]
    fn part_2_test_1() {
        let maze: PipeMaze = EXAMPLE_3.parse().unwrap();
        assert_eq!(maze.compute().1, 4);
    }
    #[test]
    fn part_2_test_2() {
        let maze: PipeMaze = EXAMPLE_4.parse().unwrap();
        assert_eq!(maze.compute().1, 8);
    }
    #[test]
    fn part_2_test_3() {
        let maze: PipeMaze = EXAMPLE_5.parse().unwrap();
        assert_eq!(maze.compute().1, 10);
    }
}
