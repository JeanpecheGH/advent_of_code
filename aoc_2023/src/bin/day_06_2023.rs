use nom::sequence::preceded;
use std::str::FromStr;
use util::basic_parser::{title, usize_list};

struct BoatRace {
    time: usize,
    distance: usize,
}

impl BoatRace {
    fn possibilities(&self) -> usize {
        //We want to solve the quadratic equation: h(t-h) - d = 0 <=> -h² + th -d = 0
        //Discriminant is t² - 4d
        let t: f64 = self.time as f64;
        let d: f64 = self.distance as f64;
        let discr: f64 = (t * t - 4_f64 * d).sqrt();
        //Root one will be the bigger one
        //We have to beat the times, not just equal it, so we add/remove 0.0001 to make sure that floor/ceil does something
        let root_one: usize = (((-t - discr) / -2_f64) - 0.0001).floor() as usize;
        let root_two: usize = ((-t + discr) / -2_f64 + 0.0001).ceil() as usize;

        root_one - root_two + 1
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
        fn number_in_line(line: &str) -> usize {
            let mut time: String = line.split_once(':').unwrap().1.to_string();
            time.retain(|c| !c.is_whitespace());
            time.parse().unwrap()
        }

        let mut lines = s.lines();
        let first: &str = lines.next().unwrap();
        let second: &str = lines.next().unwrap();
        let times: Vec<usize> = preceded(title, usize_list)(first).unwrap().1;
        let distances: Vec<usize> = preceded(title, usize_list)(second).unwrap().1;

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
