use itertools::Itertools;
use std::collections::{HashMap, HashSet};

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_13.txt").expect("Cannot open input file");

    let mut happy_map: HashMap<(String, String), i32> = HashMap::new();
    let mut guests: HashSet<String> = HashSet::new();

    lines.for_each(|l| {
        let s = l.unwrap();
        let words: Vec<&str> = s.split(' ').collect();
        guests.insert(words[0].to_string());
        let val: i32 = words[3].parse().unwrap();

        //Remove trailing dot from last name
        let mut last_name = words[10].chars();
        last_name.next_back();
        let key = (words[0].to_string(), last_name.as_str().to_string());
        let rev_key = (last_name.as_str().to_string(), words[0].to_string());

        match words[2] {
            "gain" => {
                let entry = happy_map.entry(key).or_insert(0);
                *entry += val;
                let rev_entry = happy_map.entry(rev_key).or_insert(0);
                *rev_entry += val;
            }
            "lose" => {
                let entry = happy_map.entry(key).or_insert(0);
                *entry -= val;
                let rev_entry = happy_map.entry(rev_key).or_insert(0);
                *rev_entry -= val;
            }
            _ => (),
        }
    });

    println!("Guests: {:?}", guests);

    let total_happy_sum: Option<i32> = guests
        .iter()
        .permutations(guests.len())
        .map(|perm| {
            let mut happy_sum: i32 = perm
                .windows(2)
                .flat_map(|pair| happy_map.get(&(pair[0].to_string(), pair[1].to_string())))
                .sum();
            //We need to join the table in a circle
            let first: String = perm.first().cloned().cloned().unwrap();
            let last: String = perm.last().cloned().cloned().unwrap();
            happy_sum += happy_map.get(&(first, last)).unwrap();
            happy_sum
        })
        .max();

    println!(
        "Part1: The maximum happiness change is {}",
        total_happy_sum.unwrap()
    );

    let total_happy_sum_2: Option<i32> = guests
        .iter()
        .permutations(guests.len())
        .map(|perm| {
            //We don't need to join the table in a circle, there is you !
            let happy_sum: i32 = perm
                .windows(2)
                .flat_map(|pair| happy_map.get(&(pair[0].to_string(), pair[1].to_string())))
                .sum();
            happy_sum
        })
        .max();

    println!(
        "Part2: The maximum happiness change is now {}",
        total_happy_sum_2.unwrap()
    );
}
