use std::str::FromStr;

struct Bank {
    units: Vec<usize>,
}

impl Bank {
    fn best_in_slice(&self, start: usize, end: usize) -> (usize, usize) {
        //Return biggest value and its position in the vec
        let mut joltage: usize = 0;
        let mut idx: usize = 0;
        for (i, &n) in self.units[start..end].iter().enumerate() {
            if n > joltage {
                joltage = n;
                idx = start + i;
            }
        }
        (idx, joltage)
    }

    fn joltage(&self, nb_units: usize) -> usize {
        let mut joltage = 0;
        //Find the biggest (and first) unit with enough unit remaining to fill the batteries
        let mut start: usize = 0;
        let len: usize = self.units.len();
        for i in (0..nb_units).rev() {
            let (new_start, best_unit): (usize, usize) = self.best_in_slice(start, len - i);
            start = new_start + 1;
            joltage = joltage * 10 + best_unit;
        }
        joltage
    }
}

impl FromStr for Bank {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let units: Vec<usize> = s
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        Ok(Bank { units })
    }
}

struct ElevatorBatteries {
    banks: Vec<Bank>,
}

impl ElevatorBatteries {
    fn joltage(&self, nb_units: usize) -> usize {
        self.banks.iter().map(|b| b.joltage(nb_units)).sum()
    }
}

impl FromStr for ElevatorBatteries {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let banks: Vec<Bank> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(ElevatorBatteries { banks })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_03.txt").expect("Cannot open input file");
    let batteries: ElevatorBatteries = s.parse().unwrap();

    println!(
        "Part1: The max output joltage for 2 units per bank is {}",
        batteries.joltage(2)
    );
    println!(
        "Part2: For 12 units per bank, it is {}",
        batteries.joltage(12)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "987654321111111
811111111111119
234234234234278
818181911112111
";
    #[test]
    fn test_part_1() {
        let batteries: ElevatorBatteries = EXAMPLE_1.parse().unwrap();
        assert_eq!(batteries.joltage(2), 357);
    }
    #[test]
    fn test_part_2() {
        let batteries: ElevatorBatteries = EXAMPLE_1.parse().unwrap();
        assert_eq!(batteries.joltage(12), 3_121_910_778_619);
    }
}
