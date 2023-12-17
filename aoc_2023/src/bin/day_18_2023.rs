use std::str::FromStr;
struct FactoryCity {
}

impl FactoryCity {
}

impl FromStr for FactoryCity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(FactoryCity {  })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_18.txt").expect("Cannot open input file");
    let city: FactoryCity = s.parse().unwrap();
    println!(
        "Part1: "
    );
    println!(
        "Part2: "
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "";

    #[test]
    fn part_1() {
        let city: FactoryCity = EXAMPLE_1.parse().unwrap();
        assert_eq!(1,1);
    }
    #[test]
    fn part_2_test_1() {
        let city: FactoryCity = EXAMPLE_1.parse().unwrap();
        assert_eq!(1,1);
    }
}
