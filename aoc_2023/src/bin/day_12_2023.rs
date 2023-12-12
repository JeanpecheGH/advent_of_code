use std::collections::HashMap;
use std::str::FromStr;

#[derive(Eq, PartialEq, Copy, Clone, Debug, Hash)]
enum SpringState {
    Operational,
    Damaged,
    Unknown,
}

impl SpringState {
    fn from_char(c: char) -> SpringState {
        match c {
            '.' => SpringState::Operational,
            '#' => SpringState::Damaged,
            '?' => SpringState::Unknown,
            _ => panic!("Impossible SpringState {c}"),
        }
    }

    fn can_be_damaged(&self) -> bool {
        *self == SpringState::Unknown || *self == SpringState::Damaged
    }

    fn can_be_operational(&self) -> bool {
        *self == SpringState::Unknown || *self == SpringState::Operational
    }
}

#[derive(Eq, PartialEq, Hash, Clone)]
struct SpringsRow {
    states: Vec<SpringState>,
    records: Vec<usize>,
}

impl SpringsRow {
    //We had a #, check that we can remove a full block of #
    fn remove_record(&mut self) -> bool {
        let contains_record = self
            .records
            .pop()
            .map(|record| {
                (0..record - 1).all(|_| {
                    self.states
                        .pop()
                        .map(|state| state.can_be_damaged())
                        .unwrap_or(false)
                })
            })
            .unwrap_or(false);
        let ends: bool = self
            .states
            .pop()
            .map(|state| state.can_be_operational())
            .unwrap_or(true);
        contains_record && ends
    }

    fn arrangements(&self, arr_map: &mut HashMap<SpringsRow, usize>) -> usize {
        if let Some(&value) = arr_map.get(self) {
            value
        } else {
            //The row is invalid if we cannot insert enough Damaged/Operational states in it
            let value = if self.states.len() + 1
                < self.records.iter().sum::<usize>() + self.records.len()
            {
                0
            } else {
                let mut clone: SpringsRow = self.clone();
                //Check the first Spring State
                match clone.states.pop() {
                    //Operational, we can just remove it
                    Some(SpringState::Operational) => clone.arrangements(arr_map),
                    //Damaged, we check if we can fit the first record in there
                    Some(SpringState::Damaged) => {
                        if clone.remove_record() {
                            clone.arrangements(arr_map)
                        } else {
                            0
                        }
                    }
                    //Unknown, we split here
                    Some(SpringState::Unknown) => {
                        let op: usize = clone.arrangements(arr_map);
                        let dam: usize = if clone.remove_record() {
                            clone.arrangements(arr_map)
                        } else {
                            0
                        };
                        op + dam
                    }
                    //No more states, no more records, it's a possible arrangement
                    _ if clone.records.is_empty() => 1,
                    //Impossible arrangement
                    _ => 0,
                }
            };
            //Memoization!
            arr_map.insert(self.clone(), value);

            value
        }
    }

    fn unfold(&self) -> SpringsRow {
        let mut states = self.states.clone();
        let mut records = self.records.clone();

        (0..4).for_each(|_| {
            states.push(SpringState::Unknown);
            states.extend(self.states.clone());

            records.extend(self.records.clone());
        });

        SpringsRow { states, records }
    }
}

impl FromStr for SpringsRow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (states, records) = s.split_once(' ').unwrap();
        let states: Vec<SpringState> = states.chars().map(SpringState::from_char).collect();
        let records: Vec<usize> = records
            .split(',')
            .map(|n| n.parse::<usize>().unwrap())
            .collect();
        Ok(SpringsRow { states, records })
    }
}

struct HotSprings {
    rows: Vec<SpringsRow>,
}

impl HotSprings {
    fn arrangements(&self) -> usize {
        let mut arr_map: HashMap<SpringsRow, usize> = HashMap::new();
        self.rows
            .iter()
            .map(|row| row.arrangements(&mut arr_map))
            .sum()
    }

    fn unfold(&self) -> HotSprings {
        let rows: Vec<SpringsRow> = self.rows.iter().map(|row| row.unfold()).collect();
        HotSprings { rows }
    }
}

impl FromStr for HotSprings {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rows: Vec<SpringsRow> = s.lines().map(|row| row.parse().unwrap()).collect();
        Ok(HotSprings { rows })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_12.txt").expect("Cannot open input file");
    let springs: HotSprings = s.parse().unwrap();

    println!(
        "Part1: The sum of all the possible arrangements is {}",
        springs.arrangements()
    );
    let unfolded_springs: HotSprings = springs.unfold();
    println!(
        "Part2: After unfolding the records, the sum is now {}",
        unfolded_springs.arrangements()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
";

    #[test]
    fn part_1() {
        let springs: HotSprings = EXAMPLE_1.parse().unwrap();
        assert_eq!(springs.arrangements(), 21);
    }
    #[test]
    fn part_2() {
        let springs: HotSprings = EXAMPLE_1.parse().unwrap();
        let unfolded_springs: HotSprings = springs.unfold();
        assert_eq!(unfolded_springs.arrangements(), 525152);
    }
}
