use itertools::Itertools;
use std::collections::{HashMap, HashSet};
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

struct KeyPath {
    keys: Vec<char>,
    dist: usize,
}

impl KeyPath {
    fn last_key(&self) -> char {
        *self.keys.last().unwrap()
    }

    fn reach_key(&self, target_key: char, dist: usize, doors: &[char]) -> Option<KeyPath> {
        if doors
            .iter()
            .all(|&door| self.keys.contains(&door.to_lowercase().last().unwrap()))
        {
            let mut new_keys = self.keys.clone();
            new_keys.push(target_key);
            Some(KeyPath {
                keys: new_keys,
                dist: self.dist + dist,
            })
        } else {
            None
        }
    }

    fn sort_path(&self) -> (char, Vec<char>) {
        let mut keys = self.keys.clone();
        let c = keys.pop().unwrap();
        keys.sort();
        (c, keys)
    }

    fn print(&self) {
        println!("{:?} in {} steps", self.keys, self.dist);
    }
}

struct QuadKeyPath {
    keys: Vec<char>,
    current: [char; 4],
    dist: usize,
}

impl QuadKeyPath {
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
            new_keys.push(target_key);
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

    fn sort_path(&self) -> (char, Vec<char>) {
        let mut keys = self.keys.clone();
        let c = keys.pop().unwrap();
        keys.sort();
        (c, keys)
    }

    fn print(&self) {
        println!("{:?} in {} steps", self.keys, self.dist);
    }
}

struct Tunnels {
    start: Pos,
    keys: HashMap<char, Pos>,
    grid: Vec<Vec<char>>,
}

