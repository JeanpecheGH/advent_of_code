use std::str::FromStr;

use fxhash::{FxHashMap, FxHashSet};
use nom::{
    character::complete::alpha1, character::complete::char, sequence::separated_pair, IResult,
};

struct LanParty {
    pairs: FxHashSet<(String, String)>,
}
impl LanParty {
    fn are_linked(&self, a: &str) -> FxHashSet<String> {
        self.pairs
            .iter()
            .filter_map(|(x, y)| {
                if x == a {
                    Some(y)
                } else if y == a {
                    Some(x)
                } else {
                    None
                }
            })
            .cloned()
            .collect()
    }

    fn t_triplets(&self) -> usize {
        let triplets: FxHashSet<Vec<String>> = self
            .pairs
            .iter()
            .flat_map(|(a, b)| {
                if a.starts_with("t") || b.starts_with("t") {
                    let a_candidates: FxHashSet<String> = self.are_linked(a);
                    let b_candidates: FxHashSet<String> = self.are_linked(b);
                    let inter = a_candidates.intersection(&b_candidates);
                    inter
                        .map(|c| {
                            let mut v = vec![a.clone(), b.clone(), c.clone()];
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
        let mut connections: FxHashMap<String, FxHashSet<String>> = FxHashMap::default();
        for (a, b) in &self.pairs {
            connections.entry(a.clone()).or_default().insert(b.clone());
            connections.entry(b.clone()).or_default().insert(a.clone());
        }
        let mut lans: Vec<FxHashSet<String>> = Vec::new();
        for (a, b) in &self.pairs {
            let a_conn = connections.get(a).unwrap();
            let b_conn = connections.get(b).unwrap();
            for l in lans.iter_mut() {
                if l.iter()
                    .all(|x| (x == a || a_conn.contains(x)) && (x == b || b_conn.contains(x)))
                {
                    l.insert(a.clone());
                    l.insert(b.clone());
                }
            }
            let mut new_lan: FxHashSet<String> = FxHashSet::default();
            new_lan.insert(a.clone());
            new_lan.insert(b.clone());
            lans.push(new_lan);
        }

        // Find the biggest lan
        let biggest_lan: &FxHashSet<String> = lans.iter().max_by_key(|lan| lan.len()).unwrap();
        let mut sorted_lan: Vec<String> = biggest_lan.iter().cloned().collect();
        sorted_lan.sort();
        sorted_lan.join(",")
    }
}

impl FromStr for LanParty {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pair(s: &str) -> IResult<&str, (String, String)> {
            let (s, (a, b)) = separated_pair(alpha1, char('-'), alpha1)(s)?;
            Ok((s, (a.to_string(), b.to_string())))
        }

        let pairs: FxHashSet<(String, String)> =
            s.lines().map(|l| parse_pair(l).unwrap().1).collect();

        Ok(LanParty { pairs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_23.txt").expect("Cannot open input file");
    let lan: LanParty = s.parse().unwrap();
    println!("Part1: {}", lan.t_triplets());
    println!("Part2: {}", lan.biggest_lan());
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
