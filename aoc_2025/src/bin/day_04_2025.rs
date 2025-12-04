use fxhash::FxHashSet;
use rayon::prelude::*;
use std::str::FromStr;
use util::coord::Pos;

struct PrintingDepartment {
    rolls: FxHashSet<Pos>,
}

impl PrintingDepartment {
    fn directly_accessible(&self) -> usize {
        self.accessible(&self.rolls).len()
    }

    fn eventually_accessible(&self) -> usize {
        let mut rolls: FxHashSet<Pos> = self.rolls.clone();
        let starting_len: usize = rolls.len();
        loop {
            let acc: Vec<Pos> = self.accessible(&rolls);
            if acc.is_empty() {
                break;
            }
            for p in acc {
                rolls.remove(&p);
            }
        }
        let final_len: usize = rolls.len();
        starting_len - final_len
    }

    fn accessible(&self, rolls: &FxHashSet<Pos>) -> Vec<Pos> {
        rolls
            .par_iter()
            .filter(|p| {
                let nb_ngb: usize = p
                    .neighbours_diag_safe(1000, 1000)
                    .iter()
                    .filter(|&ngb| rolls.contains(ngb))
                    .count();
                nb_ngb < 4
            })
            .copied()
            .collect()
    }
}

impl FromStr for PrintingDepartment {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut rolls: FxHashSet<Pos> = FxHashSet::default();
        s.lines().enumerate().for_each(|(y, l)| {
            l.chars().enumerate().for_each(|(x, c)| {
                if c == '@' {
                    let _ = rolls.insert(Pos(x, y));
                }
            })
        });

        Ok(PrintingDepartment { rolls })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_04.txt").expect("Cannot open input file");
    let department: PrintingDepartment = s.parse().unwrap();

    println!(
        "Part1: {} rolls of paper are directly accessible",
        department.directly_accessible()
    );
    println!(
        "Part2: {} rolls of paper can eventually be removed",
        department.eventually_accessible()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
";
    #[test]
    fn test_part_1() {
        let department: PrintingDepartment = EXAMPLE_1.parse().unwrap();
        assert_eq!(department.directly_accessible(), 13);
    }

    #[test]
    fn test_part_2() {
        let department: PrintingDepartment = EXAMPLE_1.parse().unwrap();
        assert_eq!(department.eventually_accessible(), 43);
    }
}
