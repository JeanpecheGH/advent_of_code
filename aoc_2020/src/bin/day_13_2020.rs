use util::chinese_remainders::smallest_remainder;

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_13.txt").expect("Cannot open input file");

    let mut lines = s.lines();
    let timestamp: usize = lines.next().unwrap().parse().unwrap();
    let bus_line: &str = lines.next().unwrap();
    let buses: Vec<usize> = bus_line.split(',').flat_map(|w| w.parse()).collect();

    let mut waiting_times: Vec<(usize, usize)> =
        buses.iter().map(|&b| (b, b - timestamp % b)).collect();
    waiting_times.sort_by(|a, b| a.1.cmp(&b.1));
    let first_bus = waiting_times.first().unwrap();
    println!(
        "Part1: The first bus departure is bus {} in {} minutes. The product is {}",
        first_bus.0,
        first_bus.1,
        first_bus.0 * first_bus.1
    );

    let buses_and_delays: Vec<(isize, isize)> = bus_line
        .split(',')
        .enumerate()
        .filter_map(|(delay, w)| {
            if let Ok(bus) = w.parse() {
                Some((bus, -(delay as isize)))
            } else {
                None
            }
        })
        .collect();
    let solution: isize = smallest_remainder(buses_and_delays);
    println!(
        "Part2: The earliest timestamp that solves the bus delays is {}",
        solution,
    );
}
