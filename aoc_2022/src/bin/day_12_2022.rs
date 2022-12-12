use std::cmp::max;
use std::collections::HashSet;

type Pos = (usize, usize);

#[derive(Debug)]
struct Terrain {
    grid: Vec<Vec<u8>>,
    end: Pos,
}

impl Terrain {
    fn a_star(&self, &pos: &Pos) -> usize {
        let start: Node = Node {
            pos,
            dist: 0,
            height: self.grid[pos.1][pos.0],
        };
        let end: Node = Node {
            pos: self.end,
            dist: 0,
            height: self.grid[self.end.1][self.end.0],
        };

        let mut nodes: Vec<Node> = vec![start];
        let mut visited_pos: HashSet<Pos> = HashSet::new();
        visited_pos.insert(pos);
        loop {
            if let Some(best_node) = nodes.pop() {
                if best_node.pos == end.pos {
                    return best_node.dist;
                }
                let new_nodes: Vec<Node> = self.neighbours(best_node);

                new_nodes
                    .into_iter()
                    //Filter already visited positions
                    .filter(|node| visited_pos.insert(node.pos))
                    .for_each(|node| {
                        let i: usize = nodes.partition_point(|x| x.score(&end) > node.score(&end));
                        nodes.insert(i, node);
                    })
            } else {
                //The node cannot reach the end
                return usize::MAX;
            }
        }
    }

    fn neighbours(&self, node: Node) -> Vec<Node> {
        //First we describe the 4 directions
        vec![(-1, 0), (1, 0), (0, -1), (0, 1)]
            .iter()
            //Compute the 4 neighbours of the Node
            .map(|(i, j)| (node.pos.0 as isize + i, node.pos.1 as isize + j))
            //Remove nodes out of the grid
            .filter(|(i, j)| {
                (0..self.width() as isize).contains(i) && (0..self.height() as isize).contains(j)
            })
            //Get the height of the Node
            .map(|(i, j)| (i as usize, j as usize, self.grid[j as usize][i as usize]))
            //Remove nodes that are too high
            .filter(|(_, _, h)| *h <= node.height + 1)
            .map(|(i, j, h)| Node {
                pos: (i, j),
                dist: node.dist + 1,
                height: h,
            })
            .collect()
    }

    fn width(&self) -> usize {
        self.grid[0].len()
    }
    fn height(&self) -> usize {
        self.grid.len()
    }
}

struct Node {
    pos: Pos,
    dist: usize,
    height: u8,
}

impl Node {
    fn heuristic(&self, other: &Node) -> usize {
        let dist_diff: usize = self.pos.0.abs_diff(other.pos.0) + self.pos.1.abs_diff(other.pos.1);
        let height_diff: usize = other.height.abs_diff(self.height) as usize;
        max(dist_diff, height_diff)
    }

    fn score(&self, other: &Node) -> usize {
        self.dist + self.heuristic(other)
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_12.txt").expect("Cannot open input file");

    let mut start: Pos = (0, 0);
    let mut end: Pos = (0, 0);
    let grid: Vec<Vec<u8>> = s
        .lines()
        .enumerate()
        .map(|(j, s)| {
            s.bytes()
                .enumerate()
                .map(|(i, c)| match c {
                    b'a'..=b'z' => c - b'a',
                    b'S' => {
                        start = (i, j);
                        0
                    }
                    b'E' => {
                        end = (i, j);
                        b'z' - b'a'
                    }
                    _ => 0,
                })
                .collect()
        })
        .collect();

    let terrain: Terrain = Terrain { grid, end };

    let now = std::time::Instant::now();
    let short_to_the_top = terrain.a_star(&start);
    println!(
        "Part1: The shortest path to the top of the hill takes {} steps (found in {:?})",
        short_to_the_top,
        now.elapsed()
    );

    let now = std::time::Instant::now();
    //Candidates to this starting position are all at x=0
    //All the other 'a's are surrounded by 'c's
    let starts: Vec<Pos> = (0..terrain.height()).map(|j| (0, j)).collect();

    let shortest_path: usize = starts.iter().map(|pos| terrain.a_star(pos)).min().unwrap();
    println!("Part2: The shortest path from the top of the mountain to the nearest low level tile takes {} steps (found in {:?})", shortest_path, now.elapsed());
}
