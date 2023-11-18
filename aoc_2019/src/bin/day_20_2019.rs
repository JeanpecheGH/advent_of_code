use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use util::coord::Pos;

type Portal = (char, char);
const END: Portal = ('Z', 'Z');
const START: Portal = ('A', 'A');

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Wall,
    Path,
    Inner(Portal),
    Outer(Portal),
}

struct BfsNode {
    pos: Pos,
    steps: usize,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct DijkstraNode {
    tile: Tile,
    steps: usize,
}

impl Ord for DijkstraNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other.steps.cmp(&self.steps)
    }
}

impl PartialOrd for DijkstraNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
struct RecursiveNode {
    tile: Tile,
    level: usize,
    steps: usize,
}

impl Ord for RecursiveNode {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .steps
            .cmp(&self.steps)
            .then(other.level.cmp(&self.level))
    }
}

impl PartialOrd for RecursiveNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct DonutMaze {
    grid: Vec<Vec<Tile>>,
}

impl DonutMaze {
    fn aa_to_zz(&self) -> Option<usize> {
        let matrix = self.distance_matrix();

        //Start from the "AA" portal
        let start_path: DijkstraNode = DijkstraNode {
            tile: Tile::Outer(START),
            steps: 0,
        };
        let mut best_dists: HashMap<Tile, usize> =
            HashMap::from([(start_path.tile, start_path.steps)]);
        let mut heap: BinaryHeap<DijkstraNode> = BinaryHeap::new();
        heap.push(start_path);

        //Compute all possible next nodes
        while let Some(best_path) = heap.pop() {
            //We arrived to "ZZ", we know it's the best path by definition
            if let Tile::Outer(END) = best_path.tile {
                return Some(best_path.steps);
            }

            //This node has already be attained by a better path, skip it
            if let Some(&steps) = best_dists.get(&best_path.tile) {
                if best_path.steps > steps {
                    continue;
                }
            }

            //Get next all next possible nodes from this tile
            matrix[&best_path.tile]
                .iter()
                .for_each(|(dest, add_steps)| {
                    //We automatically skip to the other side of the portals, except for "ZZ"
                    let partial_steps: usize = best_path.steps + add_steps;
                    let opt_pair: Option<(Tile, usize)> = match *dest {
                        Tile::Inner(p) => Some((Tile::Outer(p), partial_steps + 1)),
                        Tile::Outer(p) if p != END => Some((Tile::Inner(p), partial_steps + 1)),
                        Tile::Outer(p) if p == END => Some((*dest, partial_steps)),
                        _ => None,
                    };

                    if let Some((tile, new_steps)) = opt_pair {
                        //Get the best current distance for this node
                        let distance_entry = best_dists.entry(tile).or_insert(usize::MAX);

                        //If the new node has a best distance, we update the distance and add the node to the heap
                        if new_steps < *distance_entry {
                            *distance_entry = new_steps;
                            heap.push(DijkstraNode {
                                tile,
                                steps: new_steps,
                            });
                        }
                    }
                })
        }
        //No path found
        None
    }

    fn recursive_aa_to_zz(&self) -> Option<usize> {
        let matrix = self.distance_matrix();

        //Start from the "AA" portal
        let start_path: RecursiveNode = RecursiveNode {
            tile: Tile::Outer(START),
            level: 0,
            steps: 0,
        };
        let mut best_dists: HashMap<(Tile, usize), usize> =
            HashMap::from([((start_path.tile, start_path.level), start_path.steps)]);
        let mut heap: BinaryHeap<RecursiveNode> = BinaryHeap::new();
        heap.push(start_path);

        //Compute all possible next nodes
        while let Some(best_path) = heap.pop() {
            //We arrived to "ZZ", we know it's the best path by definition
            if let Tile::Outer(END) = best_path.tile {
                return Some(best_path.steps);
            }

            //This node has already be attained by a better path, skip it
            if let Some(&steps) = best_dists.get(&(best_path.tile, best_path.level)) {
                if best_path.steps > steps {
                    continue;
                }
            }

            //Get next all next possible nodes from this tile
            matrix[&best_path.tile]
                .iter()
                .for_each(|(dest, add_steps)| {
                    let partial_steps = best_path.steps + add_steps;
                    //We automatically skip to the other side of the portals, except for "ZZ"
                    let opt_trio: Option<(usize, Tile, usize)> = match *dest {
                        Tile::Inner(p) => {
                            Some((best_path.level + 1, Tile::Outer(p), partial_steps + 1))
                        }
                        //Outer portals are walls at level 0
                        Tile::Outer(p) if p != END && best_path.level > 0 => {
                            Some((best_path.level - 1, Tile::Inner(p), partial_steps + 1))
                        }
                        //The "ZZ" portal is a wall at level > 0
                        Tile::Outer(p) if p == END && best_path.level == 0 => {
                            Some((0, *dest, partial_steps))
                        }
                        _ => None,
                    };
                    if let Some((level, tile, new_steps)) = opt_trio {
                        //Get the best current distance for this node
                        let distance_entry = best_dists.entry((tile, level)).or_insert(usize::MAX);

                        //If the new node has a best distance, we update the distance and add the node to the heap
                        if new_steps < *distance_entry {
                            *distance_entry = new_steps;

                            heap.push(RecursiveNode {
                                tile,
                                level,
                                steps: new_steps,
                            });
                        }
                    }
                })
        }
        //No path found
        None
    }

