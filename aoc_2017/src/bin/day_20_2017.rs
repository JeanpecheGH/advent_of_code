use fxhash::{FxHashMap, FxHashSet};
use nom::bytes::complete::tag;
use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use std::str::FromStr;
use util::basic_parser::parse_isize;
use util::coord::Pos3I;

#[derive(Debug, Copy, Clone)]
struct Particle {
    position: Pos3I,
    velocity: Pos3I,
    acceleration: Pos3I,
}

impl Particle {
    fn abs_acceleration(&self) -> usize {
        self.acceleration.distance(Pos3I(0, 0, 0))
    }
    fn abs_velocity(&self) -> usize {
        self.velocity.distance(Pos3I(0, 0, 0))
    }

    fn tick(&mut self) {
        self.velocity = Pos3I(
            self.velocity.0 + self.acceleration.0,
            self.velocity.1 + self.acceleration.1,
            self.velocity.2 + self.acceleration.2,
        );
        self.position = Pos3I(
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
            self.position.2 + self.velocity.2,
        );
    }
}

impl FromStr for Particle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_triplet(s: &str) -> IResult<&str, Pos3I> {
            let (s, list) = separated_list1(char(','), parse_isize)(s)?;

            Ok((s, Pos3I(list[0], list[1], list[2])))
        }

        fn parse_particle(s: &str) -> IResult<&str, Particle> {
            let (s, position) = preceded(tag("p=<"), parse_triplet)(s)?;
            let (s, velocity) = preceded(tag(">, v=<"), parse_triplet)(s)?;
            let (s, acceleration) = delimited(tag(">, a=<"), parse_triplet, char('>'))(s)?;

            Ok((
                s,
                Particle {
                    position,
                    velocity,
                    acceleration,
                },
            ))
        }

        Ok(parse_particle(s).unwrap().1)
    }
}

struct ParticleSwarm {
    particles: Vec<Particle>,
}

impl ParticleSwarm {
    fn closest_particle(&self) -> usize {
        self.particles
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| {
                a.abs_acceleration()
                    .cmp(&b.abs_acceleration())
                    .then(a.abs_velocity().cmp(&b.abs_velocity()))
            })
            .unwrap()
            .0
    }

    fn surviving_particles(&self) -> usize {
        let mut moving_parts: Vec<(usize, Particle)> =
            self.particles.iter().copied().enumerate().collect();

        //Seems like 50 ticks are enough
        for _ in 0..50 {
            let mut to_delete: FxHashSet<usize> = FxHashSet::default();
            let mut cache: FxHashMap<Pos3I, usize> = FxHashMap::default();

            //Advance all particules
            moving_parts.iter_mut().for_each(|(n, p)| {
                p.tick();
                if let Some(&colide) = cache.get(&p.position) {
                    to_delete.insert(*n);
                    to_delete.insert(colide);
                } else {
                    cache.insert(p.position, *n);
                }
            });

            moving_parts.retain(|(n, _)| !to_delete.contains(n));
        }

        moving_parts.len()
    }
}

impl FromStr for ParticleSwarm {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let particles: Vec<Particle> = s.lines().map(|l| l.parse().unwrap()).collect();

        Ok(ParticleSwarm { particles })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_20.txt").expect("Cannot open input file");
    let swarm: ParticleSwarm = s.parse().unwrap();

    println!(
        "Part1: In the long term, particle {} will stay the closest",
        swarm.closest_particle()
    );
    println!(
        "Part2: After all collisions happened, {} particles are remaining",
        swarm.surviving_particles()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "p=<3,0,0>, v=<2,0,0>, a=<-1,0,0>
p=<4,0,0>, v=<0,0,0>, a=<-2,0,0>
";

    const EXAMPLE_2: &str = "p=<-6,0,0>, v=<3,0,0>, a=<0,0,0>    
p=<-4,0,0>, v=<2,0,0>, a=<0,0,0>
p=<-2,0,0>, v=<1,0,0>, a=<0,0,0>
p=<3,0,0>, v=<-1,0,0>, a=<0,0,0>
";

    #[test]
    fn part_1() {
        let swarm: ParticleSwarm = EXAMPLE_1.parse().unwrap();
        assert_eq!(0, swarm.closest_particle());
    }

    #[test]
    fn part_2() {
        let swarm: ParticleSwarm = EXAMPLE_2.parse().unwrap();
        assert_eq!(1, swarm.surviving_particles());
    }
}
