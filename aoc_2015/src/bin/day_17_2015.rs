use std::collections::HashMap;

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_17.txt").expect("Cannot open input file");

    let target_volume: u32 = 150;
    let mut buckets: Vec<u32> = s.lines().map(|s| s.parse::<u32>().unwrap()).collect();

    buckets.sort_unstable();

    let res = fill_buckets(buckets, target_volume, 0);
    println!(
        "Part1: There are {} ways to fill the buckets",
        res.values().sum::<u32>()
    );

    let min_key = res.keys().min().unwrap();
    let min_ways = res.get(min_key).unwrap();
    println!("Part2: There are {min_ways} ways to fill the minimum number of buckets");
}

fn fill_buckets(mut buckets: Vec<u32>, target: u32, nb_bucket: u32) -> HashMap<u32, u32> {
    match (buckets.pop(), target) {
        (Some(b), t) if t >= b => merge_map(
            fill_buckets(buckets.clone(), t - b, nb_bucket + 1),
            fill_buckets(buckets, t, nb_bucket),
        ),
        (Some(_), t) => fill_buckets(buckets, t, nb_bucket),
        (None, 0) => {
            let mut h = HashMap::new();
            h.insert(nb_bucket, 1);
            h
        }
        _ => HashMap::new(),
    }
}

fn merge_map(mut map_1: HashMap<u32, u32>, map_2: HashMap<u32, u32>) -> HashMap<u32, u32> {
    map_2.iter().for_each(|(&k, &v)| {
        let entry = map_1.entry(k).or_insert(0);
        *entry += v;
    });
    map_1
}