    fn distance_matrix(&self) -> HashMap<Tile, Vec<(Tile, usize)>> {
        let height: usize = self.grid.len();
        let width: usize = self.grid[0].len();

        let mut matrix: HashMap<Tile, Vec<(Tile, usize)>> = HashMap::new();

        for y in 0..height {
            for x in 0..width {
                match self.grid[y][x] {
                    inner @ Tile::Inner(_) => {
                        matrix.insert(inner, self.bfs(Pos(x, y)));
                    }
                    outer @ Tile::Outer(p) if p != END => {
                        matrix.insert(outer, self.bfs(Pos(x, y)));
                    }
                    _ => (),
                }
            }
        }
        matrix
    }

    //Returns the list of all reachable Portals from a given position and the number of steps to do so
    fn bfs(&self, start: Pos) -> Vec<(Tile, usize)> {
        let start_node: BfsNode = BfsNode {
            pos: start,
            steps: 0,
        };

        let mut nodes: Vec<BfsNode> = vec![start_node];
        let mut visited_pos: HashSet<Pos> = HashSet::from([start]);
        let mut reachable_portals: Vec<(Tile, usize)> = Vec::new();

        while let Some(curr_node) = nodes.pop() {
            //If we are on a Portal, we add this portal and the number of steps
            if curr_node.pos != start {
                match self.grid[curr_node.pos.1][curr_node.pos.0] {
                    inner @ Tile::Inner(_) => {
                        reachable_portals.push((inner, curr_node.steps));
                        continue;
                    }
                    //Never try to return to the "AA" start
                    outer @ Tile::Outer(p) if p != START => {
                        reachable_portals.push((outer, curr_node.steps));
                        continue;
                    }
                    _ => (),
                }
            }
            //Build the new nodes from the current neighbours
            let new_nodes: Vec<BfsNode> = curr_node
                .pos
                .neighbours()
                .into_iter()
                .filter_map(|p @ Pos(x, y)| {
                    if let Tile::Wall = self.grid[y][x] {
                        None
                    } else if visited_pos.insert(p) {
                        Some(BfsNode {
                            pos: p,
                            steps: curr_node.steps + 1,
                        })
                    } else {
                        None
                    }
                })
                .collect();

            nodes.extend(new_nodes);
        }
        reachable_portals
    }
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter() {
            for tile in row.iter() {
                let pair: Portal = match *tile {
                    Tile::Wall => ('█', '█'),
                    Tile::Path => (' ', ' '),
                    Tile::Inner(p) => p,
                    Tile::Outer(p) => p,
                };
                print!("{}{}", pair.0, pair.1);
            }
            println!();
        }
    }
}

impl FromStr for DonutMaze {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut char_grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let max_row: usize = char_grid.iter().map(|row| row.len()).max().unwrap();

        char_grid.iter_mut().for_each(|row| {
            (0..max_row - row.len()).for_each(|_| row.push(' '));
        });

        let mut grid: Vec<Vec<Tile>> = char_grid
            .iter()
            .map(|row| {
                row.iter()
                    .map(|&c| match c {
                        '.' => Tile::Path,
                        '#' => Tile::Wall,
                        ' ' => Tile::Wall,
                        _ => Tile::Outer((c, '*')),
                    })
                    .collect::<Vec<Tile>>()
            })
            .collect();

        let width: usize = grid[0].len();
        let height: usize = grid.len();
        //Find and replace portals
        for y in 0..height {
            for x in 0..width {
                if let Tile::Outer((c_1, _)) = grid[y][x] {
                    //Horizontal pair
                    if let Some(Tile::Outer((c_2, _))) = grid[y].get(x + 1).copied() {
                        grid[y][x] = Tile::Wall;
                        grid[y][x + 1] = Tile::Wall;
                        let portal = if (3..width - 3).contains(&x) && (3..height - 3).contains(&y)
                        {
                            Tile::Inner((c_1, c_2))
                        } else {
                            Tile::Outer((c_1, c_2))
                        };
                        //Check right
                        if let Some(Tile::Path) = grid[y].get(x + 2) {
                            grid[y][x + 2] = portal;
                        } else {
                            //Else it's obviously left
                            grid[y][x - 1] = portal;
                        }
                    }
                    //Vertical pair
                    if let Some(Tile::Outer((c_2, _))) = grid.get(y + 1).map(|row| row[x]) {
                        grid[y][x] = Tile::Wall;
                        grid[y + 1][x] = Tile::Wall;
                        let portal = if (3..width - 3).contains(&x) && (3..height - 3).contains(&y)
                        {
                            Tile::Inner((c_1, c_2))
                        } else {
                            Tile::Outer((c_1, c_2))
                        };
                        //Check down
                        if let Some(Tile::Path) = grid.get(y + 2).map(|row| row[x]) {
                            grid[y + 2][x] = portal;
                        } else {
                            //Else it's obviously up
                            grid[y - 1][x] = portal;
                        }
                    }
                }
            }
        }

