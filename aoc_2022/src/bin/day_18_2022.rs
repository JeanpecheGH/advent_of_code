use std::collections::HashSet;
use std::str::FromStr;
use util::coord::Pos3;

const MAX_COORD: usize = 25;

#[derive(Debug)]
struct Lava {
    droplets: [[[bool; MAX_COORD]; MAX_COORD]; MAX_COORD],
}

impl Lava {
    fn surface(&self) -> usize {
        let mut surface = 0;
        for (i, slice) in self.droplets.iter().enumerate() {
            for (j, row) in slice.iter().enumerate() {
                for (k, &drop) in row.iter().enumerate() {
                    surface += if drop {
                        6 - self.nb_neighbours(Pos3(i, j, k))
                    } else {
                        0
                    }
                }
            }
        }
        surface
    }

    fn outer_surface(&self) -> usize {
        //Fill the structure with steam every where it can access
        let mut steam: HashSet<Pos3> = HashSet::new();
        steam.insert(Pos3(0, 0, 0));

        let mut current: Vec<Pos3> = vec![Pos3(0, 0, 0)];
        while !current.is_empty() {
            current = current
                .into_iter()
                .flat_map(|drop| {
                    let new_drops: Vec<Pos3> = Self::neighbours(drop)
                        .into_iter()
                        .filter(|&Pos3(x, y, z)| !self.droplets[x][y][z])
                        .collect();
                    new_drops
                })
                .filter(|drop| steam.insert(*drop))
                .collect();
        }

        //Count the surface in contact with steam
        //To the number of steam neighbours, we need to add cublets that are outside our structure (and should be steam)
        let mut outer_surface = 0;
        for (i, slice) in self.droplets.iter().enumerate() {
            for (j, row) in slice.iter().enumerate() {
                for (k, &drop) in row.iter().enumerate() {
                    outer_surface += if drop {
                        let ngbs = Self::neighbours(Pos3(i, j, k));
                        let nb_steam = ngbs.iter().filter(|drop| steam.contains(*drop)).count();
                        6 - ngbs.len() + nb_steam
                    } else {
                        0
                    }
                }
            }
        }
        outer_surface
    }

    fn nb_neighbours(&self, drop: Pos3) -> usize {
        let ngbs: Vec<Pos3> = Self::neighbours(drop);
        ngbs.iter()
            .filter(|Pos3(i, j, k)| self.droplets[*i][*j][*k])
            .count()
    }

    fn neighbours(Pos3(x, y, z): Pos3) -> Vec<Pos3> {
        let mut ngbs: Vec<Pos3> = Vec::new();
        if x > 0 {
            ngbs.push(Pos3(x - 1, y, z));
        }
        if x + 1 < MAX_COORD {
            ngbs.push(Pos3(x + 1, y, z));
        }
        if y > 0 {
            ngbs.push(Pos3(x, y - 1, z));
        }
        if y + 1 < MAX_COORD {
            ngbs.push(Pos3(x, y + 1, z));
        }
        if z > 0 {
            ngbs.push(Pos3(x, y, z - 1));
        }
        if z + 1 < MAX_COORD {
            ngbs.push(Pos3(x, y, z + 1));
        }
        ngbs
    }
}

impl FromStr for Lava {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let droplets: Vec<Pos3> = s
            .lines()
            .map(|l| {
                let ns: Vec<&str> = l.split(',').collect();
                let x: usize = ns[0].parse().unwrap();
                let y: usize = ns[1].parse().unwrap();
                let z: usize = ns[2].parse().unwrap();
                Pos3(x, y, z)
            })
            .collect();

        let mut lava: Self = Self {
            droplets: [[[false; MAX_COORD]; MAX_COORD]; MAX_COORD],
        };

        droplets.iter().for_each(|&Pos3(x, y, z)| {
            lava.droplets[x][y][z] = true;
        });
        Ok(lava)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_18.txt").expect("Cannot open input file");

    let lava: Lava = s.parse().unwrap();

    println!("Part1: The surface of the lava is {}", lava.surface());
    println!(
        "Part2: The surface of the lava exposed to steam is {}",
        lava.outer_surface()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5";

    #[test]
    fn part_1() {
        let lava: Lava = INPUT.parse().unwrap();
        assert_eq!(lava.surface(), 64);
    }

    #[test]
    fn part_2() {
        let lava: Lava = INPUT.parse().unwrap();
        assert_eq!(lava.outer_surface(), 58);
    }
}
