use fxhash::FxHashMap;
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space1};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use std::str::FromStr;

struct Reactor {
    outputs: FxHashMap<String, Vec<String>>,
}

impl Reactor {
    fn paths_between(
        cache: &mut FxHashMap<usize, usize>,
        map: &FxHashMap<usize, Vec<usize>>,
        from: usize,
        to: usize,
    ) -> usize {
        if from == to {
            1
        } else if cache.contains_key(&from) {
            cache[&from]
        } else {
            let nb_paths: usize = if let Some(children) = map.get(&from) {
                children
                    .iter()
                    .map(|&child| Reactor::paths_between(cache, map, child, to))
                    .sum()
            } else {
                0
            };
            cache.insert(from, nb_paths);
            nb_paths
        }
    }

    fn paths(&self, nodes: &[&str]) -> usize {
        fn node_to_usize(name: &str) -> usize {
            name.chars().fold(0, |mut acc, c| {
                acc *= 26;
                acc += c as usize - 'a' as usize;
                acc
            })
        }

        let map: FxHashMap<usize, Vec<usize>> = self
            .outputs
            .iter()
            .map(|(k, v)| {
                let k_hash: usize = node_to_usize(k);
                let values: Vec<usize> = v.iter().map(|n| node_to_usize(n)).collect();
                (k_hash, values)
            })
            .collect();
        let mut nb_paths: usize = 1;
        let nodes: Vec<usize> = nodes.iter().map(|n| node_to_usize(n)).collect();

        for pair in nodes.windows(2).rev() {
            let start: usize = pair[0];
            let end: usize = pair[1];
            let mut cache: FxHashMap<usize, usize> = FxHashMap::default();
            let nb: usize = Reactor::paths_between(&mut cache, &map, start, end);
            nb_paths *= nb;
        }
        nb_paths
    }
}

impl FromStr for Reactor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_outputs(s: &str) -> IResult<&str, (String, Vec<String>)> {
            let (s, name) = terminated(alpha1, tag(": ")).parse(s)?;
            let (s, outputs) = separated_list1(space1, alpha1).parse(s)?;
            let outputs: Vec<String> = outputs.into_iter().map(|n| n.to_string()).collect();
            Ok((s, (name.to_string(), outputs)))
        }

        let outputs: FxHashMap<String, Vec<String>> =
            s.lines().map(|l| parse_outputs(l).unwrap().1).collect();
        Ok(Reactor { outputs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_11.txt").expect("Cannot open input file");
    let reactor: Reactor = s.parse().unwrap();

    println!(
        "Part1: There are {} paths between \"you\" and \"out\"",
        reactor.paths(&["you", "out"])
    );
    println!(
        "Part2: There are {} paths between \"svr\" and \"out\" visiting \"fft\" and \"dac\"",
        reactor.paths(&["svr", "fft", "dac", "out"])
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out
";
    #[test]
    fn test_part_1() {
        let reactor: Reactor = EXAMPLE_1.parse().unwrap();
        assert_eq!(reactor.paths(&["you", "out"]), 5);
    }

    const EXAMPLE_2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
";

    #[test]
    fn test_part_2() {
        let reactor: Reactor = EXAMPLE_2.parse().unwrap();
        assert_eq!(reactor.paths(&["svr", "fft", "dac", "out"]), 2);
    }
}
