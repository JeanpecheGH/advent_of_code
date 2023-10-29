use std::cmp::Ordering;
use std::str::FromStr;
use util::coord::{Pos3, Pos3I};

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Moon {
    pos: Pos3I,
    velocity: Pos3I,
}

impl Moon {
    fn total_energy(&self) -> isize {
        self.potential_abs() * self.kinetic_abs()
    }

    fn kinetic_abs(&self) -> isize {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }

    fn potential_abs(&self) -> isize {
        self.pos.0.abs() + self.pos.1.abs() + self.pos.2.abs()
    }

    fn gravity(&mut self, other: &Moon) {
        fn diff(a: isize, b: isize) -> isize {
            match a.cmp(&b) {
                Ordering::Less => -1,
                Ordering::Equal => 0,
                Ordering::Greater => 1,
            }
        }

        let dx: isize = diff(other.pos.0, self.pos.0);
        let dy: isize = diff(other.pos.1, self.pos.1);
        let dz: isize = diff(other.pos.2, self.pos.2);

        self.velocity = (
            self.velocity.0 + dx,
            self.velocity.1 + dy,
            self.velocity.2 + dz,
        );
    }

    fn velocity(&mut self) {
        self.pos = (
            self.pos.0 + self.velocity.0,
            self.pos.1 + self.velocity.1,
            self.pos.2 + self.velocity.2,
        );
    }
}

impl FromStr for Moon {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&['=', ',', '>']).collect();
        let x: isize = words[1].parse().unwrap();
        let y: isize = words[3].parse().unwrap();
        let z: isize = words[5].parse().unwrap();
        let pos: Pos3I = (x, y, z);
        let velocity: Pos3I = (0, 0, 0);
        Ok(Self { pos, velocity })
    }
}

#[derive(Debug, Clone)]
struct System {
    moons: Vec<Moon>,
}

impl System {
    fn total_energy(&self) -> isize {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }

    fn step(&mut self) {
        let moon_clone: Vec<Moon> = self.moons.clone();
        self.moons
            .iter_mut()
            .for_each(|moon| moon_clone.iter().for_each(|other| moon.gravity(other)));
        self.moons.iter_mut().for_each(|moon| moon.velocity());
    }

    fn steps(&mut self, n: usize) {
        for _ in 0..n {
            self.step();
        }
    }

    fn step_loop(&mut self) -> usize {
        let start_pos_x: Vec<isize> = self.moons.iter().map(|moon| moon.pos.0).collect();
        let start_pos_y: Vec<isize> = self.moons.iter().map(|moon| moon.pos.1).collect();
        let start_pos_z: Vec<isize> = self.moons.iter().map(|moon| moon.pos.2).collect();
        let start_vel_x: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.0).collect();
        let start_vel_y: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.1).collect();
        let start_vel_z: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.2).collect();

        let (mut x_cycle, mut y_cycle, mut z_cycle): Pos3 = (0, 0, 0);

        let mut i: usize = 0;
        loop {
            let current_pos_x: Vec<isize> = self.moons.iter().map(|moon| moon.pos.0).collect();
            let current_pos_y: Vec<isize> = self.moons.iter().map(|moon| moon.pos.1).collect();
            let current_pos_z: Vec<isize> = self.moons.iter().map(|moon| moon.pos.2).collect();
            let current_vel_x: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.0).collect();
            let current_vel_y: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.1).collect();
            let current_vel_z: Vec<isize> = self.moons.iter().map(|moon| moon.velocity.2).collect();
            if x_cycle == 0 && start_pos_x == current_pos_x && start_vel_x == current_vel_x {
                x_cycle = i;
            }
            if y_cycle == 0 && start_pos_y == current_pos_y && start_vel_y == current_vel_y {
                y_cycle = i;
            }
            if z_cycle == 0 && start_pos_z == current_pos_z && start_vel_z == current_vel_z {
                z_cycle = i;
            }
            if x_cycle != 0 && y_cycle != 0 && z_cycle != 0 {
                let cycle: usize = util::lcm(
                    x_cycle as isize,
                    util::lcm(y_cycle as isize, z_cycle as isize),
                ) as usize;
                return cycle;
            }
            self.step();
            i += 1;
        }
    }
}

impl FromStr for System {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moons: Vec<Moon> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { moons })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_12.txt").expect("Cannot open input file");
    let mut system: System = s.parse().unwrap();
    system.steps(1000);
    println!(
        "Part1: After 1000 steps, the total energy of the system is {}",
        system.total_energy()
    );
    println!(
        "Part2: After {} steps, the system will be back to its original state",
        system.step_loop()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "<x=-1, y=0, z=2>
<x=2, y=-10, z=-7>
<x=4, y=-8, z=8>
<x=3, y=5, z=-1>";

    const INPUT_2: &str = "<x=-8, y=-10, z=0>
<x=5, y=5, z=10>
<x=2, y=-7, z=3>
<x=9, y=-8, z=-3>";

    #[test]
    fn test_1_part_1() {
        let mut system: System = INPUT_1.parse().unwrap();
        let mut system_2: System = system.clone();
        system.steps(10);
        assert_eq!(system.total_energy(), 179);
        assert_eq!(system_2.step_loop(), 2772);
    }
    #[test]
    fn test_2_part_1() {
        let mut system: System = INPUT_2.parse().unwrap();
        let mut system_2: System = system.clone();
        system.steps(100);
        assert_eq!(system.total_energy(), 1940);
        assert_eq!(system_2.step_loop(), 4686774924);
    }
}
