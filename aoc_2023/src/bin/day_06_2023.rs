use std::str::FromStr;

struct BoatRace {
    time: usize,
    distance: usize,
}

impl BoatRace {
    fn beat_distance(&self, hold_time: usize) -> bool {
        (self.time - hold_time) * hold_time > self.distance
    }

    //We only need to find the first and last hold times beating the distance
    //Every hold time in between will be good too
    fn possibilities(&self) -> usize {
        let short_hold: usize = (1..self.time)
            .find(|hold| self.beat_distance(*hold))
            .unwrap();
        let long_hold: usize = (1..self.time)
            .rev()
            .find(|hold| self.beat_distance(*hold))
            .unwrap();
        long_hold - short_hold + 1
    }
}

struct Races {
    races: Vec<BoatRace>,
    mono_race: BoatRace,
}

impl Races {
    fn possibility_product(&self) -> usize {
        self.races.iter().map(|race| race.possibilities()).product()
    }

    fn mono_race_possibilities(&self) -> usize {
        self.mono_race.possibilities()
    }
}

impl FromStr for Races {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn numbers_in_line(line: &str) -> Vec<usize> {
            line.split_once(':')
                .unwrap()
                .1
                .split_whitespace()
                .map(|n| n.parse().unwrap())
                .collect()
        }

        fn number_in_line(line: &str) -> usize {
            let mut time: String = line.split_once(':').unwrap().1.to_string();
            time.retain(|c| !c.is_whitespace());
            time.parse().unwrap()
        }

        let mut lines = s.lines();
        let first: &str = lines.next().unwrap();
        let second: &str = lines.next().unwrap();
        let times: Vec<usize> = numbers_in_line(first);
        let distances: Vec<usize> = numbers_in_line(second);

        let races: Vec<BoatRace> = (0..times.len())
            .map(|i| BoatRace {
                time: times[i],
                distance: distances[i],
            })
            .collect();

        let time: usize = number_in_line(first);
        let distance: usize = number_in_line(second);
        let mono_race: BoatRace = BoatRace { time, distance };

        Ok(Races { races, mono_race })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_06.txt").expect("Cannot open input file");
    let races: Races = s.parse().unwrap();

    println!(
        "Part1: The product of the number of ways to win each race is {}",
        races.possibility_product()
    );
    println!(
        "Part2: The number of ways to win the big race is {}",
        races.mono_race_possibilities()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "Time:      7  15   30
Distance:  9  40  200";

    #[test]
    fn part_1() {
        let races: Races = EXAMPLE_1.parse().unwrap();
        assert_eq!(races.possibility_product(), 288);
    }
    #[test]
    fn part_2() {
        let races: Races = EXAMPLE_1.parse().unwrap();
        assert_eq!(races.mono_race_possibilities(), 71503);
    }
}
