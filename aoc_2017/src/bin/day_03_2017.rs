use fxhash::FxHashMap;
use util::coord::PosI;
use util::orientation::Dir;

const INPUT: usize = 289326;

struct SpiralRunner {
    pos: PosI,
    dir: Dir,
    up: isize,
    right: isize,
    down: isize,
    left: isize,
}

impl SpiralRunner {
    fn new() -> SpiralRunner {
        SpiralRunner {
            pos: PosI(0, 0),
            dir: Dir::East,
            up: 0,
            right: 0,
            down: 0,
            left: 0,
        }
    }
    fn next(&mut self) -> PosI {
        match self.dir {
            Dir::North => {
                if self.pos.1 == self.up {
                    self.up += 1;
                    self.dir = Dir::West;
                }
                self.pos.1 += 1;
            }
            Dir::East => {
                if self.pos.0 == self.right {
                    self.right += 1;
                    self.dir = Dir::North;
                }
                self.pos.0 += 1;
            }
            Dir::South => {
                if self.pos.1 == self.down {
                    self.down -= 1;
                    self.dir = Dir::East;
                }
                self.pos.1 -= 1;
            }
            Dir::West => {
                if self.pos.0 == self.left {
                    self.left -= 1;
                    self.dir = Dir::South;
                }
                self.pos.0 -= 1;
            }
        }
        self.pos
    }
}

fn dist_to(target: usize) -> usize {
    let mut runner: SpiralRunner = SpiralRunner::new();

    for _ in 1..target {
        runner.next();
    }

    runner.pos.distance(PosI(0, 0))
}

fn first_larger(target: usize) -> usize {
    let mut runner: SpiralRunner = SpiralRunner::new();
    let mut map: FxHashMap<PosI, usize> = FxHashMap::default();
    let mut add = 1;
    map.insert(PosI(0, 0), add);

    while add < target {
        let pos: PosI = runner.next();
        add = pos
            .neighbours_diag()
            .into_iter()
            .map(|ngb| map.get(&ngb).unwrap_or(&0))
            .sum();
        map.insert(pos, add);
    }
    add
}

fn main() {
    let now = std::time::Instant::now();

    println!(
        "Part1: The square {INPUT} is {} steps away from the center",
        dist_to(INPUT)
    );
    println!(
        "Part2: The first written value over {INPUT} is {}",
        first_larger(INPUT)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        assert_eq!(0, dist_to(1));
    }

    #[test]
    fn part_1_test_2() {
        assert_eq!(3, dist_to(12));
    }

    #[test]
    fn part_1_test_3() {
        assert_eq!(2, dist_to(23));
    }

    #[test]
    fn part_1_test_4() {
        assert_eq!(31, dist_to(1024));
    }

    #[test]
    fn part_2_test_1() {
        assert_eq!(122, first_larger(60));
    }

    #[test]
    fn part_2_test_2() {
        assert_eq!(806, first_larger(750));
    }
}
