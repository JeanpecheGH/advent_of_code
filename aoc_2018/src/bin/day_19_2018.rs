use util::wrist_device::WristDevice;

fn div_sum(n: usize) -> usize {
    let sqrt: usize = (n as f64).sqrt() as usize;
    let mut sum: usize = 0;
    for i in 1..=sqrt {
        if n.is_multiple_of(i) {
            let d = n / i;
            if d != i {
                sum += d + i;
            } else {
                sum += d;
            }
        }
    }
    sum
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_19.txt").expect("Cannot open input file");
    let mut device: WristDevice = s.parse().unwrap();
    device.apply_all_with_pointer(false);

    println!(
        "Part1: When the process halts, register 0 contains {}",
        device.reg_value(0)
    );
    device.reset();
    device.set_reg_value(0, 1);
    device.apply_all_with_pointer(true);
    let to_divide: usize = device.reg_value(2);

    //Answer is 10996992
    println!(
        "Part2: When the process halts, register 0 should contain {}",
        div_sum(to_divide)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use util::wrist_device::WristDevice;
    const EXAMPLE_1: &str = "#ip 0
seti 5 0 1
seti 6 0 2
addi 0 1 0
addr 1 2 3
setr 1 0 0
seti 8 0 4
seti 9 0 5";

    #[test]
    fn part_1() {
        let mut device: WristDevice = EXAMPLE_1.parse().unwrap();
        device.apply_all_with_pointer(false);
        assert_eq!(device.reg_value(0), 7);
    }
}
