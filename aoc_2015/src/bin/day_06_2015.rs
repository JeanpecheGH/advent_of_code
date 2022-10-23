use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
enum Action {
    TurnOff,
    Toggle,
    TurnOn,
}

impl FromStr for Action {
    type Err = ();

    fn from_str(input: &str) -> Result<Action, Self::Err> {
        match input {
            "turn off" => Ok(Action::TurnOff),
            "toggle" => Ok(Action::Toggle),
            "turn on" => Ok(Action::TurnOn),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct ActionZone {
    action: Action,
    start: (usize, usize),
    end: (usize, usize),
}

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_06.txt").expect("Cannot open input file");

    let re = Regex::new(r"^([a-z ]*) (\d{1,3}),(\d{1,3}) through (\d{1,3}),(\d{1,3})$").unwrap();

    let actions: Vec<ActionZone> = lines
        .filter_map(|l| {
            if let Some(cap) = re.captures_iter(l.unwrap().as_str()).next() {
                let action = Action::from_str(&cap[1]).unwrap();
                let start: (usize, usize) = (
                    cap[2].parse::<usize>().unwrap(),
                    cap[3].parse::<usize>().unwrap(),
                );
                let end: (usize, usize) = (
                    cap[4].parse::<usize>().unwrap(),
                    cap[5].parse::<usize>().unwrap(),
                );
                Some(ActionZone { action, start, end })
            } else {
                None
            }
        })
        .collect();

    //Part 1
    let mut grid: [[bool; 1000]; 1000] = [[false; 1000]; 1000];

    actions.iter().for_each(|ac| match ac.action {
        Action::TurnOff => {
            for row in grid.iter_mut().take(ac.end.0 + 1).skip(ac.start.0) {
                for item in row.iter_mut().take(ac.end.1 + 1).skip(ac.start.1) {
                    *item = false
                }
            }
        }
        Action::Toggle => {
            for row in grid.iter_mut().take(ac.end.0 + 1).skip(ac.start.0) {
                for item in row.iter_mut().take(ac.end.1 + 1).skip(ac.start.1) {
                    *item = !*item
                }
            }
        }
        Action::TurnOn => {
            for row in grid.iter_mut().take(ac.end.0 + 1).skip(ac.start.0) {
                for item in row.iter_mut().take(ac.end.1 + 1).skip(ac.start.1) {
                    *item = true
                }
            }
        }
    });

    let nb_lights: usize = grid
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum();

    println!("Part1: {} lights are on", nb_lights);

    //Part 2
    let mut grid: [[i32; 1000]; 1000] = [[0; 1000]; 1000];

    actions.iter().for_each(|ac| {
        let add: i32 = match ac.action {
            Action::TurnOff => -1,
            Action::Toggle => 2,
            Action::TurnOn => 1,
        };
        for row in grid.iter_mut().take(ac.end.0 + 1).skip(ac.start.0) {
            for item in row.iter_mut().take(ac.end.1 + 1).skip(ac.start.1) {
                let res: i32 = *item + add;
                *item = if res > 0 { res } else { 0 }
            }
        }
    });

    let brightness: i32 = grid.iter().map(|row| row.iter().sum::<i32>()).sum();

    println!("Part2: Total brightness is {}", brightness);
}
