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

    fn coeff(&self, prod: usize) -> isize {
        fn inner_coeff(
            r: isize,
            u: isize,
            v: isize,
            r_prime: isize,
            u_prime: isize,
            v_prime: isize,
        ) -> (isize, isize, isize) {
            if r_prime == 0 {
                (r, u, v)
            } else {
                let q = r / r_prime;
                inner_coeff(
                    r_prime,
                    u_prime,
                    v_prime,
                    r - q * r_prime,
                    u - q * u_prime,
                    v - q * v_prime,
                )
            }
        }
        inner_coeff(prod as isize, 1, 0, self.period as isize, 0, 1).1
    }

    fn elem(&self, product: usize) -> isize {
        let prod = product / self.period;
        //Use -position
        -(self.position as isize) * self.coeff(prod) * prod as isize
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_15.txt").expect("Cannot open input file");
    let mut aligned_discs: Vec<Disc> = lines
        .map(|l| {
            let s: String = l.unwrap();
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
    let discs = aligned_discs.clone();
    let prod = discs.iter().map(|disc| disc.period).product();
    let solution: isize = discs.into_iter().map(|disc| disc.elem(prod)).sum();
    let elapsed = now.elapsed();
    println!(
        "Part1: You need to press the button at {}s (Answer found in {:?} with extended Euclid algorithm )",
        modulo(solution, prod as isize),
        elapsed
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
    let prod = aligned_discs.iter().map(|disc| disc.period).product();
    let solution: isize = aligned_discs.iter().map(|disc| disc.elem(prod)).sum();
    let elapsed = now.elapsed();
    println!(
        "Part2: You need to press the button at {}s (Answer found in {:?} with extended Euclid algorithm )",
        modulo(solution, prod as isize),
        elapsed
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

fn modulo(n: isize, modu: isize) -> isize {
    let r = n % modu;
    if r < 0 {
        r + modu
    } else {
        r
    }
}
