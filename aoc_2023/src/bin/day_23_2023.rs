use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum Tile {
    Path,
    Forest,
    Slope(Dir),
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '.' => Tile::Path,
            '^' => Tile::Slope(Dir::North),
            'v' => Tile::Slope(Dir::South),
            '<' => Tile::Slope(Dir::West),
            '>' => Tile::Slope(Dir::East),
            _ => Tile::Forest,
        }
    }
}

#[derive(Copy, Clone, Debug)]
struct Node {
    pos: Pos,
    from: Pos,
    steps: usize,
}

impl Node {
    fn neighbours(&self, max_x: usize, max_y: usize) -> Vec<Node> {
        self.pos
            .neighbours_safe(max_x, max_y)
            .iter()
            .filter_map(|&p| {
                if p != self.from {
                    Some(Node {
                        pos: p,
                        from: self.pos,
                        steps: self.steps + 1,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct ANode {
    pos: Pos,
    from: Pos,
    steps: usize,
    h: usize,
}

impl ANode {
    fn neighbours(&self, max_x: usize, max_y: usize, to: Pos) -> Vec<ANode> {
        self.pos
            .neighbours_safe(max_x, max_y)
            .into_iter()
            .filter(|&p| p != self.from)
            .map(|p| ANode {
                pos: p,
                from: self.pos,
                steps: self.steps + 1,
                h: p.distance(to),
            })
            .collect()
    }
    fn score(&self) -> usize {
        self.steps + self.h
    }
}

impl Ord for ANode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.score().cmp(&self.score())
    }
}

impl PartialOrd for ANode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Debug)]
struct NoIceNode {
    pos: usize,
    visited: usize,
    steps: usize,
}

impl NoIceNode {
    fn neighbours(&self, graph: &HashMap<usize, usize>) -> Vec<NoIceNode> {
        graph
            .iter()
            .filter_map(|(&k, &v)| {
                let new_pos: usize = k ^ self.pos;
                if k & self.pos > 0 && self.visited & new_pos == 0 {
                    Some(NoIceNode {
                        pos: new_pos,
                        visited: self.visited | k,
                        steps: self.steps + v,
                    })
                } else {
                    None
                }
            })
            .collect()
    }
}

struct TrailMap {
    start: Pos,
    end: Pos,
    grid: Vec<Vec<Tile>>,
    max_x: usize,
    max_y: usize,
}

impl TrailMap {
    fn tile_at(&self, Pos(x, y): Pos) -> Tile {
        self.grid[y][x]
    }

    fn can_go_from_to(&self, from: Pos, to: Pos) -> bool {
        let Pos(x, y): Pos = from;
        let Pos(i, j): Pos = to;
        match self.tile_at(to) {
            Tile::Forest => false,
            Tile::Path => true,
            Tile::Slope(Dir::North) => y > j,
            Tile::Slope(Dir::South) => y < j,
            Tile::Slope(Dir::West) => x > i,
            Tile::Slope(Dir::East) => x < i,
        }
    }

    fn can_go_to(&self, to: Pos) -> bool {
        self.tile_at(to) != Tile::Forest
    }

    fn find_graph_nodes(&self) -> Vec<Pos> {
        let mut graph_nodes: Vec<Pos> = vec![self.start, self.end];
        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let pos = Pos(x, y);
                if self.can_go_to(pos) {
                    let ngbs: Vec<Pos> = pos
                        .neighbours_safe(self.max_x, self.max_y)
                        .into_iter()
                        .filter(|&p| self.can_go_to(p))
                        .collect();
                    if ngbs.len() >= 3 {
                        graph_nodes.push(pos)
                    }
                }
            }
        }
        graph_nodes
    }

    fn dist(&self, from: Pos, to: Pos, graph_nodes: &[Pos]) -> Option<usize> {
        let start_node: ANode = ANode {
            pos: from,
            from,
            steps: 0,
            h: from.distance(to),
        };
        let mut queue: BinaryHeap<ANode> = BinaryHeap::new();
        queue.push(start_node);

        while let Some(n) = queue.pop() {
            //We reach the target
            if n.pos == to {
                return Some(n.steps);
            }

            //We reach another node, stop here
            if n.steps > 0 && graph_nodes.contains(&n.pos) {
                continue;
            }

            n.neighbours(self.max_x, self.max_y, to)
                .into_iter()
                .filter(|&n| self.can_go_to(n.pos))
                .for_each(|n| queue.push(n));
        }
        None
    }

    fn build_graph(&self) -> HashMap<(Pos, Pos), usize> {
        let mut graph: HashMap<(Pos, Pos), usize> = HashMap::new();
        let graph_nodes: Vec<Pos> = self.find_graph_nodes();
        for i in 0..graph_nodes.len() {
            for j in i + 1..graph_nodes.len() {
                if let Some(dist) = self.dist(graph_nodes[i], graph_nodes[j], &graph_nodes) {
                    graph.insert((graph_nodes[i], graph_nodes[j]), dist);
                }
            }
        }
        graph
    }

    fn longest_hike_no_ice(&self) -> usize {
        let graph: HashMap<(Pos, Pos), usize> = self.build_graph();

        //Map every node to a different bit of an usize
        let mut nodes: Vec<Pos> = Vec::new();
        let mut nodes_set: HashSet<Pos> = HashSet::new();
        let mut small_graph: HashMap<usize, usize> = HashMap::new();

        graph.into_iter().for_each(|((l, r), v)| {
            let l_idx: usize = if nodes_set.insert(l) {
                let offset: usize = nodes.len();
                nodes.push(l);
                1 << offset
            } else {
                let offset: usize = nodes.iter().position(|&p| p == l).unwrap();
                1 << offset
            };
            let r_idx: usize = if nodes_set.insert(r) {
                let offset: usize = nodes.len();
                nodes.push(r);
                1 << offset
            } else {
                let offset: usize = nodes.iter().position(|&p| p == r).unwrap();
                1 << offset
            };
            small_graph.insert(l_idx | r_idx, v);
        });

        let start: usize = 1 << nodes.iter().position(|&p| p == self.start).unwrap();
        let end: usize = 1 << nodes.iter().position(|&p| p == self.end).unwrap();

        let start_node: NoIceNode = NoIceNode {
            pos: start,
            visited: start,
            steps: 0,
        };
        let mut current_nodes: Vec<NoIceNode> = vec![start_node];

        let mut max_steps: usize = 0;
        while let Some(node) = current_nodes.pop() {
            if node.pos == end && node.steps > max_steps {
                //println!("New max at {} {}", node.pos, node.steps);
                max_steps = node.steps;
            }

            node.neighbours(&small_graph)
                .into_iter()
                .for_each(|n| current_nodes.push(n));
        }

        max_steps
    }

    fn longest_hike(&self) -> usize {
        let start_node: Node = Node {
            pos: Pos(1, 1),
            from: self.start,
            steps: 1,
        };
        let mut current_nodes: Vec<Node> = vec![start_node];

        let mut max_steps: usize = 0;
        while let Some(worst_node) = current_nodes.pop() {
            if worst_node.pos == self.end && worst_node.steps > max_steps {
                max_steps = worst_node.steps;
            }

            worst_node
                .neighbours(self.max_x, self.max_y)
                .into_iter()
                .filter(|n| self.can_go_from_to(n.from, n.pos))
                .for_each(|n| current_nodes.push(n));
        }

        max_steps
    }
}

impl FromStr for TrailMap {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect::<Vec<Tile>>())
            .collect();
        let max_x: usize = grid[0].len();
        let max_y: usize = grid.len();
        let start: Pos = Pos(1, 0);
        let end: Pos = Pos(max_x - 2, max_y - 1);
        Ok(TrailMap {
            start,
            end,
            grid,
            max_x,
            max_y,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_23.txt").expect("Cannot open input file");
    let map: TrailMap = s.parse().unwrap();
    println!(
        "Part1: The longest hike you can take is {} steps long",
        map.longest_hike()
    );
    println!("Part2: After realizing you can climb the steep slopes, the longest hike is now {} steps long", map.longest_hike_no_ice());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "#.#####################
#.......#########...###
#######.#########.#.###
###.....#.>.>.###.#.###
###v#####.#v#.###.#.###
###.>...#.#.#.....#...#
###v###.#.#.#########.#
###...#.#.#.......#...#
#####.#.#.#######.#.###
#.....#.#.#.......#...#
#.#####.#.#.#########v#
#.#...#...#...###...>.#
#.#.#v#######v###.###v#
#...#.>.#...>.>.#.###.#
#####v#.#.###v#.#.###.#
#.....#...#...#.#.#...#
#.#########.###.#.#.###
#...###...#...#...#.###
###.###.#.###v#####v###
#...#...#.#.>.>.#.>.###
#.###.###.#.###.#.#v###
#.....###...###...#...#
#####################.#
";

    #[test]
    fn part_1() {
        let map: TrailMap = EXAMPLE_1.parse().unwrap();
        assert_eq!(map.longest_hike(), 94);
    }
    #[test]
    fn part_2() {
        let map: TrailMap = EXAMPLE_1.parse().unwrap();
        assert_eq!(map.longest_hike_no_ice(), 154);
    }
}
