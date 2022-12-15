use std::str::FromStr;

type Pos = (isize, isize);

#[derive(Debug)]
struct Sensor {
    pos: Pos,
    beacon: Pos,
}

impl FromStr for Sensor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let x_sensor: isize = words[2]
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse()
            .unwrap();
        let y_sensor: isize = words[3]
            .strip_prefix("y=")
            .unwrap()
            .strip_suffix(':')
            .unwrap()
            .parse()
            .unwrap();
        let x_beacon: isize = words[8]
            .strip_prefix("x=")
            .unwrap()
            .strip_suffix(',')
            .unwrap()
            .parse()
            .unwrap();
        let y_beacon: isize = words[9].strip_prefix("y=").unwrap().parse().unwrap();
        Ok(Self {
            pos: (x_sensor, y_sensor),
            beacon: (x_beacon, y_beacon),
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_15.txt").expect("Cannot open input file");

    let beacons: Vec<Sensor> = s.lines().map(|l| l.parse().unwrap()).collect();

    dbg!(beacons);
}
