use util::hashers::KnotHash;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_10.txt").expect("Cannot open input file");
    let khash: KnotHash = s.parse().unwrap();
    println!(
        "Part1: The simple hash method gives {}",
        khash.weak_hash(256)
    );
    let khash: KnotHash = KnotHash::new(s.lines().next().unwrap());
    println!(
        "Part2: When using the complete hash method, the result is {}",
        khash.hash()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "3,4,1,5";

    #[test]
    fn part_1() {
        let khash: KnotHash = EXAMPLE_1.parse().unwrap();
        assert_eq!(12, khash.weak_hash(5));
    }

    #[test]
    fn part_2_test_1() {
        let khash: KnotHash = KnotHash::new("");
        assert_eq!("a2582a3a0e66e6e86e3812dcb672a272", khash.hash());
    }

    #[test]
    fn part_2_test_2() {
        let khash: KnotHash = KnotHash::new("AoC 2017");
        assert_eq!("33efeb34ea91902bb2f59c9920caa6cd", khash.hash());
    }

    #[test]
    fn part_2_test_3() {
        let khash: KnotHash = KnotHash::new("1,2,3");
        assert_eq!("3efbe78a8d82f29979031a4aa0b16a9d", khash.hash());
    }

    #[test]
    fn part_2_test_4() {
        let khash: KnotHash = KnotHash::new("1,2,4");
        assert_eq!("63960835bcdc130f0b66d7ff4f6a5a8e", khash.hash());
    }
}
