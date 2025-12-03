use fxhash::FxHashSet;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::character::complete::{anychar, char};
use nom::combinator::map;
use nom::sequence::{separated_pair, terminated};
use nom::IResult;
use nom::Parser;
use std::cmp::{max, min};
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Clay,
    DrySand,
    WetSand,
    Water,
}

impl Tile {
    fn is_blocking(&self) -> bool {
        match self {
            Tile::Clay | Tile::Water => true,
            Tile::DrySand | Tile::WetSand => false,
        }
    }
}

#[derive(Debug, Copy, Clone)]
struct ClayVein {
    vert: bool,
    spot: usize,
    start: usize,
    end: usize,
}

impl FromStr for ClayVein {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_vein(s: &str) -> IResult<&str, ClayVein> {
            let (s, vert) = map(terminated(anychar, char('=')), |c| c == 'x').parse(s)?;
            let (s, spot) = parse_usize(s)?;
            let (s, _) = alt((tag(", x="), tag(", y="))).parse(s)?;
            let (s, (start, end)) = separated_pair(parse_usize, tag(".."), parse_usize).parse(s)?;

            Ok((
                s,
                ClayVein {
                    vert,
                    spot,
                    start,
                    end,
                },
            ))
        }

        Ok(parse_vein(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct Reservoir {
    grid: Vec<Vec<Tile>>,
    min_x: usize,
    min_y: usize,
    max_y: usize,
}

impl Reservoir {
    fn flood(&mut self) -> (usize, usize) {
        let source_y: usize = 500 - self.min_x;
        let mut flow: Vec<Pos> = vec![Pos(source_y, 0)];
        let mut source_set: FxHashSet<Pos> = FxHashSet::default();

        while let Some(p) = flow.pop() {
            let (new_sources, side_spill) = self.flood_row(p);
            for new_p in new_sources {
                if !side_spill || source_set.insert(new_p) {
                    flow.push(new_p);
                }
            }
        }
        self.nb_wet_and_water_tiles()
    }

    fn flood_sideways(&self, Pos(x, y): Pos, left: bool) -> (usize, bool) {
        let offset: isize = if left { -1 } else { 1 };
        let mut limit: isize = x as isize;

        while self.grid[y + 1][limit as usize].is_blocking()
            && !self.grid[y][(limit + offset) as usize].is_blocking()
        {
            limit += offset;
        }

        let stop: bool = (limit == 0
            || limit == (self.grid[0].len() - 1) as isize
            || self.grid[y][(limit + offset) as usize].is_blocking())
            && self.grid[y + 1][limit as usize].is_blocking();

        (limit as usize, stop)
    }

    fn flood_row(&mut self, Pos(x, y): Pos) -> (Vec<Pos>, bool) {
        let mut ret: Vec<Pos> = Vec::new();
        let mut side_spill: bool = true;

        if y >= (self.max_y - self.min_y) {
            //We are too deep, stop here
            self.grid[y][x] = Tile::WetSand;
        } else if self.grid[y + 1][x].is_blocking() {
            let (left, stop_left) = self.flood_sideways(Pos(x, y), true);
            let (right, stop_right) = self.flood_sideways(Pos(x, y), false);

            let to_fill: Tile = match (stop_left, stop_right) {
                (true, true) => {
                    ret.push(Pos(x, y - 1));
                    side_spill = false;
                    Tile::Water
                }
                (false, true) => {
                    ret.push(Pos(left, y));
                    Tile::WetSand
                }
                (true, false) => {
                    ret.push(Pos(right, y));
                    Tile::WetSand
                }
                (false, false) => {
                    ret.push(Pos(left, y));
                    ret.push(Pos(right, y));
                    Tile::WetSand
                }
            };
            for i in left..=right {
                self.grid[y][i] = to_fill;
            }
        } else {
            self.grid[y][x] = Tile::WetSand;
            side_spill = false;
            ret.push(Pos(x, y + 1))
        }
        (ret, side_spill)
    }

    fn nb_wet_and_water_tiles(&self) -> (usize, usize) {
        self.grid.iter().fold((0, 0), |(wet, water), row| {
            let (wet_row, water_row): (usize, usize) =
                row.iter()
                    .fold((0, 0), |(wet_r, water_r), tile| match tile {
                        Tile::Clay | Tile::DrySand => (wet_r, water_r),
                        Tile::WetSand => (wet_r + 1, water_r),
                        Tile::Water => (wet_r + 1, water_r + 1),
                    });
            (wet + wet_row, water + water_row)
        })
    }

    #[allow(dead_code)]
    fn print(&self) {
        for row in &self.grid {
            for t in row {
                let c: char = match t {
                    Tile::Clay => '#',
                    Tile::DrySand => '.',
                    Tile::WetSand => '|',
                    Tile::Water => '~',
                };
                print!("{c}");
            }

            println!();
        }
    }
}

impl FromStr for Reservoir {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let veins: Vec<ClayVein> = s.lines().map(|l| l.parse().unwrap()).collect();

        //Compute limits of the grid
        let (min_x, max_x, min_y, max_y): (usize, usize, usize, usize) = veins.iter().fold(
            (usize::MAX, 0, usize::MAX, 0),
            |(mut min_x, mut max_x, mut min_y, mut max_y), vein| {
                if vein.vert {
                    min_x = min(min_x, vein.spot);
                    max_x = max(max_x, vein.spot);
                    min_y = min(min_y, vein.start);
                    max_y = max(max_y, vein.end);
                } else {
                    min_x = min(min_x, vein.start);
                    max_x = max(max_x, vein.end);
                    min_y = min(min_y, vein.spot);
                    max_y = max(max_y, vein.spot);
                }

                (min_x, max_x, min_y, max_y)
            },
        );

        let min_x = min_x - 1;
        let max_x = max_x + 1;

        let x_range: usize = max_x - min_x + 1;
        let y_range: usize = max_y - min_y + 1;

        //Build grid from the clay veins
        let mut grid: Vec<Vec<Tile>> = vec![vec![Tile::DrySand; x_range]; y_range];

        for vein in &veins {
            if vein.vert {
                for y in vein.start..=vein.end {
                    grid[y - min_y][vein.spot - min_x] = Tile::Clay
                }
            } else {
                for x in vein.start..=vein.end {
                    grid[vein.spot - min_y][x - min_x] = Tile::Clay
                }
            }
        }

        Ok(Reservoir {
            grid,
            min_x,
            min_y,
            max_y,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_17.txt").expect("Cannot open input file");
    let mut reservoir: Reservoir = s.parse().unwrap();
    let (wet, water) = reservoir.flood();

    println!("Part1: Water reaches {} tiles", wet);
    println!("Part2: There is resting water in {} tiles", water);
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "x=495, y=2..7
y=7, x=495..501
x=501, y=3..7
x=498, y=2..4
x=506, y=1..2
x=498, y=10..13
x=504, y=10..13
y=13, x=498..504
";

    #[test]
    fn part_1() {
        let mut reservoir: Reservoir = EXAMPLE_1.parse().unwrap();

        assert_eq!(reservoir.flood(), (57, 29));
    }
}
