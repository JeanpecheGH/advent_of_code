use std::collections::HashMap;
use std::str::FromStr;
use util::coord::PosI;

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
struct Slope {
    x: isize,
    y: isize,
}

impl Slope {
    fn from_pos(orig: PosI, target: PosI) -> Self {
        let x: isize = target.0 - orig.0;
        let y: isize = target.1 - orig.1;

        //Special case for verticals and horizontals
        if x == 0 {
            let y = y / y.abs();
            return Self { x, y };
        }
        if y == 0 {
            let x = x / x.abs();
            return Self { x, y };
        }

        let gcd: isize = util::gcd(x, y);
        Self {
            x: x / gcd,
            y: y / gcd,
        }
    }

    fn score(&self) -> f64 {
        let y = -self.y as f64;
        let x = self.x as f64;

        match (x, y) {
            (0f64, b) => b * 50f64,
            (a, b) if a > 0f64 => b / a,
            (a, b) => (b / a) - 50f64,
        }
    }
}

struct AsteroidField {
    asteroids: Vec<PosI>,
}

impl AsteroidField {
    fn responses(&self) -> (usize, usize) {
        let (nb_ast, mut vec) = self.best_asteroid();
        vec.sort_by(|(a, _), (b, _)| a.score().total_cmp(&b.score()));
        vec.reverse();
        let pos: PosI = vec[199].1;
        (nb_ast, (pos.0 * 100 + pos.1) as usize)
    }

    fn best_asteroid(&self) -> (usize, Vec<(Slope, PosI)>) {
        fn dist(a: &PosI, b: &PosI) -> usize {
            a.0.abs_diff(b.0) + a.1.abs_diff(b.1)
        }

        self.asteroids
            .iter()
            .map(|ast| {
                let mut slope_map: HashMap<Slope, PosI> = HashMap::new();
                self.asteroids.iter().filter(|p| *ast != **p).for_each(|p| {
                    let s: Slope = Slope::from_pos(*ast, *p);
                    let e = slope_map.entry(s).or_insert(*p);
                    if dist(ast, p) < dist(ast, e) {
                        *e = *p;
                    }
                });
                let slopes: Vec<(Slope, PosI)> = slope_map.into_iter().collect();
                (slopes.len(), slopes)
            })
            .max_by(|(a, _), (b, _)| a.cmp(b))
            .unwrap()
    }
}

impl FromStr for AsteroidField {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let asteroids: Vec<PosI> = s
            .lines()
            .enumerate()
            .flat_map(|(j, row)| {
                row.chars()
                    .enumerate()
                    .filter_map(|(i, c)| {
                        if c == '#' {
                            Some(PosI(i as isize, j as isize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<PosI>>()
            })
            .collect();
        Ok(Self { asteroids })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_10.txt").expect("Cannot open input file");
    let field: AsteroidField = s.parse().unwrap();
    let (nb_detect, score): (usize, usize) = field.responses();
    println!("Part1: From the best location, we can detect {nb_detect} asteroids");
    println!("Part2: The 200th asteroid to be vaporized has score {score}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = ".#..#
.....
#####
....#
...##";

    const INPUT_2: &str = "......#.#.
#..#.#....
..#######.
.#.#.###..
.#..#.....
..#....#.#
#..#....#.
.##.#..###
##...#..#.
.#....####";

    const INPUT_3: &str = "#.#...#.#.
.###....#.
.#....#...
##.#.#.#.#
....#.#.#.
.##..###.#
..#...##..
..##....##
......#...
.####.###.";

    const INPUT_4: &str = ".#..#..###
####.###.#
....###.#.
..###.##.#
##.##.#.#.
....###..#
..#.#..#.#
#..#.#.###
.##...##.#
.....#.#..";

    const INPUT_5: &str = ".#..##.###...#######
##.############..##.
.#.######.########.#
.###.#######.####.#.
#####.##.#.##.###.##
..#####..#.#########
####################
#.####....###.#.#.##
##.#################
#####.##.###..####..
..######..##.#######
####.##.####...##..#
.#####..#.######.###
##...#.##########...
#.##########.#######
.####.#.###.###.#.##
....##.##.###..#####
.#.#.###########.###
#.#.#.#####.####.###
###.##.####.##.#..##";

    #[test]
    fn test_1_part_1() {
        let field: AsteroidField = INPUT_1.parse().unwrap();
        assert_eq!(field.best_asteroid().0, 8);
    }

    #[test]
    fn test_2_part_1() {
        let field: AsteroidField = INPUT_2.parse().unwrap();
        assert_eq!(field.best_asteroid().0, 33);
    }

    #[test]
    fn test_3_part_1() {
        let field: AsteroidField = INPUT_3.parse().unwrap();
        assert_eq!(field.best_asteroid().0, 35);
    }

    #[test]
    fn test_4_part_1() {
        let field: AsteroidField = INPUT_4.parse().unwrap();
        assert_eq!(field.best_asteroid().0, 41);
    }

    #[test]
    fn test_5_part_1() {
        let field: AsteroidField = INPUT_5.parse().unwrap();
        assert_eq!(field.responses(), (210, 802));
    }
}
