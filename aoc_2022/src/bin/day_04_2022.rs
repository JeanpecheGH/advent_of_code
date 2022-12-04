type Pair = (usize, usize);

fn main() {
    let lines = util::file_as_lines("aoc_2022/input/day_04.txt").expect("Cannot open input file");

    let ranges_pair: Vec<(Pair, Pair)> = lines
        .map(|l| {
            let s: String = l.unwrap();
            let pairs: Vec<&str> = s.split(',').collect();
            let pair_1: Vec<&str> = pairs[0].split('-').collect();
            let pair_2: Vec<&str> = pairs[1].split('-').collect();
            (
                (pair_1[0].parse().unwrap(), pair_1[1].parse().unwrap()),
                (pair_2[0].parse().unwrap(), pair_2[1].parse().unwrap()),
            )
        })
        .collect();

    let nb_includes = filter_and_count(&ranges_pair, &include);
    println!(
        "Part1: The number of pairs where a range includes the other is {}",
        nb_includes
    );

    let nb_overlap = filter_and_count(&ranges_pair, &overlap);
    println!(
        "Part2: The number of pairs where the ranges overlap with one another is {}",
        nb_overlap
    );
}

fn filter_and_count(pairs: &[(Pair, Pair)], f: &dyn Fn(Pair, Pair) -> bool) -> usize {
    pairs.iter().filter(|&&(a, b)| f(a, b)).count()
}

fn include((s_1, e_1): Pair, (s_2, e_2): Pair) -> bool {
    (s_1 <= s_2 && e_1 >= e_2) || (s_2 <= s_1 && e_2 >= e_1)
}

fn overlap((s_1, e_1): Pair, (s_2, e_2): Pair) -> bool {
    (s_1 <= s_2 && e_1 >= s_2) || (s_2 <= s_1 && e_2 >= s_1)
}
