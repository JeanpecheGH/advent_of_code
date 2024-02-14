use fxhash::FxHashSet;
use itertools::Itertools;
use std::cmp::{min_by, Ordering};
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::coord::Pos;

#[derive(Debug, Copy, Clone)]
struct Unit {
    hp: usize,
    dmg: usize,
    goblin: bool,
    pos: Pos,
}

impl Unit {
    fn new(pos: Pos, goblin: bool) -> Self {
        Unit {
            hp: 200,
            dmg: 3,
            goblin,
            pos,
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Elf,
    Goblin,
}

impl Tile {
    fn from(c: char) -> Self {
        match c {
            '#' => Tile::Wall,
            'E' => Tile::Elf,
            'G' => Tile::Goblin,
            _ => Tile::Empty,
        }
    }

    fn to_char(&self) -> char {
        match self {
            Tile::Empty => '.',
            Tile::Wall => '#',
            Tile::Elf => 'E',
            Tile::Goblin => 'G',
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct AStarNode {
    pos: Pos,
    first_move: Pos,
    dist: usize,
    heuristic: usize,
}

impl AStarNode {
    fn from_first_move(first_move: Pos, to: Pos) -> Self {
        AStarNode {
            pos: first_move,
            first_move,
            dist: 1,
            heuristic: first_move.distance(to),
        }
    }
}

impl Ord for AStarNode {
    fn cmp(&self, other: &Self) -> Ordering {
        (other.dist + other.heuristic)
            .cmp(&(self.dist + self.heuristic))
            .then(other.first_move.1.cmp(&self.first_move.1))
            .then(other.first_move.0.cmp(&self.first_move.0))
    }
}

impl PartialOrd for AStarNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct CaveBattle {
    grid: Vec<Vec<Tile>>,
    units: Vec<Unit>,
    nb_turn: usize,
}

impl CaveBattle {
    fn outcome_score(&self) -> usize {
        self.units.iter().map(|u| u.hp).sum::<usize>() * self.nb_turn
    }

    fn outcome(&mut self) -> usize {
        while self.turn(0).is_none() {}
        self.outcome_score()
    }

    fn cheating_outcome(&self) -> (usize, usize) {
        let mut bonus_dmg: usize = 0;
        loop {
            bonus_dmg += 1;
            let mut new_battle: CaveBattle = self.clone();

            loop {
                match new_battle.turn(bonus_dmg) {
                    None => (),
                    Some(true) => break, //An elf died, we can skip this battle
                    Some(false) => return (new_battle.outcome_score(), bonus_dmg + 3),
                }
            }
        }
    }

    //Returns false when a Unit does not find a target
    fn turn(&mut self, bonus_dmg: usize) -> Option<bool> {
        //Sort unit by reading order
        self.units
            .sort_by(|a, b| a.pos.1.cmp(&b.pos.1).then(a.pos.0.cmp(&b.pos.0)));

        let mut i: usize = 0;
        while i < self.units.len() {
            let unit: Unit = self.units[i];

            //Search for valid foes
            let foes: Vec<Unit> = self
                .units
                .iter()
                .filter(|u| u.goblin != unit.goblin)
                .copied()
                .collect();

            if foes.is_empty() {
                //No foe to fight anymore, battle ends
                return Some(false);
            }

            //Check if we can already attack a neighbour
            let ngbs: Vec<Pos> = unit.pos.neighbours();
            let need_to_move: bool = ngbs.into_iter().all(|Pos(x, y)| {
                let t: Tile = if unit.goblin { Tile::Elf } else { Tile::Goblin };
                self.grid[y][x] != t
            });

            if need_to_move {
                //Move and update current unit
                self.move_to_foe(foes, unit, i);
            }

            //Attack best neighbour
            let (add, elf_died): (usize, bool) = self.attack(i, bonus_dmg);
            i += add;
            if elf_died && bonus_dmg > 0 {
                return Some(true);
            }
        }

        self.nb_turn += 1;
        //Finishing the turn naturally, expect another turn
        None
    }

    fn move_to_foe(&mut self, foes: Vec<Unit>, unit: Unit, index: usize) {
        //Compute valid targets
        let mut targets: Vec<(Pos, usize)> = foes
            .into_iter()
            .flat_map(|u| u.pos.neighbours())
            .filter(|&Pos(x, y)| self.grid[y][x] == Tile::Empty)
            .map(|p| (p, unit.pos.distance(p)))
            .collect();

        targets.sort_by(|(_, a), (_, b)| a.cmp(b));

        //Find best target to move to
        let move_to: Option<Pos> = targets
            .into_iter()
            .fold((None, usize::MAX), |(opt_pos, min_dist), (p, dist)| {
                if dist > min_dist {
                    //Path to p cannot be shorter, don't compute shortest path
                    (opt_pos, min_dist)
                } else if let Some((first_move, actual_dist)) = self.a_star(unit.pos, p) {
                    match actual_dist.cmp(&min_dist) {
                        Ordering::Less => (Some(first_move), actual_dist),
                        Ordering::Equal => {
                            let new_pos: Pos = min_by(opt_pos.unwrap(), first_move, |a, b| {
                                a.1.cmp(&b.1).then(a.0.cmp(&b.0))
                            });
                            (Some(new_pos), min_dist)
                        }
                        Ordering::Greater => (opt_pos, min_dist),
                    }
                } else {
                    (opt_pos, min_dist)
                }
            })
            .0;
        if let Some(Pos(x, y)) = move_to {
            let Pos(old_x, old_y) = unit.pos;
            self.grid[old_y][old_x] = Tile::Empty;
            let t: Tile = if unit.goblin { Tile::Goblin } else { Tile::Elf };
            self.grid[y][x] = t;
            self.units[index].pos = Pos(x, y);
        }
    }

    //First element is 0 if a creature with inferior index to the current one died, 1 otherwise
    //Second element is true if an elf died
    fn attack(&mut self, index: usize, bonus_dmg: usize) -> (usize, bool) {
        let mut ret: usize = 1;
        let mut elf_died: bool = false;
        let unit: Unit = self.units[index];
        let ngbs: Vec<(Pos, usize, usize)> = unit
            .pos
            .neighbours()
            .into_iter()
            .filter(|&Pos(x, y)| {
                let t: Tile = if unit.goblin { Tile::Elf } else { Tile::Goblin };
                self.grid[y][x] == t
            })
            .map(|p| {
                let (i, foe) = self.units.iter().find_position(|u| u.pos == p).unwrap();
                (p, i, foe.hp)
            })
            .collect();

        let to_attack: Option<usize> = ngbs
            .iter()
            .min_by(|(Pos(x_1, y_1), _, hp_1), (Pos(x_2, y_2), _, hp_2)| {
                hp_1.cmp(hp_2).then(y_1.cmp(y_2)).then(x_1.cmp(x_2))
            })
            .map(|(_, i, _)| *i);

        to_attack.into_iter().for_each(|i| {
            let dmg: usize = if unit.goblin {
                unit.dmg
            } else {
                unit.dmg + bonus_dmg
            };
            self.units[i].hp = self.units[i].hp.saturating_sub(dmg);
            if self.units[i].hp == 0 {
                let Pos(x, y) = self.units[i].pos;
                if !self.units[i].goblin {
                    elf_died = true;
                }
                self.grid[y][x] = Tile::Empty;
                self.units.remove(i);
                if i < index {
                    ret = 0;
                }
            }
        });

        (ret, elf_died)
    }

    fn a_star(&self, from: Pos, to: Pos) -> Option<(Pos, usize)> {
        let starting_neighbours: Vec<Pos> = from
            .neighbours()
            .iter()
            .filter(|&&Pos(x, y)| self.grid[y][x] == Tile::Empty)
            .copied()
            .collect();
        let mut queue: BinaryHeap<AStarNode> = starting_neighbours
            .iter()
            .map(|&p| AStarNode::from_first_move(p, to))
            .collect();

        let mut visited_pos: FxHashSet<(Pos, Pos)> =
            starting_neighbours.iter().map(|&p| (p, p)).collect();

        while let Some(best) = queue.pop() {
            //We arrived to the end, we know it's the best path by definition
            if best.pos == to {
                return Some((best.first_move, best.dist));
            }

            //Compute neighbours
            best.pos
                .neighbours()
                .iter()
                .filter(|&&Pos(x, y)| self.grid[y][x] == Tile::Empty)
                .filter(|&&p| visited_pos.insert((p, best.first_move)))
                .for_each(|&p| {
                    let n: AStarNode = AStarNode {
                        pos: p,
                        first_move: best.first_move,
                        dist: best.dist + 1,
                        heuristic: p.distance(to),
                    };
                    queue.push(n);
                })
        }
        None
    }
    #[allow(dead_code)]
    fn print(&self) {
        for row in self.grid.iter() {
            for tile in row.iter() {
                print!("{}", tile.to_char());
            }
            println!();
        }
    }
}

impl FromStr for CaveBattle {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut units: Vec<Unit> = Vec::new();
        let grid: Vec<Vec<Tile>> = s
            .lines()
            .enumerate()
            .map(|(y, l)| {
                l.chars()
                    .enumerate()
                    .map(|(x, c)| {
                        let t = Tile::from(c);
                        if t == Tile::Elf {
                            units.push(Unit::new(Pos(x, y), false))
                        } else if t == Tile::Goblin {
                            units.push(Unit::new(Pos(x, y), true))
                        }
                        t
                    })
                    .collect()
            })
            .collect();

        Ok(CaveBattle {
            grid,
            units,
            nb_turn: 0,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_15.txt").expect("Cannot open input file");
    let mut battle: CaveBattle = s.parse().unwrap();
    let second_battle: CaveBattle = battle.clone();

    println!("Part1: The outcome of the battle is {}", battle.outcome());
    let (score, elf_dmg) = second_battle.cheating_outcome();
    println!(
        "Part2: The first flawless victory for the Elves happens with {} attack power. Its outcome is {}",
        elf_dmg, score
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######";

    const EXAMPLE_2: &str = "#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######";

    const EXAMPLE_3: &str = "#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######";

    const EXAMPLE_4: &str = "#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######";

    const EXAMPLE_5: &str = "#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######";

    const EXAMPLE_6: &str = "#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########";

    #[test]
    fn part_1_test_1() {
        let mut battle: CaveBattle = EXAMPLE_1.parse().unwrap();
        assert_eq!(battle.outcome(), 27730);
    }

    #[test]
    fn part_1_test_2() {
        let mut battle: CaveBattle = EXAMPLE_2.parse().unwrap();
        assert_eq!(battle.outcome(), 36334);
    }

    #[test]
    fn part_1_test_3() {
        let mut battle: CaveBattle = EXAMPLE_3.parse().unwrap();
        assert_eq!(battle.outcome(), 39514);
    }

    #[test]
    fn part_1_test_4() {
        let mut battle: CaveBattle = EXAMPLE_4.parse().unwrap();
        assert_eq!(battle.outcome(), 27755);
    }

    #[test]
    fn part_1_test_5() {
        let mut battle: CaveBattle = EXAMPLE_5.parse().unwrap();
        assert_eq!(battle.outcome(), 28944);
    }

    #[test]
    fn part_1_test_6() {
        let mut battle: CaveBattle = EXAMPLE_6.parse().unwrap();
        assert_eq!(battle.outcome(), 18740);
    }

    #[test]
    fn part_2_test_1() {
        let battle: CaveBattle = EXAMPLE_1.parse().unwrap();
        assert_eq!(battle.cheating_outcome(), (4988, 15));
    }

    #[test]
    fn part_2_test_3() {
        let battle: CaveBattle = EXAMPLE_3.parse().unwrap();
        assert_eq!(battle.cheating_outcome(), (31284, 4));
    }

    #[test]
    fn part_2_test_4() {
        let battle: CaveBattle = EXAMPLE_4.parse().unwrap();
        assert_eq!(battle.cheating_outcome(), (3478, 15));
    }

    #[test]
    fn part_2_test_5() {
        let battle: CaveBattle = EXAMPLE_5.parse().unwrap();
        assert_eq!(battle.cheating_outcome(), (6474, 12));
    }

    #[test]
    fn part_2_test_6() {
        let battle: CaveBattle = EXAMPLE_6.parse().unwrap();
        assert_eq!(battle.cheating_outcome(), (1140, 34));
    }
}
