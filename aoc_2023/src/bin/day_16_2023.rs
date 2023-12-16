use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::HashSet;
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum Tile {
    Empty,
    HorizontalSplit,
    VerticalSplit,
    Mirror,
    AntiMirror,
}

impl Tile {
    fn from_char(c: char) -> Tile {
        match c {
            '-' => Tile::HorizontalSplit,
            '|' => Tile::VerticalSplit,
            '/' => Tile::Mirror,
            '\\' => Tile::AntiMirror,
            _ => Tile::Empty,
        }
    }
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Beam {
    pos: Pos,
    dir: Dir,
}

impl Beam {
    fn new(pos: Pos, dir: Dir) -> Beam {
        Beam { pos, dir }
    }
    fn encounters(&self, t: Tile) -> (Beam, Option<Beam>) {
        match (t, self.dir) {
            //Horizontal splits
            (Tile::HorizontalSplit, Dir::North | Dir::South) => (
                Beam::new(self.pos, Dir::West),
                Some(Beam::new(self.pos, Dir::East)),
            ),
            //Vertical splits
            (Tile::VerticalSplit, Dir::West | Dir::East) => (
                Beam::new(self.pos, Dir::North),
                Some(Beam::new(self.pos, Dir::South)),
            ),
            //Mirrors
            (Tile::Mirror, Dir::North) => (Beam::new(self.pos, Dir::East), None),
            (Tile::Mirror, Dir::West) => (Beam::new(self.pos, Dir::South), None),
            (Tile::Mirror, Dir::South) => (Beam::new(self.pos, Dir::West), None),
            (Tile::Mirror, Dir::East) => (Beam::new(self.pos, Dir::North), None),
            //Anti-mirrors
            (Tile::AntiMirror, Dir::North) => (Beam::new(self.pos, Dir::West), None),
            (Tile::AntiMirror, Dir::West) => (Beam::new(self.pos, Dir::North), None),
            (Tile::AntiMirror, Dir::South) => (Beam::new(self.pos, Dir::East), None),
            (Tile::AntiMirror, Dir::East) => (Beam::new(self.pos, Dir::South), None),
            //Empty or pass through split
            (_, _) => (Beam::new(self.pos, self.dir), None),
        }
    }

    fn advances(&self, max_x: usize, max_y: usize) -> Option<Beam> {
        match (self.pos, self.dir) {
            (Pos(0, _), Dir::West) => None,
            (Pos(_, 0), Dir::North) => None,
            (Pos(x, _), Dir::East) if x >= max_x - 1 => None,

            (Pos(_, y), Dir::South) if y >= max_y - 1 => None,
            (Pos(x, y), d @ Dir::North) => Some(Beam::new(Pos(x, y - 1), d)),
            (Pos(x, y), d @ Dir::West) => Some(Beam::new(Pos(x - 1, y), d)),
            (Pos(x, y), d @ Dir::South) => Some(Beam::new(Pos(x, y + 1), d)),
            (Pos(x, y), d @ Dir::East) => Some(Beam::new(Pos(x + 1, y), d)),
        }
    }
}

#[derive(Clone, Eq, Debug, PartialEq, Hash)]
struct BeamCave {
    grid: Vec<Vec<Tile>>,
}

impl BeamCave {
    fn nb_energized_top_left(&self) -> usize {
        self.nb_energized(Pos(0, 0), Dir::East)
    }
    fn tile_at(&self, Pos(x, y): Pos) -> Tile {
        self.grid[y][x]
    }

    fn best_energized(&self) -> usize {
        let w: usize = self.grid[0].len();
        let h: usize = self.grid.len();

        let mut top: Vec<(Pos, Dir)> = (0..w).map(|x| (Pos(x, 0), Dir::South)).collect();
        let bottom: Vec<(Pos, Dir)> = (0..w).map(|x| (Pos(x, h - 1), Dir::North)).collect();
        let left: Vec<(Pos, Dir)> = (0..h).map(|y| (Pos(0, y), Dir::East)).collect();
        let right: Vec<(Pos, Dir)> = (0..w).map(|y| (Pos(w - 1, y), Dir::West)).collect();

        top.extend(bottom);
        top.extend(left);
        top.extend(right);

        top.into_par_iter()
            .map(|(pos, dir)| self.nb_energized(pos, dir))
            .max()
            .unwrap()
    }
    fn nb_energized(&self, start_pos: Pos, start_dir: Dir) -> usize {
        let mut energized_set: HashSet<Beam> = HashSet::new();

        let mut beams: Vec<Beam> = vec![];
        let start_beam: Beam = Beam::new(start_pos, start_dir);

        let (first_beam, second_beam) = start_beam.encounters(self.tile_at(start_pos));
        if let Some(s) = second_beam {
            beams.push(s);
        }
        beams.push(first_beam);

        while let Some(b) = beams.pop() {
            //Do not treat a beam going in the same direction and in the same place twice
            if energized_set.insert(b) {
                if let Some(beam) = b.advances(self.grid[0].len(), self.grid.len()) {
                    let (beam, second_beam) = beam.encounters(self.tile_at(beam.pos));

                    if let Some(s) = second_beam {
                        beams.push(s)
                    }
                    beams.push(beam);
                }
            }
        }

        //We now remove the Pos duplicates in the set
        let deduplicated_set: HashSet<Pos> =
            energized_set.into_iter().map(|beam| beam.pos).collect();

        deduplicated_set.len()
    }
}

impl FromStr for BeamCave {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Tile>> = s
            .lines()
            .map(|l| l.chars().map(Tile::from_char).collect::<Vec<Tile>>())
            .collect();
        Ok(BeamCave { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_16.txt").expect("Cannot open input file");
    let cave: BeamCave = s.parse().unwrap();
    println!(
        "Part1: When entering from top-left, the beam energizes {} tiles",
        cave.nb_energized_top_left()
    );
    println!(
        "Part2: The maximum number of tiles we can energize at once is {}",
        cave.best_energized()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = r#".|...\....
|.-.\.....
.....|-...
........|.
..........
.........\
..../.\\..
.-.-/..|..
.|....-|.\
..//.|....
"#;

    #[test]
    fn part_1() {
        let cave: BeamCave = EXAMPLE_1.parse().unwrap();
        assert_eq!(cave.nb_energized_top_left(), 46);
    }
    #[test]
    fn part_2() {
        let cave: BeamCave = EXAMPLE_1.parse().unwrap();
        assert_eq!(cave.best_energized(), 51);
    }
}
