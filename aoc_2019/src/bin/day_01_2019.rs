fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_01.txt").expect("Cannot open input file");
    let sum: usize = s
        .lines()
        .map(|l| {
            let n: usize = l.parse().unwrap();
            fuel_for_mass(n)
        })
        .sum();
    println!("Part1: The sum of all the fuel requirements is {}", sum);
    let total_sum: usize = s
        .lines()
        .map(|l| {
            let n: usize = l.parse().unwrap();
            total_fuel(n)
        })
        .sum();
    println!(
        "Part2: The total sum of all the fuel requirements actually is {}",
        total_sum
    );
    println!("Computing time: {:?}", now.elapsed());
}

fn fuel_for_mass(n: usize) -> usize {
    let d: usize = n / 3;
    if d >= 2 {
        d - 2
    } else {
        0
    }
}

fn total_fuel(n: usize) -> usize {
    let mut current: usize = fuel_for_mass(n);
    let mut total: usize = current;
    while current > 0 {
        current = fuel_for_mass(current);
        total += current;
    }
    total
}