        Ok(DonutMaze { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_20.txt").expect("Cannot open input file");
    let maze: DonutMaze = s.parse().unwrap();

    println!(
        "Part1: It takes {} steps to go from AA to ZZ",
        maze.aa_to_zz().unwrap()
    );
    println!(
        "Part2: It takes {} steps to go from AA to ZZ in a recursive maze",
        maze.recursive_aa_to_zz().unwrap()
    );

    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "         A           
         A           
  #######.#########  
  #######.........#  
  #######.#######.#  
  #######.#######.#  
  #######.#######.#  
  #####  B    ###.#  
BC...##  C    ###.#  
  ##.##       ###.#  
  ##...DE  F  ###.#  
  #####    G  ###.#  
  #########.#####.#  
DE..#######...###.#  
  #.#########.###.#  
FG..#########.....#  
  ###########.#####  
             Z       
             Z       ";

    const EXAMPLE_2: &str = "                   A               
                   A               
  #################.#############  
  #.#...#...................#.#.#  
  #.#.#.###.###.###.#########.#.#  
  #.#.#.......#...#.....#.#.#...#  
  #.#########.###.#####.#.#.###.#  
  #.............#.#.....#.......#  
  ###.###########.###.#####.#.#.#  
  #.....#        A   C    #.#.#.#  
  #######        S   P    #####.#  
  #.#...#                 #......VT
  #.#.#.#                 #.#####  
  #...#.#               YN....#.#  
  #.###.#                 #####.#  
DI....#.#                 #.....#  
  #####.#                 #.###.#  
ZZ......#               QG....#..AS
  ###.###                 #######  
JO..#.#.#                 #.....#  
  #.#.#.#                 ###.#.#  
  #...#..DI             BU....#..LF
  #####.#                 #.#####  
YN......#               VT..#....QG
  #.###.#                 #.###.#  
  #.#...#                 #.....#  
  ###.###    J L     J    #.#.###  
  #.....#    O F     P    #.#...#  
  #.###.#####.#.#####.#####.###.#  
  #...#.#.#...#.....#.....#.#...#  
  #.#####.###.###.#.#.#########.#  
  #...#.#.....#...#.#.#.#.....#.#  
  #.###.#####.###.###.#.#.#######  
  #.#.........#...#.............#  
  #########.###.###.#############  
           B   J   C               
           U   P   P               ";

    const EXAMPLE_3: &str = "             Z L X W       C                 
             Z P Q B       K                 
  ###########.#.#.#.#######.###############  
  #...#.......#.#.......#.#.......#.#.#...#  
  ###.#.#.#.#.#.#.#.###.#.#.#######.#.#.###  
  #.#...#.#.#...#.#.#...#...#...#.#.......#  
  #.###.#######.###.###.#.###.###.#.#######  
  #...#.......#.#...#...#.............#...#  
  #.#########.#######.#.#######.#######.###  
  #...#.#    F       R I       Z    #.#.#.#  
  #.###.#    D       E C       H    #.#.#.#  
  #.#...#                           #...#.#  
  #.###.#                           #.###.#  
  #.#....OA                       WB..#.#..ZH
  #.###.#                           #.#.#.#  
CJ......#                           #.....#  
  #######                           #######  
  #.#....CK                         #......IC
  #.###.#                           #.###.#  
  #.....#                           #...#.#  
  ###.###                           #.#.#.#  
XF....#.#                         RF..#.#.#  
  #####.#                           #######  
  #......CJ                       NM..#...#  
  ###.#.#                           #.###.#  
RE....#.#                           #......RF
  ###.###        X   X       L      #.#.#.#  
  #.....#        F   Q       P      #.#.#.#  
  ###.###########.###.#######.#########.###  
  #.....#...#.....#.......#...#.....#.#...#  
  #####.#.###.#######.#######.###.###.#.#.#  
  #.......#.......#.#.#.#.#...#...#...#.#.#  
  #####.###.#####.#.#.#.#.###.###.#.###.###  
  #.......#.....#.#...#...............#...#  
  #############.#.#.###.###################  
               A O F   N                     
               A A D   M                     ";

    #[test]
    fn example_1() {
        let maze: DonutMaze = EXAMPLE_1.parse().unwrap();
        maze.print();
        assert_eq!(Some(23), maze.aa_to_zz());
    }

    #[test]
    fn example_2() {
        let maze: DonutMaze = EXAMPLE_2.parse().unwrap();
        maze.print();
        assert_eq!(Some(58), maze.aa_to_zz());
    }

    #[test]
    fn recursive_example_1() {
        let maze: DonutMaze = EXAMPLE_1.parse().unwrap();
        maze.print();
        assert_eq!(Some(26), maze.recursive_aa_to_zz());
    }

    #[test]
    fn recursive_example_3() {
        let maze: DonutMaze = EXAMPLE_3.parse().unwrap();
        maze.print();
        assert_eq!(Some(396), maze.recursive_aa_to_zz());
    }
}
