#[derive(Debug, Copy, Clone)]
struct Disc {
    delay: u16,
    position: u16,
    period: u16,
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
    let lines = util::file_as_lines("aoc_2016/input/day_15.txt").expect("Cannot open input file");
    let mut aligned_discs: Vec<Disc> = lines
        .map(|l| {
            let s: String = l.unwrap();
            let words: Vec<&str> = s.split_whitespace().collect();
            let delay: u16 = words[1].strip_prefix('#').unwrap().parse().unwrap();
            let period: u16 = words[3].parse().unwrap();
            let position: u16 = words[11].strip_suffix('.').unwrap().parse().unwrap();
            let mut d = Disc {
                delay,
                position,
                period,
            };
            d.apply_delay();
            d
        })
        .collect();

    let mut part1_discs = aligned_discs.clone();

    let now = std::time::Instant::now();
    let mut time: u32 = 0;
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

    let now = std::time::Instant::now();
    let mut time: u32 = 0;
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
