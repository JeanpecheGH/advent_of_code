use fxhash::FxHashSet;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

struct Patrol {
    pos: Pos,
    dir: Dir,
    obstructions: FxHashSet<Pos>,
    max_x: usize,
    max_y: usize,
}

impl Patrol {
    fn patrol(&self, new_obstruction: Option<Pos>) -> Option<FxHashSet<Pos>> {
        fn gets_out(Pos(x, y): Pos, dir: Dir, max_x: usize, max_y: usize) -> bool {
            (y == 0 && dir == Dir::North)
                || (x == 0 && dir == Dir::West)
                || (x == max_x - 1 && dir == Dir::East)
                || (y == max_y - 1 && dir == Dir::South)
        }

        fn in_front(Pos(x, y): Pos, dir: &Dir) -> Pos {
            match dir {
                Dir::North => Pos(x, y - 1),
                Dir::East => Pos(x + 1, y),
                Dir::South => Pos(x, y + 1),
                Dir::West => Pos(x - 1, y),
            }
        }

        let mut pos: Pos = self.pos;
        let mut dir: Dir = self.dir;
        let mut obstructions: FxHashSet<Pos> = self.obstructions.clone();
        new_obstruction.iter().for_each(|&o| {
            let _ = obstructions.insert(o);
        });
        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        visited.insert(pos);

        let mut visited_with_orient: FxHashSet<(Pos, Dir)> = FxHashSet::default();
        visited_with_orient.insert((pos, dir));

        while !gets_out(pos, dir, self.max_x, self.max_y) {
            let next: Pos = in_front(pos, &dir);
            if obstructions.contains(&next) {
                dir = dir.turn_right();
                if !visited_with_orient.insert((pos, dir)) {
                    return None;
                }
            } else {
                pos = next;
                visited.insert(pos);
                if !visited_with_orient.insert((pos, dir)) {
                    return None;
                }
            }
        }

        Some(visited)
    }

    fn solve(&self) -> (usize, usize) {
        let visited: FxHashSet<Pos> = self.patrol(None).unwrap();
        let nb_visited: usize = visited.len();
        let obstructions: usize = visited
            .par_iter()
            .filter(|&&pos| self.pos != pos && self.patrol(Some(pos)).is_none())
            .count();

        (nb_visited, obstructions)
    }
}

impl FromStr for Patrol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut obstructions: FxHashSet<Pos> = FxHashSet::default();
        let mut pos: Pos = Pos(0, 0);
        let lines: Vec<&str> = s.lines().collect();
        let max_x: usize = lines[0].len();
        let max_y: usize = lines.len();

        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        let _ = obstructions.insert(Pos(x, y));
                    }
                    '^' => pos = Pos(x, y),
                    _ => (),
                }
            }
        }

        Ok(Patrol {
            pos,
            dir: Dir::North,
            obstructions,
            max_x,
            max_y,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_06.txt").expect("Cannot open input file");
    let patrol: Patrol = s.parse().unwrap();

    let (visited, obstructions) = patrol.solve();
    println!("Part1: The guard will visit {visited} positions");
    println!("Part2: {obstructions} obstructions can be added to send the guard in a loop");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...
";
    #[test]
    fn test() {
        let patrol: Patrol = EXAMPLE_1.parse().unwrap();
        assert_eq!(patrol.solve(), (41, 6));
    }
}
