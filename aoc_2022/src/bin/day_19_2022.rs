use std::cmp::max;
use std::str::FromStr;

const NB_ELEM: usize = 4;
const ORE: usize = 0;
const CLAY: usize = 1;
const OBSI: usize = 2;
const GEO: usize = 3;
const CLAY_ORE_MINUTES: usize = 19;

#[derive(Debug, Copy, Clone)]
struct Factory {
    id: u16,
    ore_cost: u16,
    clay_cost: u16,
    obsi_cost: (u16, u16),
    geo_cost: (u16, u16),
    no_build: [bool; NB_ELEM],
    store: [u16; NB_ELEM],
    robots: [u16; NB_ELEM],
}

impl Factory {
    fn next_minute(&self, minutes: usize) -> Vec<Self> {
        let mut potential_next: Vec<Option<usize>> = Vec::new();
        let mut no_build = [false; NB_ELEM];
        match (self.store[ORE], self.store[CLAY], self.store[OBSI], minutes) {
            //If enough for geode, build geode
            (ore, _, obsi, _) if ore >= self.geo_cost.0 && obsi >= self.geo_cost.1 => {
                potential_next.push(Some(GEO));
            }
            //If enough for obsi, build obsi or nothing if already mining obsi
            (ore, clay, _, _)
                if ore >= self.obsi_cost.0 && clay >= self.obsi_cost.1 && !self.no_build[OBSI] =>
            {
                potential_next.push(Some(OBSI));
                if self.robots[OBSI] > 0 {
                    potential_next.push(None);
                    no_build[OBSI] = true;
                }
            }
            //Only if clay is cheaper than ore, we build ore or nothing
            (ore, _, _, 0..=CLAY_ORE_MINUTES) if ore >= self.clay_cost && ore >= self.ore_cost => {
                if self.robots[CLAY] < self.max_clay() && !self.no_build[CLAY] {
                    potential_next.push(Some(CLAY));
                }
                if self.robots[ORE] < self.max_ore() && !self.no_build[ORE] {
                    potential_next.push(Some(ORE));
                }
                potential_next.push(None);
                no_build[ORE] = true;
            }
            //Build clay or nothing
            (ore, _, _, 0..=CLAY_ORE_MINUTES)
                if ore >= self.clay_cost
                    && self.robots[CLAY] < self.max_clay()
                    && !self.no_build[CLAY] =>
            {
                potential_next.push(Some(CLAY));
                potential_next.push(None);
                no_build[CLAY] = true;
            }
            //Build ore or nothing
            (ore, _, _, 0..=CLAY_ORE_MINUTES)
                if ore >= self.ore_cost
                    && self.robots[ORE] < self.max_ore()
                    && !self.no_build[ORE] =>
            {
                potential_next.push(Some(ORE));
                potential_next.push(None);
                no_build[ORE] = true;
            }
            //Build nothing
            _ => potential_next.push(None),
        }

        potential_next
            .into_iter()
            .map(|elem| {
                let mut new_state: Factory = *self;
                new_state.gather();
                if let Some(e) = elem {
                    new_state.build(e)
                } else {
                    for (i, &item) in no_build.iter().enumerate() {
                        if !new_state.no_build[i] {
                            new_state.no_build[i] = item;
                        }
                    }
                }
                new_state
            })
            .collect()
    }

    fn gather(&mut self) {
        for i in 0..NB_ELEM {
            self.store[i] += self.robots[i];
        }
    }

    fn build(&mut self, robot: usize) {
        match robot {
            ORE => self.store[ORE] -= self.ore_cost,
            CLAY => self.store[ORE] -= self.clay_cost,
            OBSI => {
                self.store[ORE] -= self.obsi_cost.0;
                self.store[CLAY] -= self.obsi_cost.1;
            }
            GEO => {
                self.store[ORE] -= self.geo_cost.0;
                self.store[OBSI] -= self.geo_cost.1;
            }
            _ => (), //Should not happen
        }
        self.robots[robot] += 1;
        self.no_build = [false; NB_ELEM];
    }

    fn max_ore(&self) -> u16 {
        max(
            max(max(self.ore_cost, self.clay_cost), self.obsi_cost.0),
            self.geo_cost.0,
        )
    }

    fn max_clay(&self) -> u16 {
        self.obsi_cost.1
    }

    fn quality_level(&self) -> u16 {
        self.id * self.store[GEO]
    }
}

impl FromStr for Factory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&[' ', ':']).collect();
        let id: u16 = words[1].parse().unwrap();
        let ore_cost: u16 = words[7].parse().unwrap();
        let clay_cost: u16 = words[13].parse().unwrap();
        let obsi_cost: (u16, u16) = (words[19].parse().unwrap(), words[22].parse().unwrap());
        let geo_cost: (u16, u16) = (words[28].parse().unwrap(), words[31].parse().unwrap());

        let robots: [u16; NB_ELEM] = [1, 0, 0, 0];
        Ok(Factory {
            id,
            ore_cost,
            clay_cost,
            obsi_cost,
            geo_cost,
            no_build: [false; NB_ELEM],
            store: [0; NB_ELEM],
            robots,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_19.txt").expect("Cannot open input file");

    let factories: Vec<Factory> = s.lines().map(|l| l.parse().unwrap()).collect();

    let sum: u16 = factories
        .iter()
        .map(|&f| {
            let mut current: Vec<Factory> = vec![f];

            for t in 0..24 {
                current = current.into_iter().flat_map(|f| f.next_minute(t)).collect();
            }
            let max_quality: u16 = current.iter().map(|f| f.quality_level()).max().unwrap();
            max_quality
        })
        .sum();
    println!("Part1: The sum of all the quality levels is {}", sum);

    let prod: usize = factories
        .iter()
        .take(3)
        .map(|&f| {
            let mut current: Vec<Factory> = vec![f];

            for t in 0..32 {
                current = current.into_iter().flat_map(|f| f.next_minute(t)).collect();
            }
            let max_geode: u16 = current.iter().map(|f| f.store[GEO]).max().unwrap();
            max_geode as usize
        })
        .product();
    println!(
        "Part2: The product of the number of geode opened for the first 3 blueprint is {}",
        prod
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.";

    #[test]
    fn part_1() {
        let factories: Vec<Factory> = INPUT.lines().map(|l| l.parse().unwrap()).collect();

        let sum: u16 = factories
            .iter()
            .map(|&f| {
                let mut current: Vec<Factory> = vec![f];

                for t in 0..24 {
                    current = current.into_iter().flat_map(|f| f.next_minute(t)).collect();
                }
                current.iter().map(|f| f.quality_level()).max().unwrap()
            })
            .sum();
        assert_eq!(sum, 33);
    }

    #[test]
    fn part_2() {
        let factories: Vec<Factory> = INPUT.lines().map(|l| l.parse().unwrap()).collect();
        let max_geodes: Vec<u16> = factories
            .iter()
            .map(|&f| {
                let mut current: Vec<Factory> = vec![f];

                for t in 0..32 {
                    current = current.into_iter().flat_map(|f| f.next_minute(t)).collect();
                }
                current.iter().map(|f| f.store[GEO]).max().unwrap()
            })
            .collect();

        assert_eq!(max_geodes[0], 56);
        assert_eq!(max_geodes[1], 62);
    }
}
