use std::collections::HashSet;
use std::str::FromStr;

type Pos = (usize, usize);

#[derive(Debug, Eq, PartialEq, Clone)]
struct Blizzards {
    width: usize,
    height: usize,
    northern: Vec<HashSet<usize>>,
    southern: Vec<HashSet<usize>>,
    western: Vec<HashSet<usize>>,
    eastern: Vec<HashSet<usize>>,
}

impl Blizzards {
    fn is_free(&self, pos: &Pos, time: usize) -> bool {
        if pos.1 == 0 || pos.1 == self.height + 1 {
            true
        } else {
            self.northern_free(pos, time)
                && self.southern_free(pos, time)
                && self.western_free(pos, time)
                && self.eastern_free(pos, time)
        }
    }

    fn northern_free(&self, pos: &Pos, time: usize) -> bool {
        let (x, y) = *pos;
        let v: usize = (y - 1 + time) % self.height + 1;
        !self.northern[x].contains(&v)
    }

    fn southern_free(&self, pos: &Pos, time: usize) -> bool {
        let (x, y) = *pos;
        let v: usize = (y + self.height * time - 1 - time) % self.height + 1;
        !self.southern[x].contains(&v)
    }

    fn western_free(&self, pos: &Pos, time: usize) -> bool {
        let (x, y) = *pos;
        let v: usize = (x - 1 + time) % self.width + 1;
        !self.western[y].contains(&v)
    }

    fn eastern_free(&self, pos: &Pos, time: usize) -> bool {
        let (x, y) = *pos;
        let v: usize = (x + self.width * time - 1 - time) % self.width + 1;
        !self.eastern[y].contains(&v)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
struct Baasin {
    width: usize,
    height: usize,
    start: Pos,
    end: Pos,
    blizzards: Blizzards,
}

impl Baasin {
    fn a_star(&self, start: Expedition, backwards: bool) -> Expedition {
        let mut current_exp: Vec<Expedition> = vec![start];
        let mut visited: HashSet<Expedition> = HashSet::new();
        visited.insert(start);

        let target: Pos = if backwards { self.start } else { self.end };

        loop {
            let best_expe: Expedition = current_exp.pop().unwrap();
            if best_expe.pos == target {
                return best_expe;
            }
            self.neighbours(best_expe, backwards)
                .into_iter()
                .filter(|ngb| visited.insert(*ngb))
                .for_each(|ngb| {
                    let score: usize = ngb.score(&target);
                    let idx: usize =
                        current_exp.partition_point(|other| other.score(&target) > score);
                    current_exp.insert(idx, ngb);
                })
        }
    }

    fn neighbours(&self, expe: Expedition, backwards: bool) -> Vec<Expedition> {
        let candidates: Vec<Pos> = self.ngb_pos(&expe.pos, backwards);
        candidates
            .into_iter()
            .filter(|p| self.blizzards.is_free(p, expe.time + 1))
            .map(|p| Expedition {
                pos: p,
                time: expe.time + 1,
            })
            .collect()
    }

    fn ngb_pos(&self, pos: &Pos, backwards: bool) -> Vec<Pos> {
        let (x, y) = *pos;
        if y == 0 {
            vec![(1, 0), (1, 1)]
        } else if y == self.height + 1 {
            vec![(self.width, self.height), (self.width, self.height + 1)]
        } else if x == self.width && y == self.height && !backwards {
            vec![(self.width, self.height + 1)]
        } else if x == 1 && y == 1 && backwards {
            vec![(1, 0)]
        } else {
            let candidates: Vec<Pos> = vec![(x - 1, y), (x, y - 1), (x, y), (x + 1, y), (x, y + 1)];
            candidates
                .into_iter()
                .filter(|(i, j)| *i > 0 && *i <= self.width && *j > 0 && *j <= self.height)
                .collect()
        }
    }
}

impl FromStr for Baasin {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let height: usize = lines.len() - 2;
        let width: usize = lines.first().unwrap().len() - 2;
        let start: Pos = (1, 0);
        let end: Pos = (width, height + 1);
        let mut northern: Vec<HashSet<usize>> = vec![HashSet::new(); width + 1];
        let mut southern: Vec<HashSet<usize>> = vec![HashSet::new(); width + 1];
        let mut western: Vec<HashSet<usize>> = vec![HashSet::new(); height + 1];
        let mut eastern: Vec<HashSet<usize>> = vec![HashSet::new(); height + 1];
        lines.iter().enumerate().for_each(|(j, row)| {
            row.chars().enumerate().for_each(|(i, c)| {
                match c {
                    '^' => {
                        northern[i].insert(j);
                    }
                    'v' => {
                        southern[i].insert(j);
                    }
                    '<' => {
                        western[j].insert(i);
                    }
                    '>' => {
                        eastern[j].insert(i);
                    }
                    _ => (),
                };
            });
        });

        let blizzards: Blizzards = Blizzards {
            width,
            height,
            northern,
            southern,
            western,
            eastern,
        };
        Ok(Self {
            width,
            height,
            start,
            end,
            blizzards,
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
struct Expedition {
    pos: Pos,
    time: usize,
}

impl Expedition {
    fn new(pos: Pos) -> Self {
        Expedition { pos, time: 0 }
    }

    fn score(&self, target: &Pos) -> usize {
        target.0.abs_diff(self.pos.0) + target.1.abs_diff(self.pos.1) + self.time
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_24.txt").expect("Cannot open input file");

    let basin: Baasin = s.parse().unwrap();
    let start: Expedition = Expedition::new(basin.start);
    let target: Expedition = basin.a_star(start, false);
    println!(
        "Part1: You take {} minutes to reach the target",
        target.time
    );
    let start_again: Expedition = basin.a_star(target, true);
    println!(
        "And then {} minutes to reach back to the start",
        start_again.time
    );
    let target_again: Expedition = basin.a_star(start_again, false);
    println!(
        "Part1: You take {} total minutes to reach the target after going back to get the snack of this silly Elf",
        target_again.time
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#";

    #[test]
    fn part_1() {
        let basin: Baasin = INPUT.parse().unwrap();
        let start: Expedition = Expedition::new(basin.start);
        let target: Expedition = basin.a_star(start, false);
        assert_eq!(target.time, 18);
    }

    #[test]
    fn part_2() {
        let basin: Baasin = INPUT.parse().unwrap();
        let start: Expedition = Expedition::new(basin.start);
        let target: Expedition = basin.a_star(start, false);
        let start_again: Expedition = basin.a_star(target, true);
        let target_again: Expedition = basin.a_star(start_again, false);
        assert_eq!(target_again.time, 54);
    }
}
