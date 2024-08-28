use fxhash::FxHashMap;
use nom::character::complete::{alpha1, char};
use nom::sequence::separated_pair;
use nom::IResult;
use std::str::FromStr;

#[derive(Debug, Clone)]
struct CavePath {
    path: Vec<u16>,
    double_small: bool,
}

impl CavePath {
    fn extend(&self, next_nodes: Vec<u16>, visit_twice: bool) -> (bool, Vec<CavePath>) {
        let mut reach_end: bool = false;
        let mut new_paths: Vec<CavePath> = Vec::new();

        for n in next_nodes {
            if n == 0 {
                reach_end = true;
            } else if n > 100 {
                //Big caves can be visited any number of time
                let mut new: Vec<u16> = self.path.clone();
                new.push(n);
                new_paths.push(CavePath {
                    path: new,
                    double_small: self.double_small,
                });
            } else {
                let contains: bool = self.path.contains(&n);

                //We can build this path only if it has not been visited yet
                //or if it's the first small cave visited twice
                if !contains || (visit_twice && !self.double_small) {
                    let mut new: Vec<u16> = self.path.clone();
                    new.push(n);
                    new_paths.push(CavePath {
                        path: new,
                        double_small: contains || self.double_small,
                    });
                }
            }
        }

        (reach_end, new_paths)
    }
}

#[derive(Debug, Clone)]
struct CaveSystem {
    tunnels: FxHashMap<u16, Vec<u16>>,
}

impl CaveSystem {
    fn visit_caves(&self, visit_twice: bool) -> u16 {
        let mut nb_paths: u16 = 0;
        let mut working_paths: Vec<CavePath> = vec![CavePath {
            path: vec![1],
            double_small: false,
        }];

        while let Some(p) = working_paths.pop() {
            let last: u16 = p.path.last().copied().unwrap();
            let next_nodes = self.tunnels.get(&last).unwrap().clone();
            let (reached_end, new_paths) = p.extend(next_nodes, visit_twice);
            if reached_end {
                nb_paths += 1;
            }
            working_paths.extend(new_paths);
        }

        nb_paths
    }
    fn small_caves_once(&self) -> u16 {
        self.visit_caves(false)
    }
    fn small_caves_twice(&self) -> u16 {
        self.visit_caves(true)
    }
}

impl FromStr for CaveSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_tunnel(s: &str) -> IResult<&str, (String, String)> {
            let (s, (first, last)) = separated_pair(alpha1, char('-'), alpha1)(s)?;
            Ok((s, (first.to_string(), last.to_string())))
        }
        let mut tunnels: FxHashMap<u16, Vec<u16>> = FxHashMap::default();
        let mut names: FxHashMap<String, u16> = FxHashMap::default();
        let mut counter: u16 = 2;

        s.lines().for_each(|l| {
            let (first_name, last_name): (String, String) = parse_tunnel(l).unwrap().1;
            let first: u16 = match first_name.as_str() {
                "start" => 1,
                "end" => 0,
                n if n.to_ascii_uppercase() == first_name => {
                    let e = names.entry(first_name).or_insert_with(|| {
                        counter += 1;
                        counter + 99
                    });
                    *e
                }
                _ => {
                    let e = names.entry(first_name).or_insert_with(|| {
                        counter += 1;
                        counter - 1
                    });
                    *e
                }
            };
            let last: u16 = match last_name.as_str() {
                "start" => 1,
                "end" => 0,
                n if n.to_ascii_uppercase() == last_name => {
                    let e = names.entry(last_name).or_insert_with(|| {
                        counter += 1;
                        counter + 99
                    });
                    *e
                }
                _ => {
                    let e = names.entry(last_name).or_insert_with(|| {
                        counter += 1;
                        counter - 1
                    });
                    *e
                }
            };
            if last != 1 && first != 0 {
                tunnels.entry(first).or_default().push(last)
            }
            if last != 0 && first != 1 {
                tunnels.entry(last).or_default().push(first)
            }
        });
        Ok(CaveSystem { tunnels })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_12.txt").expect("Cannot open input file");
    let system: CaveSystem = s.parse().unwrap();
    println!(
        "Part1: There are {} paths going while going through small caves only once",
        system.small_caves_once()
    );
    println!(
        "Part2: When allowing to go through small caves twice, {} paths exist",
        system.small_caves_twice()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "start-A
start-b
A-c
A-b
b-d
A-end
b-end
";
    const EXAMPLE_2: &str = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc
";
    const EXAMPLE_3: &str = "fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW
";

    #[test]
    fn part_1_test_1() {
        let system: CaveSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!(10, system.small_caves_once());
    }

    #[test]
    fn part_1_test_2() {
        let system: CaveSystem = EXAMPLE_2.parse().unwrap();
        assert_eq!(19, system.small_caves_once());
    }

    #[test]
    fn part_1_test_3() {
        let system: CaveSystem = EXAMPLE_3.parse().unwrap();
        assert_eq!(226, system.small_caves_once());
    }

    #[test]
    fn part_2_test_1() {
        let system: CaveSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!(36, system.small_caves_twice());
    }

    #[test]
    fn part_2_test_2() {
        let system: CaveSystem = EXAMPLE_2.parse().unwrap();
        assert_eq!(103, system.small_caves_twice());
    }

    #[test]
    fn part_2_test_3() {
        let system: CaveSystem = EXAMPLE_3.parse().unwrap();
        assert_eq!(3509, system.small_caves_twice());
    }
}
