use std::ops::{Range, RangeInclusive};
use std::str::FromStr;
use util::basic_parser::parse_pos;
use util::coord::Pos;

#[derive(Copy, Clone, Debug)]
struct PolygonSide {
    vertical: bool,
    rank: usize,
    range: (usize, usize),
}

impl PolygonSide {
    fn from_pair(Pos(x_a, y_a): Pos, Pos(x_b, y_b): Pos) -> PolygonSide {
        if x_a == x_b {
            let range = if y_a > y_b { (y_b, y_a) } else { (y_a, y_b) };
            PolygonSide {
                vertical: true,
                rank: x_a,
                range,
            }
        } else {
            let range = if x_a > x_b { (x_b, x_a) } else { (x_a, x_b) };
            PolygonSide {
                vertical: false,
                rank: y_a,
                range,
            }
        }
    }

    fn full_range(&self) -> RangeInclusive<usize> {
        self.range.0..=self.range.1
    }

    fn semi_range(&self) -> Range<usize> {
        self.range.0..self.range.1
    }

    fn exclusive_range(&self) -> RangeInclusive<usize> {
        (self.range.0 + 1)..=(self.range.1 - 1)
    }

    fn contains(&self, Pos(x, y): Pos) -> bool {
        if self.vertical {
            x == self.rank && self.full_range().contains(&y)
        } else {
            y == self.rank && self.full_range().contains(&x)
        }
    }

    fn is_before(&self, Pos(x, y): Pos) -> bool {
        if self.vertical {
            x > self.rank && self.semi_range().contains(&y)
        } else {
            y > self.rank && self.semi_range().contains(&x)
        }
    }

    fn crosses(&self, other: PolygonSide) -> bool {
        // The sides have to cross anywhere but on either of the extremities of OTHER
        other.exclusive_range().contains(&self.rank) && self.full_range().contains(&other.rank)
    }
}

struct MovieTheater {
    tiles: Vec<Pos>,
}

impl MovieTheater {
    fn largest_rectangles(&self) -> (usize, usize) {
        fn is_inside(pos: Pos, rows: &[PolygonSide], cols: &[PolygonSide]) -> bool {
            let mut cross_rows: usize = 0;
            let mut cross_cols: usize = 0;

            for r in rows {
                if r.contains(pos) {
                    return true;
                } else if r.is_before(pos) {
                    cross_rows += 1;
                }
            }

            for c in cols {
                if c.contains(pos) {
                    return true;
                } else if c.is_before(pos) {
                    cross_cols += 1;
                }
            }
            // println!(
            //     "### {pos:?} crosses {} col and {} rows",
            //     cross_cols, cross_rows
            // );
            cross_cols % 2 == 1 && cross_rows % 2 == 1
        }

        fn crosses_side(side: PolygonSide, rows: &[PolygonSide], cols: &[PolygonSide]) -> bool {
            if side.vertical {
                rows.iter().any(|r| r.crosses(side))
            } else {
                cols.iter().any(|c| c.crosses(side))
            }
        }

        fn are_all_contained(
            side: PolygonSide,
            rows: &[PolygonSide],
            cols: &[PolygonSide],
        ) -> bool {
            if side.vertical {
                side.exclusive_range().all(|y| {
                    let b = is_inside(Pos(side.rank, y), rows, cols);
                    // if !b {
                    //     println!("{:?} is not inside", Pos(side.rank, y));
                    // }
                    b
                })
            } else {
                side.exclusive_range().all(|x| {
                    let b = is_inside(Pos(x, side.rank), rows, cols);

                    // if !b {
                    //     println!("{:?} is not inside", Pos(x, side.rank));
                    // }
                    b
                })
            }
        }

        let mut largest_rectangle = 0;
        let mut largest_tiled_rectangle = 0;

        // Build polygon sides
        let mut rows: Vec<PolygonSide> = Vec::new();
        let mut cols: Vec<PolygonSide> = Vec::new();
        let mut tiles_looped: Vec<Pos> = self.tiles.clone();
        tiles_looped.push(self.tiles[0]);
        tiles_looped.windows(2).for_each(|win| {
            let side: PolygonSide = PolygonSide::from_pair(win[0], win[1]);
            if side.vertical {
                cols.push(side);
            } else {
                rows.push(side);
            }
        });

        for (i, &tile) in self.tiles.iter().enumerate() {
            for j in (i + 1)..self.tiles.len() {
                let other: Pos = self.tiles[j];
                let area: usize = (tile.0.abs_diff(other.0) + 1) * (tile.1.abs_diff(other.1) + 1);
                if area > largest_rectangle {
                    largest_rectangle = area;
                }

                // Check if rectangle is tiled
                // First check both other corners are inside
                let a: Pos = Pos(tile.0, other.1);
                let b: Pos = Pos(other.0, tile.1);

                // Check no polygon side cross any of the 4 rectangle side
                let sides: Vec<PolygonSide> = vec![
                    PolygonSide::from_pair(tile, a),
                    PolygonSide::from_pair(tile, b),
                    PolygonSide::from_pair(other, a),
                    PolygonSide::from_pair(other, b),
                ];

                // if is_inside(a, &rows, &cols)
                //     && is_inside(b, &rows, &cols)
                //     && sides.iter().all(|&side| !crosses_side(side, &rows, &cols))
                // {
                //     println!("{tile:?} {other:?} is valid, area {area}")
                // } else {
                //     println!("{tile:?} {other:?} is NOT VALID, area {area}")
                // }

                if area > largest_tiled_rectangle
                    && is_inside(a, &rows, &cols)
                    && is_inside(b, &rows, &cols)
                    //&& sides.iter().all(|&side| !crosses_side(side, &rows, &cols))
                    && sides.iter().all(|&side| are_all_contained(side, &rows, &cols))
                {
                    largest_tiled_rectangle = area;
                }
            }
        }

        (largest_rectangle, largest_tiled_rectangle)
    }
}

impl FromStr for MovieTheater {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tiles: Vec<Pos> = s.lines().map(|l| parse_pos(l).unwrap().1).collect();

        Ok(MovieTheater { tiles })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_09.txt").expect("Cannot open input file");
    let theater: MovieTheater = s.parse().unwrap();
    let (largest_rectangle, largest_tiled_rectangle) = theater.largest_rectangles();
    println!("Part1: {}", largest_rectangle);
    println!("Part2: {}", largest_tiled_rectangle);
    //1603886397 too high
    //172842768 too low
    //1452422268
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3
";
    const EXAMPLE_2: &str = "7,1
11,1
11,7
9,7
9,5
5,5
5,7
2,7
2,3
7,3
";
    #[test]
    fn test_1() {
        let theater: MovieTheater = EXAMPLE_1.parse().unwrap();
        assert_eq!(theater.largest_rectangles(), (50, 24));
    }
}
