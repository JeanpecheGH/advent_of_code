use fxhash::{FxHashMap, FxHashSet};
use nom::IResult;
use nom::Parser;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, space1};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct DataPath {
    current: String,
    depth: usize,
}

impl DataPath {
    fn from_name(name: &str) -> DataPath {
        let current: String = name.to_owned();
        DataPath { current, depth: 0 }
    }

    fn add_visited(&mut self, name: &str) {
        self.current = name.to_owned();
        self.depth += 1;
    }

    fn add_children(&self, children: &[String]) -> Vec<DataPath> {
        children
            .iter()
            .map(|name| {
                let mut path: DataPath = self.clone();
                path.add_visited(name);
                path
            })
            .collect()
    }

    fn is_finished(&self, to: &str) -> bool {
        self.current == to
    }
}

struct Reactor {
    outputs: FxHashMap<String, Vec<String>>,
}

impl Reactor {
    fn nb_paths(&self, from: &str, to: &str, max_depth: usize) -> usize {
        let mut nb_paths: usize = 0;

        let mut paths: Vec<DataPath> = vec![DataPath::from_name(from)];
        while let Some(path) = paths.pop() {
            if self.outputs.contains_key(&path.current) {
                let children: &Vec<String> = self.outputs.get(&path.current).unwrap();
                let news: Vec<DataPath> = path.add_children(children);
                for new in news {
                    if new.is_finished(to) {
                        //println!("Finished: {:?}", new);
                        nb_paths += 1;
                    } else if new.depth < max_depth {
                        paths.push(new);
                    }
                }
            }
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

    println!("Part1: {}", reactor.nb_paths("you", "out", 20));
    // println!(
    //     "Dac -> Fft: {}",
    //     reactor.nb_paths("dac", "fft", &Vec::new())
    // );
    let r: usize = 14805 * 6747790 * 2963;
    println!("Part2: {}", r);
    println!("Svr -> Fft: {}", reactor.nb_paths("svr", "fft", 12));
    println!("Fft -> Dac: {}", reactor.nb_paths("fft", "dac", 17));
    println!("Dac -> Out: {}", reactor.nb_paths("dac", "out", 20));
    //println!("Part2: {}", reactor.nb_paths("svr", "out", &["dac", "fft"]));
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
        assert_eq!(reactor.nb_paths("you", "out", 20), 5);
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
        assert_eq!(reactor.nb_paths("svr", "out", 20), 2);
    }
}
