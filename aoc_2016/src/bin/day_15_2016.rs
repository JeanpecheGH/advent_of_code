use util::chinese_remainders::smallest_remainder;

#[derive(Debug, Copy, Clone)]
struct Disc {
    delay: usize,
    position: usize,
    period: usize,
}

impl Disc {
    fn advance(&mut self) {
        self.position = (self.position + 1) % self.period
    }

    fn apply_delay(&mut self) {
        self.position = (self.position + self.delay) % self.period
    }

    fn is_zero(&self) -> bool {
        self.position == 0
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_15.txt").expect("Cannot open input file");
    let mut aligned_discs: Vec<Disc> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            let delay: usize = words[1].strip_prefix('#').unwrap().parse().unwrap();
            let period: usize = words[3].parse().unwrap();
            let position: usize = words[11].strip_suffix('.').unwrap().parse().unwrap();
            let mut d = Disc {
                delay,
                position,
                period,
            };
            d.apply_delay();
            d
        })
        .collect();

    //Part1 using extended Euclid algorithm
    let now = std::time::Instant::now();
    let discs_and_delays: Vec<(isize, isize)> = aligned_discs
        .iter()
        .map(|disc| (disc.period as isize, -(disc.position as isize)))
        .collect();
    let solution = smallest_remainder(discs_and_delays);
    println!(
        "Part1: You need to press the button at {}s (Answer found in {:?} with extended Euclid algorithm)",
        solution,
        now.elapsed()
    );

    let mut part1_discs = aligned_discs.clone();

    let now = std::time::Instant::now();
    let mut time: usize = 0;
    loop {
        if part1_discs.iter().all(|disc| disc.is_zero()) {
            break;
        }
        part1_discs.iter_mut().for_each(|disc| disc.advance());
        time += 1;
    }
    let elapsed = now.elapsed();
    println!(
        "Part1: You need to press the button at {time}s (Answer found in {:?})",
        elapsed
    );

    let mut new_disc = Disc {
        delay: 7,
        position: 0,
        period: 11,
    };
    new_disc.apply_delay();
    aligned_discs.push(new_disc);

    //Part2 using extended Euclid algorithm
    let now = std::time::Instant::now();
    let discs_and_delays: Vec<(isize, isize)> = aligned_discs
        .iter()
        .map(|disc| (disc.period as isize, -(disc.position as isize)))
        .collect();
    let solution = smallest_remainder(discs_and_delays);
    println!(
        "Part2: You need to press the button at {}s (Answer found in {:?} with extended Euclid algorithm)",
        solution,
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let mut time: usize = 0;
    loop {
        if aligned_discs.iter().all(|disc| disc.is_zero()) {
            break;
        }
        aligned_discs.iter_mut().for_each(|disc| disc.advance());
        time += 1;
    }
    let elapsed = now.elapsed();
    println!(
        "Part2: You need to press the button at {time}s with this new disc (Answer found in {:?})",
        elapsed
    );
}
