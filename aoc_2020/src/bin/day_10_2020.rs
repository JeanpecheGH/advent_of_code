fn main() {
    let s = util::file_as_string("aoc_2020/input/day_10.txt").expect("Cannot open input file");

    let mut numbers: Vec<usize> = s.lines().map(|s| s.parse().unwrap()).collect();
    numbers.push(0);
    numbers.sort();

    let diffs: Vec<usize> = numbers.windows(2).map(|pair| pair[1] - pair[0]).collect();

    let ones: usize = diffs.iter().filter(|&&d| d == 1).count();
    let threes: usize = diffs.iter().filter(|&&d| d == 3).count() + 1;

    println!(
        "Part1: There are {} differences of 1 Jolt and {} differences of 3 Jolt. The product is {}",
        ones,
        threes,
        ones * threes
    );

    let nb_arrangements: usize = diffs
        //Difference of 3 means no arrangement can be done (factor stays x1)
        .split(|&d| d == 3)
        //The biggest group of consecutive numbers we found is 5 (4 differences of 1)
        //A group of 5 consecutive numbers can be arranged 7 times (diff 2 => factor x5)
        // 1 ,2, 3, 4, 5
        // 1, 2, 3, 5
        // 1, 2, 4, 5
        // 1, 3, 4, 5
        // 1, 2, 5
        // 1, 3, 5
        // 1, 4, 5
        //A group of 4 consecutive numbers can be arranged 4 times (diff 2 => factor x4)
        //A group of 3 consecutive numbers can be arranged 2 times (diff 2 => factor x2)
        //A group of 2 consecutive numbers cannot be arranged (diff 1 => factor stays x1)
        .map(|group| {
            let l = group.len();
            match l {
                4 => 7,
                3 => 4,
                2 => 2,
                _ => 1,
            }
        })
        .product();
    println!("Part2: {}", nb_arrangements);
}
