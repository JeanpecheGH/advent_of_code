type Pair = (usize, usize);

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_04.txt").expect("Cannot open input file");

    let ranges_pair: Vec<(Pair, Pair)> = s
        .lines()
        .map(|s| {
            let nbs: Vec<&str> = s.split(&[',', '-']).collect();
            (
                (nbs[0].parse().unwrap(), nbs[1].parse().unwrap()),
                (nbs[2].parse().unwrap(), nbs[3].parse().unwrap()),
            )
        })
        .collect();

    let nb_includes = filter_and_count(&ranges_pair, include);
    println!("Part1: The number of pairs where a range includes the other is {nb_includes}",);

    let nb_overlap = filter_and_count(&ranges_pair, overlap);
    println!(
        "Part2: The number of pairs where the ranges overlap with one another is {nb_overlap}",
    );
}

fn filter_and_count(pairs: &[(Pair, Pair)], f: fn(Pair, Pair) -> bool) -> usize {
    pairs.iter().filter(|&&(a, b)| f(a, b)).count()
}

fn include((s_1, e_1): Pair, (s_2, e_2): Pair) -> bool {
    (s_1 <= s_2 && e_1 >= e_2) || (s_2 <= s_1 && e_2 >= e_1)
}

fn overlap((s_1, e_1): Pair, (s_2, e_2): Pair) -> bool {
    !(s_1 > e_2 || s_2 > e_1)
}
