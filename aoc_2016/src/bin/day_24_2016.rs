use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashSet;

const NB_NODE: usize = 8;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
struct Pos {
    x: usize,
    y: usize,
}

impl Pos {
    fn dist(&self, other: Pos) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }

    fn neighbours(&self, x_max: usize, y_max: usize) -> Vec<Self> {
        if self.x == 0 || self.x == x_max || self.y == 0 || self.y == y_max {
            Vec::new()
        } else {
            vec![
                Pos {
                    x: self.x,
                    y: self.y - 1,
                },
                Pos {
                    x: self.x,
                    y: self.y + 1,
                },
                Pos {
                    x: self.x - 1,
                    y: self.y,
                },
                Pos {
                    x: self.x + 1,
                    y: self.y,
                },
            ]
        }
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct Node {
    pos: Pos,
    moved: usize,
    estimate: usize,
}

impl Node {
    fn score(&self) -> usize {
        self.moved + self.estimate
    }
}

struct Maze {
    grid: Vec<Vec<bool>>,
    x_max: usize,
    y_max: usize,
}

impl Maze {
    fn new(grid: Vec<Vec<bool>>) -> Self {
        let x_max = grid[0].len();
        let y_max = grid.len();
        Maze { grid, x_max, y_max }
    }
    fn dist(&self, src: Pos, tgt: Pos) -> usize {
        let mut current_nodes: Vec<Node> = vec![Node {
            pos: src,
            moved: 0,
            estimate: src.dist(tgt),
        }];
        let mut visited_nodes: HashSet<Pos> = HashSet::new();
        visited_nodes.insert(src);
        loop {
            let best_node: Node = current_nodes.pop().unwrap();
            if best_node.pos == tgt {
                return best_node.moved;
            }
            let candidates: Vec<Node> = self
                .neighbours(best_node.pos)
                .into_iter()
                .filter(|&pos| visited_nodes.insert(pos))
                .map(|pos| Node {
                    pos,
                    moved: best_node.moved + 1,
                    estimate: pos.dist(tgt),
                })
                .collect();
            candidates.into_iter().for_each(|node| {
                let idx: usize =
                    current_nodes.partition_point(|other| match other.score().cmp(&node.score()) {
                        Ordering::Less => false,
                        Ordering::Greater => true,
                        Ordering::Equal => other.moved > node.moved,
                    });
                current_nodes.insert(idx, node);
            })
        }
    }

    fn neighbours(&self, pos: Pos) -> Vec<Pos> {
        pos.neighbours(self.x_max - 1, self.y_max - 1)
            .into_iter()
            .filter(|p| self.is_path(p))
            .collect()
    }

    fn is_path(&self, pos: &Pos) -> bool {
        self.grid[pos.y][pos.x]
    }

    fn print(&self) {
        self.grid.iter().for_each(|row| {
            row.iter().for_each(|&b| {
                let c: char = if b { '.' } else { '#' };
                print!("{c}");
            });
            println!();
        })
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_24.txt").expect("Cannot open input file");

    //There are 8 marked number in the maze (0 to 7)
    let mut to_visit: [Pos; NB_NODE] = [Pos { x: 0, y: 0 }; NB_NODE];

    let grid: Vec<Vec<bool>> = s
        .lines()
        .enumerate()
        .map(|(y, s)| {
            let chars: Vec<char> = s.chars().collect();
            chars
                .iter()
                .enumerate()
                .map(|(x, c)| match c {
                    '#' => false,
                    '.' => true,
                    _ => {
                        let n: usize = c.to_digit(10).unwrap() as usize;
                        to_visit[n] = Pos { x, y };
                        true
                    }
                })
                .collect()
        })
        .collect();

    let maze = Maze::new(grid);
    maze.print();

    //Fill the distance matrix for every Node couple to visit
    let mut distance_matrix: [[usize; NB_NODE]; NB_NODE] = [[0; NB_NODE]; NB_NODE];
    for i in 0..NB_NODE - 1 {
        for j in i + 1..NB_NODE {
            let dist: usize = maze.dist(to_visit[i], to_visit[j]);
            distance_matrix[i][j] = dist;
            distance_matrix[j][i] = dist;
        }
    }

    //Compute the total length to travel for every permutation (starting with 0)
    let other_nodes: Vec<usize> = vec![1, 2, 3, 4, 5, 6, 7];
    let min_traveled: usize = other_nodes
        .iter()
        .permutations(NB_NODE - 1)
        .map(|perm| {
            let mut v: Vec<usize> = perm.iter().map(|n| **n).collect();
            v.push(0);
            v.windows(2)
                .map(|pair| distance_matrix[pair[0]][pair[1]])
                .sum()
        })
        .min()
        .unwrap();

    println!(
        "Part1: The shortest path to every node starting at 0 is {} steps long",
        min_traveled
    );

    let min_traveled_with_return: usize = other_nodes
        .iter()
        .permutations(NB_NODE - 1)
        .map(|perm| {
            let mut v: Vec<usize> = perm.iter().map(|n| **n).collect();
            v.push(0);
            v.reverse();
            v.push(0);
            v.windows(2)
                .map(|pair| distance_matrix[pair[0]][pair[1]])
                .sum()
        })
        .min()
        .unwrap();

    println!(
        "Part2: The shortest path to every node starting at 0 and coming back to 0 is {} steps long",
        min_traveled_with_return
    );
}