impl Tunnels {
    fn quad_collect_all_keys(&self) -> QuadKeyPath {
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

        //Compute all possible paths depending on doors
        let key_set: HashSet<&char> = self.keys.keys().collect();
        let mut current_paths: Vec<QuadKeyPath> = vec![QuadKeyPath {
            keys: vec![],
            current: ['@', '@', '@', '@'],
            dist: 0,
        }];
        let mut n = 0;
        while current_paths.last().unwrap().keys.len() < self.keys.len() {
            n += 1;
            //Get next all next possible keys for all paths
            current_paths = current_paths
                .into_iter()
                .flat_map(|path| {
                    let p_keys: HashSet<&char> = path.keys.iter().collect();
                    let target_keys: Vec<char> = key_set.difference(&p_keys).map(|&&c| c).collect();

                    let currents: [char; 4] = path.current;
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
                                path.reach_key(i, b, *dist, doors)
                            } else {
                                None
                            }
                        })
                        .collect::<Vec<QuadKeyPath>>()
                })
                .collect();

            print!("Level {}, {} paths", n, current_paths.len());

            //Sort paths
            current_paths.sort_by(|a, b| {
                let (last_a, vec_a) = a.sort_path();
                let (last_b, vec_b) = b.sort_path();
                last_a
                    .cmp(&last_b)
                    .then(vec_a.cmp(&vec_b))
                    .then(a.current.cmp(&b.current))
                    .then(a.dist.cmp(&b.dist))
            });
            //Filter redondant paths (a->b->c ~= b->a->c)
            current_paths.dedup_by(|b, a| {
                let (last_a, vec_a) = a.sort_path();
                let (last_b, vec_b) = b.sort_path();
                last_a == last_b && vec_a.eq(&vec_b) && a.current.eq(&b.current)
            });

            println!(", deduplicated to {} paths", current_paths.len());
        }

        //Get cheapest path
        current_paths
            .into_iter()
            .min_by_key(|path| path.dist)
            .unwrap()
    }

    fn collect_all_keys(&self) -> KeyPath {
        //Get the distance matrix between every keys
        let dist_matrix: DistMatrix = self.distance_matrix();

        //Compute possible start paths
        let mut current_paths: Vec<KeyPath> = self.start_keys();
        let key_set: HashSet<&char> = self.keys.keys().collect();

        //Compute all possible paths depending on doors
        let mut n = 1;
        println!("Starting with {} paths", current_paths.len());
        while current_paths.last().unwrap().keys.len() < self.keys.len() {
            n += 1;
            //Get next all next possible keys for all paths
            current_paths = current_paths
                .into_iter()
                .flat_map(|path| {
                    let p_keys: HashSet<&char> = path.keys.iter().collect();
                    let target_keys: Vec<char> = key_set.difference(&p_keys).map(|&&c| c).collect();

                    let a: char = path.last_key();
                    target_keys
                        .into_iter()
                        .filter_map(|b| {
                            let (dist, doors): &(usize, Vec<char>) = if a < b {
                                dist_matrix.get(&(a, b)).unwrap()
                            } else {
                                dist_matrix.get(&(b, a)).unwrap()
                            };
                            path.reach_key(b, *dist, doors)
                        })
                        .collect::<Vec<KeyPath>>()
                })
                .collect();

            print!("Level {}, {} paths", n, current_paths.len());

            //Sort paths
            current_paths.sort_by(|a, b| {
                let (last_a, vec_a) = a.sort_path();
                let (last_b, vec_b) = b.sort_path();
                last_a
                    .cmp(&last_b)
                    .then(vec_a.cmp(&vec_b))
                    .then(a.dist.cmp(&b.dist))
            });
            //Filter redondant paths (a->b->c ~= b->a->c)
            current_paths.dedup_by(|b, a| {
                let (last_a, vec_a) = a.sort_path();
                let (last_b, vec_b) = b.sort_path();
                last_a == last_b && vec_a.eq(&vec_b)
            });

            println!(", deduplicated to {} paths", current_paths.len());
        }

        //Get cheapest path
        current_paths
            .into_iter()
            .min_by_key(|path| path.dist)
            .unwrap()
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
        self.keys
            .iter()
            .cartesian_product(self.keys.iter())
            .filter_map(|((&a, &pos_a), (&b, &pos_b))| {
                if a < b {
                    Some(((a, b), self.a_star(pos_a, pos_b).unwrap()))
                } else {
                    None
                }
            })
            .collect()
    }

    fn start_keys(&self) -> Vec<KeyPath> {
        self.keys
            .iter()
            .filter_map(|(&c, &pos)| {
                let (dist, doors) = self.a_star(self.start, pos).unwrap();
                if doors.is_empty() {
                    Some(KeyPath {
                        keys: vec![c],
                        dist,
                    })
                } else {
                    None
                }
            })
            .collect()
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
    let path = tunnels.collect_all_keys();
    print!("Part 1: ");
    path.print();

    //Part 2
    tunnels.split_start();
    let path = tunnels.quad_collect_all_keys();
    print!("Part 2: ");
    path.print();
    println!("Computing time: {:?}", now.elapsed());
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
        let path = tunnels.collect_all_keys();
        println!("{:?}", path.keys);
        assert_eq!(path.dist, 8);
    }

    #[test]
    fn example_2() {
        let tunnels: Tunnels = EXAMPLE_2.parse().unwrap();
        tunnels.print();
        let path = tunnels.collect_all_keys();
        println!("{:?}", path.keys);
        assert_eq!(path.dist, 86);
    }

    #[test]
    fn example_3() {
        let tunnels: Tunnels = EXAMPLE_3.parse().unwrap();
        tunnels.print();
        let path = tunnels.collect_all_keys();
        println!("{:?}", path.keys);
        assert_eq!(path.dist, 132);
    }

    #[test]
    fn example_4() {
        let tunnels: Tunnels = EXAMPLE_4.parse().unwrap();
        tunnels.print();
        let path = tunnels.collect_all_keys();
        println!("{:?}", path.keys);
        assert_eq!(path.dist, 136);
    }

    #[test]
    fn example_5() {
        let tunnels: Tunnels = EXAMPLE_5.parse().unwrap();
        tunnels.print();
        let path = tunnels.collect_all_keys();
        println!("{:?}", path.keys);
        assert_eq!(path.dist, 81);
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
        let path = tunnels.quad_collect_all_keys();
        assert_eq!(path.dist, 8);
    }
    #[test]
    fn example_7() {
        let tunnels: Tunnels = EXAMPLE_7.parse().unwrap();
        tunnels.print();
        let path = tunnels.quad_collect_all_keys();
        assert_eq!(path.dist, 24);
    }
    #[test]
    fn example_8() {
        let mut tunnels: Tunnels = EXAMPLE_8.parse().unwrap();
        tunnels.print();
        let path = tunnels.quad_collect_all_keys();
        assert_eq!(path.dist, 32);
    }
    #[test]
    fn example_9() {
        let mut tunnels: Tunnels = EXAMPLE_9.parse().unwrap();
        tunnels.print();
        let path = tunnels.quad_collect_all_keys();
        path.print();
        assert_eq!(path.dist, 72);
    }
}
