use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
struct Crucible {
    pos: Pos,
    straight: usize,
    dir: Dir,
    heatloss: usize,
}

impl Crucible {
    fn next(&self, city: &FactoryCity, min: usize, max: usize) -> Vec<Crucible> {
        let mut next_crucibles: Vec<Crucible> = Vec::new();
        if let Some(left) = self.left(city, min) {
            next_crucibles.push(left)
        }
        if let Some(right) = self.right(city, min) {
            next_crucibles.push(right)
        }
        if let Some(ahead) = self.ahead(city, max) {
            next_crucibles.push(ahead)
        }
        next_crucibles
    }

    fn left(&self, city: &FactoryCity, min: usize) -> Option<Crucible> {
        if self.straight < min {
            return None;
        }
        let Pos(max_x, max_y) = city.end;
        match (self.pos, self.dir) {
            (Pos(_, 0), Dir::East) => None,
            (Pos(x, _), Dir::South) if x == max_x => None,
            (Pos(_, y), Dir::West) if y == max_y => None,
            (Pos(0, _), Dir::North) => None,
            (Pos(x, y), d) => {
                let new_dir = d.turn_left();
                let new_pos: Pos = match d {
                    Dir::North => Pos(x - 1, y),
                    Dir::West => Pos(x, y + 1),
                    Dir::South => Pos(x + 1, y),
                    Dir::East => Pos(x, y - 1),
                };
                Some(Crucible {
                    pos: new_pos,
                    straight: 1,
                    dir: new_dir,
                    heatloss: self.heatloss + city.heatloss_at(new_pos),
                })
            }
        }
    }

    fn right(&self, city: &FactoryCity, min: usize) -> Option<Crucible> {
        if self.straight < min {
            return None;
        }
        let Pos(max_x, max_y) = city.end;
        match (self.pos, self.dir) {
            (Pos(_, 0), Dir::West) => None,
            (Pos(x, _), Dir::North) if x == max_x => None,
            (Pos(_, y), Dir::East) if y == max_y => None,
            (Pos(0, _), Dir::South) => None,
            (Pos(x, y), d) => {
                let new_dir = d.turn_right();
                let new_pos: Pos = match d {
                    Dir::North => Pos(x + 1, y),
                    Dir::West => Pos(x, y - 1),
                    Dir::South => Pos(x - 1, y),
                    Dir::East => Pos(x, y + 1),
                };
                Some(Crucible {
                    pos: new_pos,
                    straight: 1,
                    dir: new_dir,
                    heatloss: self.heatloss + city.heatloss_at(new_pos),
                })
            }
        }
    }

    fn ahead(&self, city: &FactoryCity, max: usize) -> Option<Crucible> {
        if self.straight == max {
            return None;
        }
        let Pos(max_x, max_y) = city.end;
        match (self.pos, self.dir) {
            (Pos(_, 0), Dir::North) => None,
            (Pos(x, _), Dir::East) if x == max_x => None,
            (Pos(_, y), Dir::South) if y == max_y => None,
            (Pos(0, _), Dir::West) => None,
            (Pos(x, y), d) => {
                let new_pos: Pos = match d {
                    Dir::North => Pos(x, y - 1),
                    Dir::West => Pos(x - 1, y),
                    Dir::South => Pos(x, y + 1),
                    Dir::East => Pos(x + 1, y),
                };
                Some(Crucible {
                    pos: new_pos,
                    straight: self.straight + 1,
                    dir: d,
                    heatloss: self.heatloss + city.heatloss_at(new_pos),
                })
            }
        }
    }
}

impl Ord for Crucible {
    fn cmp(&self, other: &Self) -> Ordering {
        other.heatloss.cmp(&self.heatloss)
    }
}

impl PartialOrd for Crucible {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct FactoryCity {
    grid: Vec<Vec<usize>>,
    end: Pos,
}

impl FactoryCity {
    fn min_heatloss(&self, min: usize, max: usize) -> usize {
        let start: Crucible = Crucible {
            pos: Pos(0, 0),
            straight: 0,
            dir: Dir::East,
            heatloss: 0,
        };

        let mut heap: BinaryHeap<Crucible> = BinaryHeap::new();
        heap.push(start);

        let mut visit_map: HashMap<(Pos, Dir, usize), usize> = HashMap::new();
        visit_map.insert((start.pos, start.dir, start.straight), start.heatloss);

        while let Some(best) = heap.pop() {
            //We arrived to the end, we know it's the best path by definition
            if best.pos == self.end && best.straight >= min {
                return best.heatloss;
            }

            //Compute neighbours
            best.next(self, min, max).into_iter().for_each(|c| {
                //Only insert if we improve the heatloss for given (Pos,Dir,nb_straight)
                let entry = visit_map
                    .entry((c.pos, c.dir, c.straight))
                    .or_insert_with(|| {
                        heap.push(c);
                        c.heatloss
                    });
                if c.heatloss < *entry {
                    *entry = c.heatloss;
                    heap.push(c);
                }
            });
        }
        usize::MAX
    }
    fn heatloss_at(&self, Pos(x, y): Pos) -> usize {
        self.grid[y][x]
    }
}

impl FromStr for FactoryCity {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<usize>> = s
            .lines()
            .map(|l| {
                l.chars()
                    .map(|c| c.to_digit(10).unwrap() as usize)
                    .collect()
            })
            .collect();
        let end: Pos = Pos(grid[0].len() - 1, grid.len() - 1);
        Ok(FactoryCity { grid, end })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_17.txt").expect("Cannot open input file");
    let city: FactoryCity = s.parse().unwrap();
    println!(
        "Part1: The minimal heat loss occurring while moving the crucible is {}",
        city.min_heatloss(0, 3)
    );
    println!(
        "Part2: When using an ultra crucible, the minimal heat loss is {}",
        city.min_heatloss(4, 10)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533
";

    const EXAMPLE_2: &str = "111111111111
999999999991
999999999991
999999999991
999999999991
";

    #[test]
    fn part_1() {
        let city: FactoryCity = EXAMPLE_1.parse().unwrap();
        assert_eq!(city.min_heatloss(0, 3), 102);
    }
    #[test]
    fn part_2_test_1() {
        let city: FactoryCity = EXAMPLE_1.parse().unwrap();
        assert_eq!(city.min_heatloss(4, 10), 94);
    }
    #[test]
    fn part_2_test_2() {
        let city: FactoryCity = EXAMPLE_2.parse().unwrap();
        assert_eq!(city.min_heatloss(4, 10), 71);
    }
}
