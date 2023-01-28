use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_06.txt").expect("Cannot open input file");

    let bad_words: Vec<&str> = s.lines().collect();
    let mut counters: [Vec<char>; 8] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];
    bad_words
        .iter()
        .for_each(|w| w.chars().enumerate().for_each(|(i, c)| counters[i].push(c)));

    let counters_map: Vec<HashMap<char, usize>> = counters
        .into_iter()
        .map(|counter| counter.into_iter().counts())
        .collect();

    let first_pwd: String = counters_map
        .iter()
        .map(|map| {
            map.iter()
                .max_by(|&a, &b| a.1.cmp(b.1))
                .unwrap()
                .0
                .to_owned()
        })
        .collect();
    println!("Part1: The password is {first_pwd}");

    let second_pwd: String = counters_map
        .iter()
        .map(|map| {
            map.iter()
                .min_by(|&a, &b| a.1.cmp(b.1))
                .unwrap()
                .0
                .to_owned()
        })
        .collect();
    println!("Part2: The password is {second_pwd}");
}
