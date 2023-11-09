use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::{BTreeSet, BinaryHeap, HashMap, HashSet};
use std::str::FromStr;
use util::coord::Pos;

type DistMatrix = HashMap<(char, char), (usize, Vec<char>)>;

struct AStarNode {
    pos: Pos,
    dist: usize,
    doors: Vec<char>,
}

impl AStarNode {
    fn score(&self, end: Pos) -> usize {
        self.dist + self.pos.distance(end)
    }
}

#[derive(Clone, Eq, PartialEq)]
struct KeyPath {
    keys: BTreeSet<char>,
    last: char,
    dist: usize,
}

impl KeyPath {
    fn new() -> KeyPath {
        KeyPath {
            keys: BTreeSet::new(),
            last: '@',
            dist: 0,
        }
    }

    fn reach_all_keys(&self, key_set: &BTreeSet<char>, matrix: &DistMatrix) -> Vec<KeyPath> {
        let target_keys: Vec<char> = key_set.difference(&self.keys).copied().collect();

        let a: char = self.last;
        target_keys
            .into_iter()
            .filter_map(|b| {
                let (dist, doors): &(usize, Vec<char>) = if a < b {
                    matrix.get(&(a, b)).unwrap()
                } else {
                    matrix.get(&(b, a)).unwrap()
                };
                self.reach_key(b, *dist, doors)
            })
            .collect::<Vec<KeyPath>>()
    }

    fn reach_key(&self, target_key: char, dist: usize, doors: &[char]) -> Option<KeyPath> {
        if doors
            .iter()
            .all(|&door| self.keys.contains(&door.to_lowercase().last().unwrap()))
        {
            let mut new_keys = self.keys.clone();
            new_keys.insert(target_key);
            Some(KeyPath {
                keys: new_keys,
                last: target_key,
                dist: self.dist + dist,
            })
        } else {
            None
        }
    }

    fn hash(&self) -> (usize, char) {
        let path_score: usize = self
            .keys
            .iter()
            .map(|&c| {
                let i: usize = c as usize - 'a' as usize + 1;
                1 << i
            })
            .sum();
        (path_score, self.last)
    }
}

//We reverse order (Binary Heap pops the greatest item)
impl Ord for KeyPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(other.keys.len().cmp(&self.keys.len()))
    }
}

impl PartialOrd for KeyPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone, Eq, PartialEq)]
struct QuadKeyPath {
    keys: BTreeSet<char>,
    current: [char; 4],
    dist: usize,
}

impl QuadKeyPath {
    fn new() -> QuadKeyPath {
        QuadKeyPath {
            keys: BTreeSet::new(),
            current: ['@', '@', '@', '@'],
            dist: 0,
        }
    }

    fn reach_all_keys(
        &self,
        key_set: &BTreeSet<char>,
        matrixes: &[DistMatrix],
    ) -> Vec<QuadKeyPath> {
        let target_keys: Vec<char> = key_set.difference(&self.keys).copied().collect();

        let currents: [char; 4] = self.current;
        target_keys
            .into_iter()
            .flat_map(|b| vec![(b, 0), (b, 1), (b, 2), (b, 3)])
            .filter_map(|(b, i)| {
                let a: char = currents[i];
                let res: Option<&(usize, Vec<char>)> = if a < b {
                    matrixes[i].get(&(a, b))
                } else {
                    matrixes[i].get(&(b, a))
                };
                if let Some((dist, doors)) = res {
                    self.reach_key(i, b, *dist, doors)
                } else {
                    None
                }
            })
            .collect::<Vec<QuadKeyPath>>()
    }

    fn reach_key(
        &self,
        quarter: usize,
        target_key: char,
        dist: usize,
        doors: &[char],
    ) -> Option<QuadKeyPath> {
        if doors
            .iter()
            .all(|&door| self.keys.contains(&door.to_lowercase().last().unwrap()))
        {
            let mut new_keys = self.keys.clone();
            new_keys.insert(target_key);
            let mut new_current: [char; 4] = self.current;
            new_current[quarter] = target_key;
            Some(QuadKeyPath {
                keys: new_keys,
                current: new_current,
                dist: self.dist + dist,
            })
        } else {
            None
        }
    }

    fn hash(&self) -> (usize, usize) {
        let path_score: usize = self
            .keys
            .iter()
            .map(|&c| {
                let i: usize = c as usize - 'a' as usize + 1;
                1 << i
            })
            .sum();
        let current_score: usize = self
            .current
            .iter()
            .enumerate()
            .map(|(i, &c)| {
                let char_value = match c {
                    '@' => 1,
                    _ => c as usize - 'a' as usize + 2,
                };
                let pow: usize = 100 ^ i;
                char_value * pow
            })
            .sum();
        (path_score, current_score)
    }
}

