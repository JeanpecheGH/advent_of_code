use std::cmp::Ordering;
use std::collections::HashSet;

const SIZE: usize = 50;

#[derive(Debug)]
struct Node {
    pos: (usize, usize),
    depth: usize,
}

impl Node {
    fn score(&self, (x, y): (usize, usize)) -> usize {
        let dist: usize = Self::dist_abs(x, self.pos.0) + Self::dist_abs(y, self.pos.1);
        dist + self.depth
    }

    fn dist_abs(a: usize, b: usize) -> usize {
        if a > b {
            a - b
        } else {
            b - a
        }
    }

    fn eq(&self, target: (usize, usize)) -> bool {
        self.pos == target
    }
}

struct Maze {
    seed: usize,
    grid: [[Option<bool>; SIZE]; SIZE],
}

impl Maze {
    fn get(&mut self, (x, y): (usize, usize)) -> bool {
        match self.grid[y][x] {
            Some(b) => b,
            None => {
                let b = Maze::is_open((x, y), self.seed);
                self.grid[y][x] = Some(b);
                b
            }
        }
    }

    fn is_open((x, y): (usize, usize), seed: usize) -> bool {
        let s: usize = x * x + 3 * x + 2 * x * y + y + y * y + seed;
        s.count_ones() % 2 == 0
    }

    fn print(&self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid[y].len() {
                let c: char = match self.grid[y][x] {
                    Some(true) => '.',
                    Some(false) => '#',
                    None => ' ',
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    //Input Data//
    let seed: usize = 1350;
    let target: (usize, usize) = (31, 39);
    //////////////

    let mut maze: Maze = Maze {
        seed,
        grid: [[None; SIZE]; SIZE],
    };

    //Part1: Depth First Search
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::from([(1, 1)]);
    let mut current_nodes: Vec<Node> = vec![Node {
        pos: (1, 1),
        depth: 0,
    }];
    loop {
        let first_node: Node = current_nodes.pop().unwrap();
        if first_node.eq(target) {
            println!(
                "Part1: Found the shortest path to {:?} in {} moves",
                target, first_node.depth
            );
            break;
        }
        let mut new_nodes: Vec<Node> = neighbours(first_node.pos)
            .into_iter()
            .filter(|&pos| maze.get(pos) && visited_nodes.insert(pos))
            .map(|pos| Node {
                pos,
                depth: first_node.depth + 1,
            })
            .collect();
        new_nodes.sort_by(|a, b| match a.score(target).cmp(&b.score(target)) {
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less,
            Ordering::Equal => b.depth.cmp(&a.depth),
        });
        current_nodes.extend(new_nodes);
    }

    maze.print();

    //Part2: Breadth First search
    //reset maze
    maze = Maze {
        seed,
        grid: [[None; SIZE]; SIZE],
    };
    let mut visited_nodes: HashSet<(usize, usize)> = HashSet::from([(1, 1)]);
    let mut current_nodes: Vec<(usize, usize)> = vec![(1, 1)];
    for _ in 0..50 {
        current_nodes = current_nodes
            .into_iter()
            .flat_map(neighbours)
            .filter(|&pos| maze.get(pos) && visited_nodes.insert(pos))
            .collect();
    }
    println!(
        "Part2: At depth 50, Number of total positions computed: {}, Number of positions for this step: {}",
        visited_nodes.len(),
        current_nodes.len(),
    );
    maze.print();
}

fn neighbours((x, y): (usize, usize)) -> Vec<(usize, usize)> {
    match (x, y) {
        (0, 0) => vec![(0, 1), (1, 0)],
        (0, _) => vec![(0, y - 1), (0, y + 1), (1, y)],
        (_, 0) => vec![(x - 1, 0), (x + 1, 0), (x, 1)],
        _ => vec![(x, y - 1), (x, y + 1), (x - 1, y), (x + 1, y)],
    }
}
