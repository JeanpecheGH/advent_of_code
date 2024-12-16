use fxhash::{FxHashMap, FxHashSet};
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Clone, Debug, Eq, PartialEq)]
struct ReindeerNode {
    pos: Pos,
    dir: Dir,
    score: usize,
    h: usize,
    visited: FxHashSet<Pos>,
}

impl ReindeerNode {
    fn neighbours(&self, end: Pos) -> Vec<ReindeerNode> {
        fn heuristic(pos: Pos, dir: Dir, end: Pos) -> usize {
            let h: usize = match dir {
                Dir::North if pos.0 == end.0 => 0,
                Dir::East if pos.1 == end.1 => 0,
                Dir::South => 2000,
                Dir::West => 2000,
                _ => 1000,
            };
            h + pos.distance(end)
        }
        fn ahead(Pos(x, y): Pos, dir: Dir) -> Pos {
            match dir {
                Dir::North => Pos(x, y - 1),
                Dir::East => Pos(x + 1, y),
                Dir::South => Pos(x, y + 1),
                Dir::West => Pos(x - 1, y),
            }
        }
        vec![
            (self.dir, 1),
            (self.dir.turn_left(), 1001),
            (self.dir.turn_right(), 1001),
        ]
        .into_iter()
        .map(|(dir, add_score)| {
            let ahead_pos = ahead(self.pos, dir);
            let mut ahead_visited: FxHashSet<Pos> = self.visited.clone();
            ahead_visited.insert(ahead_pos);
            ReindeerNode {
                pos: ahead_pos,
                dir,
                score: self.score + add_score,
                h: heuristic(ahead_pos, dir, end),
                visited: ahead_visited,
            }
        })
        .collect()
    }
    fn heuristic(&self) -> usize {
        self.score + self.h
    }
}

impl Ord for ReindeerNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .heuristic()
            .cmp(&self.heuristic())
            .then(other.score.cmp(&self.score))
    }
}

impl PartialOrd for ReindeerNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct ReindeerMaze {
    grid: Vec<Vec<bool>>,
    start: Pos,
    start_dir: Dir,
    end: Pos,
}
impl ReindeerMaze {
    fn is_wall(&self, Pos(x, y): Pos) -> bool {
        self.grid[y][x]
    }
    fn solve(&self) -> (usize, usize) {
        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        visited.insert(self.start);
        let starting_node: ReindeerNode = ReindeerNode {
            pos: self.start,
            dir: self.start_dir,
            score: 0,
            h: 0,
            visited,
        };
        let mut cache: FxHashMap<(Pos, Dir), usize> = FxHashMap::default();
        cache.insert((self.start, self.start_dir), 0);
        let mut priority_queue: BinaryHeap<ReindeerNode> = BinaryHeap::default();
        priority_queue.push(starting_node);

        let mut min_score: usize = usize::MAX;
        let mut visited_pos: FxHashSet<Pos> = FxHashSet::default();

        while let Some(node) = priority_queue.pop() {
            if node.score <= min_score {
                if node.pos == self.end {
                    min_score = node.score;
                    visited_pos.extend(node.visited);
                } else {
                    // 3 possible neighbours : go ahead, turn left, turn right
                    node.neighbours(self.end)
                        .into_iter()
                        .filter(|n| !self.is_wall(n.pos))
                        .for_each(|n| {
                            let min = cache.entry((n.pos, n.dir)).or_insert(usize::MAX);
                            if n.score <= *min {
                                *min = n.score;
                                priority_queue.push(n);
                            }
                        });
                }
            }
        }
        (min_score, visited_pos.len())
    }
}

impl FromStr for ReindeerMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<bool>> = s
            .lines()
            .map(|l| l.chars().map(|c| c == '#').collect())
            .collect();
        let start: Pos = Pos(1, grid.len() - 2);
        let end: Pos = Pos(grid[0].len() - 2, 1);
        let start_dir: Dir = Dir::East;

        Ok(ReindeerMaze {
            grid,
            start,
            start_dir,
            end,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_16.txt").expect("Cannot open input file");
    let maze: ReindeerMaze = s.parse().unwrap();
    let (min_score, best_seats) = maze.solve();
    println!("Part1: The lowest possible score is {min_score}",);
    println!("Part2: The best seats are on {best_seats} tiles");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############
";

    const EXAMPLE_2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################
";

    #[test]
    fn test_1() {
        let maze: ReindeerMaze = EXAMPLE_1.parse().unwrap();
        assert_eq!(maze.solve(), (7036, 45));
    }
    #[test]
    fn test_2() {
        let maze: ReindeerMaze = EXAMPLE_2.parse().unwrap();
        assert_eq!(maze.solve(), (11048, 64));
    }
}
