use std::collections::HashMap;

#[derive(Debug)]
struct Reindeer {
    speed: u32,
    period: u32,
    rest: u32,
}

impl Reindeer {
    fn distance(&self, time: u32) -> u32 {
        let cycle = self.period + self.rest;
        let mut running_time = (time / cycle) * self.period;
        let r = time % cycle;
        running_time += if r > self.period { self.period } else { r };
        running_time * self.speed
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_14.txt").expect("Cannot open input file");

    let reindeers: Vec<Reindeer> = lines
        .map(|l| {
            let s: String = l.unwrap();
            let words: Vec<&str> = s.split(' ').collect();
            let speed: u32 = words[3].parse().unwrap();
            let period: u32 = words[6].parse().unwrap();
            let rest: u32 = words[13].parse().unwrap();
            Reindeer {
                speed,
                period,
                rest,
            }
        })
        .collect();

    let race_duration: u32 = 2503;
    let max_dist: Option<u32> = reindeers.iter().map(|r| r.distance(race_duration)).max();
    println!(
        "Part1: The fastest Reindeer covered {} kms",
        max_dist.unwrap()
    );

    let reindeers_points: HashMap<usize, u32> = (1..=race_duration)
        .map(|t| {
            //Getting the id of the winning reindeers every second
            reindeers.iter().map(|r| r.distance(t)).enumerate().fold(
                (Vec::new(), 0),
                |(mut acc, max_dist), (i, dist)| match (max_dist, dist) {
                    (a, b) if b > a => {
                        acc.clear();
                        acc.push(i);
                        (acc, b)
                    }
                    (a, b) if b == a => {
                        acc.push(i);
                        (acc, a)
                    }
                    _ => (acc, max_dist),
                },
            )
        })
        //Summing the points for each reindeer
        .fold(HashMap::new(), |mut acc, (v, _)| {
            v.iter().for_each(|&n| {
                let entry = acc.entry(n).or_insert(0);
                *entry += 1;
            });
            acc
        });
    println!(
        "Part2: The best Reindeer gained {:?} points",
        reindeers_points.values().max().unwrap()
    );
}
