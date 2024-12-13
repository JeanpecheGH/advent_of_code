use fxhash::FxHashSet;
use std::cmp::Ordering;
use std::str::FromStr;
use util::coord::Pos;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
struct Fence {
    start: Pos,
    end: Pos,
    diff: Pos,
}

impl Fence {
    fn from(a: Pos, b: Pos) -> Self {
        let diff: Pos = Pos(b.0 - a.0, b.1 - a.1);
        Fence {
            start: a,
            end: b,
            diff,
        }
    }

    fn before_opt(&self, fences: &FxHashSet<Fence>) -> Option<Self> {
        if fences.contains(&self.cross_bb()) || fences.contains(&self.cross_ba()) {
            None
        } else {
            let before = self.before();
            if fences.contains(&before) {
                Some(before)
            } else {
                None
            }
        }
    }

    fn before(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(
            Pos(
                self.start.0.saturating_sub(x),
                self.start.1.saturating_sub(y),
            ),
            self.start,
        )
    }

    fn after_opt(&self, fences: &FxHashSet<Fence>) -> Option<Self> {
        if fences.contains(&self.cross_ab()) || fences.contains(&self.cross_aa()) {
            None
        } else {
            let after = self.after();
            if fences.contains(&after) {
                Some(after)
            } else {
                None
            }
        }
    }

    fn after(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(self.end, Pos(self.end.0 + x, self.end.1 + y))
    }

    fn cross_bb(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(
            Pos(
                self.start.0.saturating_sub(y),
                self.start.1.saturating_sub(x),
            ),
            self.start,
        )
    }

    fn cross_ba(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(self.start, Pos(self.start.0 + y, self.start.1 + x))
    }

    fn cross_ab(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(
            Pos(self.end.0.saturating_sub(y), self.end.1.saturating_sub(x)),
            self.end,
        )
    }

    fn cross_aa(&self) -> Self {
        let Pos(x, y) = self.diff;
        Self::from(self.end, Pos(self.end.0 + y, self.end.1 + x))
    }
}
struct Gardens {
    grid: Vec<Vec<char>>,
    max_x: usize,
    max_y: usize,
}
impl Gardens {
    fn plant_at(&self, Pos(x, y): Pos) -> char {
        self.grid[y][x]
    }
    fn fence_from(&self, start: Pos, visited: &mut FxHashSet<Pos>) -> usize {
        let mut area: usize = 0;
        let mut perimeter: usize = 0;

        let mut working_plots: Vec<Pos> = vec![start];

        while let Some(p) = working_plots.pop() {
            area += 1;
            perimeter += 4;
            let plant: char = self.plant_at(p);
            p.neighbours_safe(self.max_x, self.max_y)
                .iter()
                .for_each(|&ngb| {
                    if self.plant_at(ngb) == plant {
                        perimeter -= 1;
                        if visited.insert(ngb) {
                            working_plots.push(ngb);
                        }
                    }
                });
        }

        area * perimeter
    }
    fn fence_from_with_discount(&self, start: Pos, visited: &mut FxHashSet<Pos>) -> usize {
        fn count_sides(fences: &FxHashSet<Fence>) -> usize {
            let mut used: FxHashSet<Fence> = FxHashSet::default();
            let mut nb_sides: usize = 0;
            for &f in fences {
                if !used.contains(&f) {
                    // This fence is not used yet, it is a new side
                    nb_sides += 1;
                    used.insert(f);
                    // Get the 2 touching fences left & right || up & down
                    let mut before = f.before_opt(fences);
                    while let Some(b) = before {
                        used.insert(b);
                        before = b.before_opt(fences);
                    }
                    let mut after = f.after_opt(fences);
                    while let Some(a) = after {
                        used.insert(a);
                        after = a.after_opt(fences);
                    }
                }
            }
            nb_sides
        }
        let mut area: usize = 0;

        let mut working_plots: Vec<Pos> = vec![start];

        let mut fences: FxHashSet<Fence> = FxHashSet::default();

        while let Some(p) = working_plots.pop() {
            area += 1;
            // Add 4 fences around
            let b: Pos = Pos(p.0, p.1 + 1);
            let r: Pos = Pos(p.0 + 1, p.1);
            let br: Pos = Pos(p.0 + 1, p.1 + 1);

            fences.insert(Fence::from(p, b));
            fences.insert(Fence::from(p, r));
            fences.insert(Fence::from(r, br));
            fences.insert(Fence::from(b, br));

            let plant: char = self.plant_at(p);
            p.neighbours_safe(self.max_x, self.max_y)
                .iter()
                .for_each(|&ngb| {
                    if self.plant_at(ngb) == plant {
                        match (p.0.cmp(&ngb.0), p.1.cmp(&ngb.1)) {
                            // Remove fence to the right
                            (Ordering::Less, Ordering::Equal) => {
                                let _ = fences
                                    .remove(&Fence::from(Pos(p.0 + 1, p.1), Pos(p.0 + 1, p.1 + 1)));
                            }
                            // Remove fence to left
                            (Ordering::Greater, Ordering::Equal) => {
                                let _ = fences.remove(&Fence::from(p, Pos(p.0, p.1 + 1)));
                            }
                            // Remove fence to the bottom
                            (Ordering::Equal, Ordering::Less) => {
                                let _ = fences
                                    .remove(&Fence::from(Pos(p.0, p.1 + 1), Pos(p.0 + 1, p.1 + 1)));
                            }
                            // Remove fence to the top
                            (Ordering::Equal, Ordering::Greater) => {
                                let _ = fences.remove(&Fence::from(p, Pos(p.0 + 1, p.1)));
                            }
                            // Should not happen
                            _ => (),
                        }
                        if visited.insert(ngb) {
                            working_plots.push(ngb);
                        }
                    }
                });
        }
        let nb_sides = count_sides(&fences);

        area * nb_sides
    }

