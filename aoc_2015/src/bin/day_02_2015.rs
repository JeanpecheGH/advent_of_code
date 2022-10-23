fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_02.txt").expect("Cannot open input file");

    let wrapping: Option<(u32, u32)> = lines
        .map(|l| {
            let mut sides: Vec<u32> = l
                .unwrap()
                .split('x')
                .map(|n| n.parse::<u32>().unwrap())
                .collect();
            sides.sort_unstable();
            let s = sides[0];
            let m = sides[1];
            let l = sides[2];
            (3 * s * m + 2 * s * l + 2 * m * l, 2 * s + 2 * m + s * m * l)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1));

    println!(
        "Part1: We need {:?} square feet of wrapping paper",
        wrapping.unwrap().0
    );
    println!("Part2: We need {:?} feet of ribbon", wrapping.unwrap().1);
}