//We reverse order (Binary Heap pops the greatest item)
impl Ord for QuadKeyPath {
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .dist
            .cmp(&self.dist)
            .then(other.keys.len().cmp(&self.keys.len()))
    }
}

impl PartialOrd for QuadKeyPath {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

struct Tunnels {
    start: Pos,
    keys: HashMap<char, Pos>,
    grid: Vec<Vec<char>>,
}

impl Tunnels {
    fn quad_collect_all_keys(&self) -> Option<usize> {
        //Compute the new starting points
        let Pos(x, y) = Pos(self.grid[0].len() / 2, self.grid.len() / 2);

        let nw: Pos = Pos(x - 1, y - 1);
        let ne: Pos = Pos(x + 1, y - 1);
        let sw: Pos = Pos(x - 1, y + 1);
        let se: Pos = Pos(x + 1, y + 1);

        //Get the distance matrix for each quarter tunnel
        let dist_nw: DistMatrix = self.distance_matrix_quarter(nw);
        let dist_ne: DistMatrix = self.distance_matrix_quarter(ne);
        let dist_sw: DistMatrix = self.distance_matrix_quarter(sw);
        let dist_se: DistMatrix = self.distance_matrix_quarter(se);

        let matrixes: [DistMatrix; 4] = [dist_nw, dist_ne, dist_sw, dist_se];

        //Start each robot from the '@' char
        let start_path: QuadKeyPath = QuadKeyPath::new();
        let mut best_dists: HashMap<(usize, usize), usize> =
            HashMap::from([(start_path.hash(), start_path.dist)]);
        let mut heap: BinaryHeap<QuadKeyPath> = BinaryHeap::new();
        heap.push(start_path);
        let key_set: BTreeSet<char> = self.keys.keys().copied().collect();

        //Compute all possible paths depending on doors
        while let Some(best_path) = heap.pop() {
            //We got a path picking all keys, we know it's the best path
            if best_path.keys.len() == self.keys.len() {
                return Some(best_path.dist);
            }

            //This node has already be attained by a better path, skip it
            if let Some(&dist) = best_dists.get(&best_path.hash()) {
                if best_path.dist > dist {
                    continue;
                }
            }

            //Get next all next possible keys for this path
            best_path
                .reach_all_keys(&key_set, &matrixes)
                .into_iter()
                .for_each(|new_path| {
                    //Get the best current distance for this node
                    let distance_entry = best_dists.entry(new_path.hash()).or_insert(usize::MAX);

                    //If the new node has a best distance, we update the distance and add the Path to the heap
                    if new_path.dist < *distance_entry {
                        *distance_entry = new_path.dist;

                        heap.push(new_path);
                    }
                })
        }
        //No path found
        None
    }

    fn collect_all_keys(&self) -> Option<usize> {
        //Get the distance matrix between every keys
        let dist_matrix: DistMatrix = self.distance_matrix();

        //Start from the '@' char
        let start_path: KeyPath = KeyPath::new();
        let mut best_dists: HashMap<(usize, char), usize> =
            HashMap::from([(start_path.hash(), start_path.dist)]);
        let mut heap: BinaryHeap<KeyPath> = BinaryHeap::new();
        heap.push(start_path);
        let key_set: BTreeSet<char> = self.keys.keys().copied().collect();

        //Compute all possible paths depending on doors
        while let Some(best_path) = heap.pop() {
            //We got a path picking all keys, we know it's the best path
            if best_path.keys.len() == self.keys.len() {
                return Some(best_path.dist);
            }

            //This node has already be attained by a better path, skip it
            if let Some(&dist) = best_dists.get(&best_path.hash()) {
                if best_path.dist > dist {
                    continue;
                }
            }

            //Get next all next possible keys for this path
            best_path
                .reach_all_keys(&key_set, &dist_matrix)
                .into_iter()
                .for_each(|new_path| {
                    //Get the best current distance for this node
                    let distance_entry = best_dists.entry(new_path.hash()).or_insert(usize::MAX);

                    //If the new node has a best distance, we update the distance and add the Path to the heap
                    if new_path.dist < *distance_entry {
                        *distance_entry = new_path.dist;

                        heap.push(new_path);
                    }
                })
        }
        //No path found
        None
    }