    fn total_price(&self, with_discount: bool) -> usize {
        let mut visited: FxHashSet<Pos> = FxHashSet::default();
        let mut total_price: usize = 0;

        for y in 0..self.max_y {
            for x in 0..self.max_x {
                let p = Pos(x, y);
                if visited.insert(p) {
                    if with_discount {
                        total_price += self.fence_from_with_discount(p, &mut visited);
                    } else {
                        total_price += self.fence_from(p, &mut visited);
                    }
                }
            }
        }

        total_price
    }
}

impl FromStr for Gardens {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<char>> = s
            .lines()
            .map(|l| l.chars().collect::<Vec<char>>())
            .collect();
        let max_x: usize = grid[0].len();
        let max_y: usize = grid.len();
        Ok(Gardens { grid, max_x, max_y })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_12.txt").expect("Cannot open input file");
    let gardens: Gardens = s.parse().unwrap();
    println!(
        "Part1: The price of fencing all the regions is {}",
        gardens.total_price(false)
    );
    println!(
        "Part2: With the bulk discount, the price drops to {}",
        gardens.total_price(true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "AAAA
BBCD
BBCC
EEEC
";

    const EXAMPLE_2: &str = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";

    const EXAMPLE_3: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";

    const EXAMPLE_4: &str = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";

    const EXAMPLE_5: &str = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";

    #[test]
    fn part_1_test_1() {
        let gardens: Gardens = EXAMPLE_1.parse().unwrap();
        assert_eq!(gardens.total_price(false), 140);
    }

    #[test]
    fn part_1_test_2() {
        let gardens: Gardens = EXAMPLE_2.parse().unwrap();
        assert_eq!(gardens.total_price(false), 772);
    }

    #[test]
    fn part_1_test_3() {
        let gardens: Gardens = EXAMPLE_3.parse().unwrap();
        assert_eq!(gardens.total_price(false), 1930);
    }

    #[test]
    fn part_2_test_1() {
        let gardens: Gardens = EXAMPLE_1.parse().unwrap();
        assert_eq!(gardens.total_price(true), 80);
    }

    #[test]
    fn part_2_test_2() {
        let gardens: Gardens = EXAMPLE_2.parse().unwrap();
        assert_eq!(gardens.total_price(true), 436);
    }

    #[test]
    fn part_2_test_3() {
        let gardens: Gardens = EXAMPLE_3.parse().unwrap();
        assert_eq!(gardens.total_price(true), 1206);
    }

    #[test]
    fn part_2_test_4() {
        let gardens: Gardens = EXAMPLE_4.parse().unwrap();
        assert_eq!(gardens.total_price(true), 236);
    }

    #[test]
    fn part_2_test_5() {
        let gardens: Gardens = EXAMPLE_5.parse().unwrap();
        assert_eq!(gardens.total_price(true), 368);
    }
}
