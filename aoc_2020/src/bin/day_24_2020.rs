use std::collections::HashSet;
use std::str::FromStr;
use util::coord::PosI;

const DAYS: usize = 100;

#[derive(Debug, Hash, Eq, PartialEq, Copy, Clone)]
struct Tile {
    coords: PosI,
}

impl Tile {
    fn neighbours(&self) -> Vec<Tile> {
        let moves: Vec<PosI> = vec![
            PosI(0, 1),
            PosI(1, 1),
            PosI(-1, 0),
            PosI(1, 0),
            PosI(-1, -1),
            PosI(0, -1),
        ];
        moves
            .into_iter()
            .map(|PosI(x, y)| {
                let coords: PosI = PosI(x + self.coords.0, y + self.coords.1);
                Tile { coords }
            })
            .collect()
    }
}

impl FromStr for Tile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut prev: char = 'a';
        for c in s.chars() {
            match (c, prev) {
                ('w', 'n') => y += 1,
                ('e', 'n') => {
                    x += 1;
                    y += 1
                }
                ('w', 's') => {
                    x -= 1;
                    y -= 1;
                }
                ('e', 's') => y -= 1,
                ('w', _) => x -= 1,
                ('e', _) => x += 1,
                _ => (),
            }
            prev = c;
        }

        Ok(Tile { coords: PosI(x, y) })
    }
}

struct Floor {
    flipped: HashSet<Tile>,
}

impl Floor {
    fn nb_black(&self) -> usize {
        self.flipped.len()
    }

    fn days(&mut self, days: usize) {
        for _ in 0..days {
            self.day();
        }
    }

    fn day(&mut self) {
        //The whites are all the neighbours of the blacks minus the black themselves
        let mut whites: HashSet<Tile> = self
            .flipped
            .iter()
            .flat_map(|black| black.neighbours())
            .collect();
        whites = whites.difference(&self.flipped).cloned().collect();

        //Keep black tiles that are not flipped
        let stays_black: HashSet<Tile> = self
            .flipped
            .iter()
            .filter(|tile| {
                let nb_white_ngb: usize = tile
                    .neighbours()
                    .iter()
                    .filter(|tile| whites.contains(tile))
                    .count();
                nb_white_ngb == 4 || nb_white_ngb == 5
            })
            .cloned()
            .collect();

        //Add flipping white tiles
        let new_black: HashSet<Tile> = whites
            .iter()
            .filter(|tile| {
                let nb_black_ngb: usize = tile
                    .neighbours()
                    .iter()
                    .filter(|tile| self.flipped.contains(tile))
                    .count();
                nb_black_ngb == 2
            })
            .cloned()
            .collect();

        self.flipped = stays_black.union(&new_black).cloned().collect();
    }
}

impl FromStr for Floor {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut flipped: HashSet<Tile> = HashSet::new();
        s.lines()
            .map(|l| l.parse().unwrap())
            .for_each(|tile: Tile| {
                if !flipped.insert(tile) {
                    flipped.remove(&tile);
                }
            });

        Ok(Floor { flipped })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_24.txt").expect("Cannot open input file");
    let mut floor: Floor = s.parse().unwrap();
    println!(
        "Part1: There are {} black tiles on the floor",
        floor.nb_black()
    );
    floor.days(DAYS);
    println!(
        "Part2: After {} days, there are {} black tiles on the floor",
        DAYS,
        floor.nb_black()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";

    #[test]
    fn test_tile_1() {
        let tile: Tile = "esew".parse().unwrap();
        assert_eq!(tile.coords, PosI(0, -1));
    }
    #[test]
    fn test_tile_2() {
        let tile: Tile = "nwwswee".parse().unwrap();
        assert_eq!(tile.coords, PosI(0, 0));
    }

    #[test]
    fn test_part_1() {
        let floor: Floor = INPUT.parse().unwrap();
        assert_eq!(floor.nb_black(), 10);
    }

    #[test]
    fn test_part_2() {
        let mut floor: Floor = INPUT.parse().unwrap();
        floor.days(DAYS);
        assert_eq!(floor.nb_black(), 2208);
    }
}
