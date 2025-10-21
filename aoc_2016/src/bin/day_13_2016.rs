use std::cmp::Ordering;
use std::collections::HashSet;
use util::coord::Pos;

const SIZE: usize = 50;

#[derive(Debug)]
struct Node {
    pos: Pos,
    depth: usize,
}

impl Node {
    fn score(&self, Pos(x, y): Pos) -> usize {
        let dist: usize = x.abs_diff(self.pos.0) + y.abs_diff(self.pos.1);
        dist + self.depth
    }

    fn eq(&self, target: Pos) -> bool {
        self.pos == target
    }
}

struct Maze {
    seed: usize,
    grid: [[Option<bool>; SIZE]; SIZE],
}

impl Maze {
    fn get(&mut self, Pos(x, y): Pos) -> bool {
        match self.grid[y][x] {
            Some(b) => b,
            None => {
                let b = Maze::is_open(Pos(x, y), self.seed);
                self.grid[y][x] = Some(b);
                b
            }
        }
    }

    fn is_open(Pos(x, y): Pos, seed: usize) -> bool {
        let s: usize = x * x + 3 * x + 2 * x * y + y + y * y + seed;
        s.count_ones().is_multiple_of(2)
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
    let target: Pos = Pos(31, 39);
    //////////////

    let mut maze: Maze = Maze {
        seed,
        grid: [[None; SIZE]; SIZE],
    };

    //Part1: Depth First Search
    let mut visited_nodes: HashSet<Pos> = HashSet::from([Pos(1, 1)]);
    let mut current_nodes: Vec<Node> = vec![Node {
        pos: Pos(1, 1),
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
    let mut visited_nodes: HashSet<Pos> = HashSet::from([Pos(1, 1)]);
    let mut current_nodes: Vec<Pos> = vec![Pos(1, 1)];
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

fn neighbours(pos @ Pos(x, y): Pos) -> Vec<Pos> {
    match pos {
        Pos(0, 0) => vec![Pos(0, 1), Pos(1, 0)],
        Pos(0, _) => vec![Pos(0, y - 1), Pos(0, y + 1), Pos(1, y)],
        Pos(_, 0) => vec![Pos(x - 1, 0), Pos(x + 1, 0), Pos(x, 1)],
        _ => vec![Pos(x, y - 1), Pos(x, y + 1), Pos(x - 1, y), Pos(x + 1, y)],
    }
}