    fn distance_matrix_quarter(&self, start: Pos) -> DistMatrix {
        let mut dist_matrix: DistMatrix = self
            .keys
            .iter()
            .filter_map(|(&a, &pos_a)| self.a_star(start, pos_a).map(|pair| (('@', a), pair)))
            .collect();

        let keys: Vec<char> = dist_matrix.keys().map(|&(_, k)| k).collect();

        let keys_map: HashMap<char, Pos> = self
            .keys
            .clone()
            .into_iter()
            .filter(|(k, _)| keys.contains(k))
            .collect();

        let rest_matrix: DistMatrix = keys_map
            .iter()
            .cartesian_product(keys_map.iter())
            .filter_map(|((&a, &pos_a), (&b, &pos_b))| {
                if a < b {
                    Some(((a, b), self.a_star(pos_a, pos_b).unwrap()))
                } else {
                    None
                }
            })
            .collect();

        dist_matrix.extend(rest_matrix);

        dist_matrix
    }

    fn distance_matrix(&self) -> DistMatrix {
        let mut dist_matrix: DistMatrix = self
            .keys
            .iter()
            .filter_map(|(&a, &pos)| self.a_star(self.start, pos).map(|pair| (('@', a), pair)))
            .collect();
        let rest_matrix: DistMatrix = self
            .keys
            .iter()
            .cartesian_product(self.keys.iter())
            .filter_map(|((&a, &pos_a), (&b, &pos_b))| {
                if a < b {
                    Some(((a, b), self.a_star(pos_a, pos_b).unwrap()))
                } else {
                    None
                }
            })
            .collect();

        dist_matrix.extend(rest_matrix);
        dist_matrix
    }

    fn a_star(&self, start: Pos, end: Pos) -> Option<(usize, Vec<char>)> {
        let start_node: AStarNode = AStarNode {
            pos: start,
            dist: 0,
            doors: Vec::new(),
        };
        let mut nodes: Vec<AStarNode> = vec![start_node];
        let mut visited_pos: HashSet<Pos> = HashSet::from([start]);
        loop {
            if let Some(best_node) = nodes.pop() {
                if best_node.pos == end {
                    return Some((best_node.dist, best_node.doors));
                }
                //Build the new nodes from the current neighbours
                let new_nodes: Vec<AStarNode> = best_node
                    .pos
                    .neighbours()
                    .into_iter()
                    .filter_map(|p @ Pos(x, y)| {
                        let c: char = self.grid[y][x];
                        match c {
                            '#' => None,
                            'A'..='Z' => {
                                let mut doors: Vec<char> = best_node.doors.clone();
                                doors.push(c);
                                Some(AStarNode {
                                    pos: p,
                                    dist: best_node.dist + 1,
                                    doors,
                                })
                            }
                            _ => Some(AStarNode {
                                pos: p,
                                dist: best_node.dist + 1,
                                doors: best_node.doors.clone(),
                            }),
                        }
                    })
                    .collect();

                new_nodes
                    .into_iter()
                    //Filter already visited positions
                    .filter(|node| visited_pos.insert(node.pos))
                    .for_each(|node| {
                        let i: usize = nodes.partition_point(|x| x.score(end) > node.score(end));
                        nodes.insert(i, node);
                    })
            } else {
                //The node cannot reach the end
                return None;
            }
        }
    }

    fn split_start(&mut self) {
        //Replace all neighbour by an entry
        for Pos(x, y) in self.start.neighbours_diag() {
            self.grid[y][x] = '@';
        }

        //Rewrite the cross neighbour as a wall
        for Pos(x, y) in self.start.neighbours() {
            self.grid[y][x] = '#';
        }

        let Pos(x, y) = self.start;
        self.grid[y][x] = '#';
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter() {
            for &pix in row {
                let c = match pix {
                    '#' => '█',
                    '.' => ' ',
                    _ => pix,
                };
                print!("{c}");
            }
            println!();
        }
    }
}

impl FromStr for Tunnels {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(|line| line.chars().collect::<Vec<char>>())
            .collect();

