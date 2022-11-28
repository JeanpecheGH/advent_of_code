use std::cmp::max;

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_20.txt").expect("Cannot open input file");

    let now = std::time::Instant::now();
    let mut bls: Vec<(u64, u64)> = lines
        .map(|l| {
            let s: String = l.unwrap();
            let words: Vec<u64> = s.split('-').map(|w| w.parse().unwrap()).collect();
            (words[0], words[1])
        })
        .collect();

    bls.sort();

    println!(
        "There are {} ips allowed, the first one is: {}, found in {:?}",
        nb_allowed(&bls),
        min_allowed(&bls),
        now.elapsed()
    );
}

fn min_allowed(bls: &[(u64, u64)]) -> u64 {
    bls.iter().fold(0, |acc, (low, high)| {
        if *low > acc {
            return acc;
        }
        max(high + 1, acc)
    })
}

fn nb_allowed(bls: &[(u64, u64)]) -> u64 {
    bls.iter()
        .fold((0, 0), |(high_all, count), (low, high)| {
            if *low < high_all {
                (max(high_all, *high + 1), count)
            } else {
                (*high + 1, count + (low - high_all))
            }
        })
        .1
}
