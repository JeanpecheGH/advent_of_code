use std::str::FromStr;

use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use nom::character::complete::anychar;
use nom::multi::count;
use nom::{character::complete::char, sequence::separated_pair, IResult};

struct LanParty {
    pairs: Vec<(usize, usize)>,
    connections: FxHashMap<usize, FxHashSet<usize>>,
}

impl LanParty {
    fn t_triplets(&self) -> usize {
        let triplets: FxHashSet<Vec<usize>> = self
            .pairs
            .iter()
            .flat_map(|(a, b)| {
                if (a / 26) == 19 || (b / 26) == 19 {
                    let a_candidates: &FxHashSet<usize> = self.connections.get(a).unwrap();
                    let b_candidates: &FxHashSet<usize> = self.connections.get(b).unwrap();
                    let inter = a_candidates.intersection(b_candidates);
                    inter
                        .map(|c| {
                            let mut v = vec![*a, *b, *c];
                            v.sort();
                            v
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            })
            .collect();
        triplets.len()
    }

    fn biggest_lan(&self) -> String {
        fn bron_kerbosch(
            clique: FxHashSet<usize>,
            mut pot: FxHashSet<usize>,
            mut ex: FxHashSet<usize>,
            connections: &FxHashMap<usize, FxHashSet<usize>>,
            biggest_lan: &mut FxHashSet<usize>,
        ) {
            if biggest_lan.len() > (clique.len() + pot.iter().len()) {
                //We already have a bigger clique than the potential biggest, prune this branch
                return;
            }
            if pot.is_empty() && ex.is_empty() && clique.len() > biggest_lan.len() {
                *biggest_lan = clique.clone();
            }
            // Choose a pivot with the maximum degree in P ∪ X
            let pivot: Option<usize> = pot
                .union(&ex)
                .max_by_key(|&v| connections.get(v).map_or(0, |neighbors| neighbors.len()))
                .copied();

            if let Some(pivot_vertex) = pivot {
                let neighbors: FxHashSet<usize> =
                    connections.get(&pivot_vertex).cloned().unwrap_or_default();
                let candidates: Vec<usize> = pot.difference(&neighbors).copied().collect();

                for v in candidates {
                    let mut new_clique: FxHashSet<usize> = clique.clone();
                    new_clique.insert(v);
                    let ngbs: &FxHashSet<usize> = connections.get(&v).unwrap();
                    let new_pot: FxHashSet<usize> = pot.intersection(ngbs).copied().collect();
                    let new_ex: FxHashSet<usize> = ex.intersection(ngbs).copied().collect();
                    bron_kerbosch(new_clique, new_pot, new_ex, connections, biggest_lan);
                    pot.remove(&v);
                    ex.insert(v);
                }
            }
        }
        let mut biggest_lan: FxHashSet<usize> = FxHashSet::default();
        let clique: FxHashSet<usize> = FxHashSet::default();
        let potential: FxHashSet<usize> = self.connections.keys().copied().collect();
        let excluded: FxHashSet<usize> = FxHashSet::default();
        bron_kerbosch(
            clique,
            potential,
            excluded,
            &self.connections,
            &mut biggest_lan,
        );

        biggest_lan
            .into_iter()
            .sorted()
            .map(|n| {
                format!(
                    "{}{}",
                    ((n / 26) as u8 + b'a') as char,
                    ((n % 26) as u8 + b'a') as char
                )
            })
            .join(",")
    }
}

impl FromStr for LanParty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (usize, usize)> {
            let (s, (a, b)) = separated_pair(count(anychar, 2), char('-'), count(anychar, 2))(s)?;
            Ok((s, (to_usize(&a), to_usize(&b))))
        }

        fn to_usize(chars: &[char]) -> usize {
            (chars[0] as usize - 'a' as usize) * 26 + chars[1] as usize - 'a' as usize
        }

        let mut connections: FxHashMap<usize, FxHashSet<usize>> = FxHashMap::default();
        let pairs: Vec<(usize, usize)> = s
            .lines()
            .map(|l| {
                let (a, b) = parse_pair(l).unwrap().1;
                connections.entry(a).or_default().insert(b);
                connections.entry(b).or_default().insert(a);
                (a, b)
            })
            .collect();

        Ok(LanParty { pairs, connections })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_23.txt").expect("Cannot open input file");
    let lan: LanParty = s.parse().unwrap();
    println!(
        "Part1: There are {} triplets of connected computer where at least one starts with a 't' ",
        lan.t_triplets()
    );
    println!(
        "Part2: The password to the LAN party is {}",
        lan.biggest_lan()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "kh-tc
qp-kh
de-cg
ka-co
yn-aq
qp-ub
cg-tb
vc-aq
tb-ka
wh-tc
yn-cg
kh-ub
ta-co
de-co
tc-td
tb-wq
wh-td
ta-ka
td-qp
aq-cg
wq-ub
ub-vc
de-ta
wq-aq
wq-vc
wh-yn
ka-de
kh-ta
co-tc
wh-qp
tb-vc
td-yn
";

    #[test]
    fn part_1() {
        let lan: LanParty = EXAMPLE_1.parse().unwrap();
        assert_eq!(lan.t_triplets(), 7);
    }

    #[test]
    fn part_2() {
        let lan: LanParty = EXAMPLE_1.parse().unwrap();
        assert_eq!(lan.biggest_lan(), "co,de,ka,ta");
    }
}
