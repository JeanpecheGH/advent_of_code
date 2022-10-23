use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use util;

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_09.txt").expect("Cannot open input file");

    let mut towns: HashSet<String> = HashSet::new();
    let mut distances: HashMap<(String, String), u16> = HashMap::new();

    lines.for_each(|l| {
        let s = l.unwrap();
        let split: Vec<&str> = s.split(' ').collect();
        let town_1 = split[0];
        let town_2 = split[2];
        let dist = split[4].parse::<u16>().unwrap();
        towns.insert(town_1.to_string());
        towns.insert(town_2.to_string());
        distances.insert((town_1.to_string(), town_2.to_string()), dist);
        distances.insert((town_2.to_string(), town_1.to_string()), dist);
    });

    let dists: Vec<u16> = towns
        .iter()
        .permutations(towns.len())
        .map(|perm| {
            perm.windows(2)
                .map(|pair| {
                    let tuple: (String, String) = (pair[0].to_string(), pair[1].to_string());
                    distances.get(&tuple).cloned().unwrap()
                })
                .sum::<u16>()
        })
        .collect();

    let min_dist: u16 = dists.iter().copied().min().unwrap();
    let max_dist: u16 = dists.iter().copied().max().unwrap();

    println!("Part1: Min distance to cover is {}", min_dist);
    println!("Part2: Max distance to cover is {}", max_dist);
}
