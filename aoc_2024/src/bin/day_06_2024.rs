use fxhash::FxHashSet;
use rayon::prelude::*;
use std::collections::BTreeSet;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Debug)]
struct Patrol {
    pos: Pos,
    dir: Dir,
    columns: Vec<BTreeSet<usize>>,
    rows: Vec<BTreeSet<usize>>,
    max_x: usize,
    max_y: usize,
}

impl Patrol {
    fn with_new_obstacle(&self, Pos(x, y): Pos) -> Patrol {
        let mut columns = self.columns.clone();
        let mut rows = self.rows.clone();

        columns[x].insert(y);
        rows[y].insert(x);

        Patrol {
            pos: self.pos,
            dir: self.dir,
            columns,
            rows,
            max_x: self.max_x,
            max_y: self.max_y,
        }
    }
    fn next_obstacle(&self, Pos(x, y): Pos, dir: Dir) -> Option<Pos> {
        match dir {
            Dir::North => self.columns[x]
                .range(..y)
                .next_back()
                .map(|&new_y| Pos(x, new_y)),
            Dir::East => self.rows[y]
                .range(x + 1..)
                .next()
                .map(|&new_x| Pos(new_x, y)),
            Dir::South => self.columns[x]
                .range(y + 1..)
                .next()
                .map(|&new_y| Pos(x, new_y)),
            Dir::West => self.rows[y]
                .range(..x)
                .next_back()
                .map(|&new_x| Pos(new_x, y)),
        }
    }

    fn next_pos(Pos(x, y): Pos, dir: Dir, Pos(obs_x, obs_y): Pos) -> (Pos, Dir) {
        match dir {
            Dir::North => (Pos(x, obs_y + 1), Dir::East),
            Dir::East => (Pos(obs_x - 1, y), Dir::South),
            Dir::South => (Pos(x, obs_y - 1), Dir::West),
            Dir::West => (Pos(obs_x + 1, y), Dir::North),
        }
    }

    fn new_visited_pos(Pos(x, y): Pos, dir: Dir, Pos(n_x, n_y): Pos) -> Vec<Pos> {
        match dir {
            Dir::North => (n_y..=y).map(|h| Pos(x, h)).collect(),
            Dir::East => (x..=n_x).map(|h| Pos(h, y)).collect(),
            Dir::South => (y..=n_y).map(|h| Pos(x, h)).collect(),
            Dir::West => (n_x..=x).map(|h| Pos(h, y)).collect(),
        }
    }

    fn last_pos_in(&self, Pos(x, y): Pos, dir: Dir) -> Pos {
        match dir {
            Dir::North => Pos(x, 0),
            Dir::East => Pos(self.max_x - 1, y),
            Dir::South => Pos(x, self.max_y - 1),
            Dir::West => Pos(0, y),
        }
    }

    fn visited(&self) -> FxHashSet<Pos> {
        let mut pos: Pos = self.pos;
        let mut dir: Dir = self.dir;
        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        visited.insert(pos);

        while let Some(obs) = self.next_obstacle(pos, dir) {
            //Compute next position
            let (next_pos, next_dir) = Patrol::next_pos(pos, dir, obs);
            //Visit all position between old position and new one
            let new_visited = Patrol::new_visited_pos(pos, dir, next_pos);
            visited.extend(new_visited);
            //Move to next pos
            pos = next_pos;
            dir = next_dir;
        }
        //Visit all position between old position and getting out
        let out: Pos = self.last_pos_in(pos, dir);
        let new_visited = Patrol::new_visited_pos(pos, dir, out);
        visited.extend(new_visited);
        visited
    }

    fn is_loop(&self) -> bool {
        let mut pos: Pos = self.pos;
        let mut dir: Dir = self.dir;
        let mut visited: FxHashSet<(Pos, Dir)> = FxHashSet::default();

        while let Some(obs) = self.next_obstacle(pos, dir) {
            //Compute next position
            let (next_pos, next_dir) = Patrol::next_pos(pos, dir, obs);
            //Check if we're in a loop
            if !visited.insert((next_pos, next_dir)) {
                return true;
            } //Move to next pos
            pos = next_pos;
            dir = next_dir;
        }

        false
    }

    fn solve(&self) -> (usize, usize) {
        let visited: FxHashSet<Pos> = self.visited();
        let nb_loop: usize = visited
            .par_iter()
            .filter(|&&p| self.with_new_obstacle(p).is_loop())
            .count();
        (visited.len(), nb_loop)
    }
}
impl FromStr for Patrol {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut pos: Pos = Pos(0, 0);
        let lines: Vec<&str> = s.lines().collect();
        let max_x: usize = lines[0].len();
        let max_y: usize = lines.len();
        let mut columns: Vec<BTreeSet<usize>> = vec![BTreeSet::default(); max_x];
        let mut rows: Vec<BTreeSet<usize>> = vec![BTreeSet::default(); max_y];

        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                match c {
                    '#' => {
                        columns[x].insert(y);
                        rows[y].insert(x);
                    }
                    '^' => pos = Pos(x, y),
                    _ => (),
                }
            }
        }

        Ok(Patrol {
            pos,
            dir: Dir::North,
            columns,
            rows,
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