        let letters: HashMap<char, Pos> = grid
            .iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(x, &c)| {
                        if c != '#' && c != '.' {
                            Some((c, Pos(x, y)))
                        } else {
                            None
                        }
                    })
                    .collect::<HashMap<char, Pos>>()
            })
            .collect();

        let start: Pos = *letters.get(&'@').unwrap();
        let keys: HashMap<char, Pos> = letters
            .into_iter()
            .filter(|(k, _)| k.is_ascii_lowercase())
            .collect();

        Ok(Tunnels { start, keys, grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_18.txt").expect("Cannot open input file");
    let mut tunnels: Tunnels = s.parse().unwrap();

    //tunnels.print();
    let best_dist = tunnels.collect_all_keys().unwrap();
    println!(
        "Part 1: The shortest path to collect all keys takes {} steps",
        best_dist
    );
    println!("Computing time part 1: {:?}", now.elapsed());

    //Part 2
    let now = std::time::Instant::now();
    tunnels.split_start();
    let best_dist = tunnels.quad_collect_all_keys().unwrap();
    println!(
        "Part 2: The shortest path to collect all keys takes {} steps when using 4 robots",
        best_dist
    );
    println!("Computing time part 2: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "#########
#b.A.@.a#
#########";

    const EXAMPLE_2: &str = "########################
#f.D.E.e.C.b.A.@.a.B.c.#
######################.#
#d.....................#
########################";

    const EXAMPLE_3: &str = "########################
#...............b.C.D.f#
#.######################
#.....@.a.B.c.d.A.e.F.g#
########################";

    const EXAMPLE_4: &str = "#################
#i.G..c...e..H.p#
########.########
#j.A..b...f..D.o#
########@########
#k.E..a...g..B.n#
########.########
#l.F..d...h..C.m#
#################";

    const EXAMPLE_5: &str = "########################
#@..............ac.GI.b#
###d#e#f################
###A#B#C################
###g#h#i################
########################";

    #[test]
    fn example_1() {
        let tunnels: Tunnels = EXAMPLE_1.parse().unwrap();
        tunnels.print();
        let dist = tunnels.collect_all_keys();
        assert_eq!(dist, Some(8));
    }

    #[test]
    fn example_2() {
        let tunnels: Tunnels = EXAMPLE_2.parse().unwrap();
        tunnels.print();
        let dist = tunnels.collect_all_keys();
        assert_eq!(dist, Some(86));
    }

    #[test]
    fn example_3() {
        let tunnels: Tunnels = EXAMPLE_3.parse().unwrap();
        tunnels.print();
        let dist = tunnels.collect_all_keys();
        assert_eq!(dist, Some(132));
    }

    #[test]
    fn example_4() {
        let tunnels: Tunnels = EXAMPLE_4.parse().unwrap();
        tunnels.print();
        let dist = tunnels.collect_all_keys();
        assert_eq!(dist, Some(136));
    }

    #[test]
    fn example_5() {
        let tunnels: Tunnels = EXAMPLE_5.parse().unwrap();
        tunnels.print();
        let dist = tunnels.collect_all_keys();
        assert_eq!(dist, Some(81));
    }

    const EXAMPLE_6: &str = "#######
#a.#Cd#
##...##
##.@.##
##...##
#cB#.b#
#######";

    const EXAMPLE_7: &str = "###############
#d.ABC.#.....a#
######@#@######
###############
######@#@######
#b.....#.....c#
###############";

    const EXAMPLE_8: &str = "#############
#DcBa.#.GhKl#
#.###@#@#I###
#e#d#####j#k#
###C#@#@###J#
#fEbA.#.FgHi#
#############";

    const EXAMPLE_9: &str = "#############
#g#f.D#..h#l#
#F###e#E###.#
#dCba@#@BcIJ#
#############
#nK.L@#@G...#
#M###N#H###.#
#o#m..#i#jk.#
#############";

    #[test]
    fn example_6() {
        let mut tunnels: Tunnels = EXAMPLE_6.parse().unwrap();
        tunnels.split_start();
        tunnels.print();
        let dist = tunnels.quad_collect_all_keys();
        assert_eq!(dist, Some(8));
    }
    #[test]
    fn example_7() {
        let tunnels: Tunnels = EXAMPLE_7.parse().unwrap();
        tunnels.print();
        let dist = tunnels.quad_collect_all_keys();
        assert_eq!(dist, Some(24));
    }
    #[test]
    fn example_8() {
        let tunnels: Tunnels = EXAMPLE_8.parse().unwrap();
        tunnels.print();
        let dist = tunnels.quad_collect_all_keys();
        assert_eq!(dist, Some(32));
    }
    #[test]
    fn example_9() {
        let tunnels: Tunnels = EXAMPLE_9.parse().unwrap();
        tunnels.print();
        let dist = tunnels.quad_collect_all_keys();
        assert_eq!(dist, Some(72));
    }
}
