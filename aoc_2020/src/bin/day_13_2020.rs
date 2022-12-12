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

    let buses_starts: Vec<(usize, usize)> = bus_line
        .split(',')
        .enumerate()
        .filter_map(|(i, w)| {
            if let Ok(n) = w.parse() {
                Some((i, n))
            } else {
                None
            }
        })
        .collect();

    let product: usize = buses_starts.iter().map(|(_, period)| period).product();
    let solution: isize = buses_starts
        .iter()
        .map(|(delay, period)| elem(*period, *delay, product))
        .sum();
    println!(
        "Part2: The earliest timestamp that solves the bus delays is {}",
        modulo(solution, product as isize)
    )
}

fn modulo(n: isize, modu: isize) -> isize {
    let r = n % modu;
    if r < 0 {
        r + modu
    } else {
        r
    }
}

//Using extended Euclid algorithm
fn elem(period: usize, delay: usize, product: usize) -> isize {
    let prod = product / period;
    //Use -delay
    -(delay as isize) * coeff(period, prod) * prod as isize
}

fn coeff(period: usize, prod: usize) -> isize {
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
    inner_coeff(prod as isize, 1, 0, period as isize, 0, 1).1
}
