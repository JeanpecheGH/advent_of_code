use std::str::FromStr;
use util::coord::Pos;
use util::orientation::Dir;

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tube {
    Vertical,
    Horizontal,
    Cross,
    Letter(char),
}

impl Tube {
    fn from_char(c: char) -> Option<Tube> {
        match c {
            '|' => Some(Tube::Vertical),
            '-' => Some(Tube::Horizontal),
            '+' => Some(Tube::Cross),
            'A'..='Z' => Some(Tube::Letter(c)),
            _ => None,
        }
    }
}

struct RoutineDiagram {
    grid: Vec<Vec<Option<Tube>>>,
}

impl RoutineDiagram {
    fn collect_letters(&self) -> (String, usize) {
        let mut letters: Vec<char> = Vec::new();
        let mut steps: usize = 0;

        //Find starting position
        let x: usize = self.grid[0].iter().position(|&t| t.is_some()).unwrap();
        let mut pos = Pos(x, 0);
        let mut dir = Dir::South;

        while let Some(tube) = self.grid[pos.1][pos.0] {
            if let Tube::Letter(c) = tube {
                letters.push(c);
            }
            steps += 1;
            (pos, dir) = self.next_pos(pos, dir);
        }

        (letters.iter().collect(), steps)
    }

    fn next_pos(&self, Pos(x, y): Pos, dir: Dir) -> (Pos, Dir) {
        let t: Option<Tube> = self.grid[y][x];
        match t {
            Some(Tube::Vertical) | Some(Tube::Horizontal) | Some(Tube::Letter(_)) => {
                //Continue ahead
                match dir {
                    Dir::North => (Pos(x, y - 1), dir),
                    Dir::East => (Pos(x + 1, y), dir),
                    Dir::South => (Pos(x, y + 1), dir),
                    Dir::West => (Pos(x - 1, y), dir),
                }
            }
            Some(Tube::Cross) => {
                //Check left and right for the next tube
                match dir {
                    Dir::North | Dir::South => {
                        if x.checked_sub(1)
                            .and_then(|new_x| {
                                self.grid
                                    .get(y)
                                    .and_then(|row| row.get(new_x).copied().flatten())
                            })
                            .is_some()
                        {
                            (Pos(x - 1, y), Dir::West)
                        } else if self
                            .grid
                            .get(y)
                            .and_then(|row| row.get(x + 1).copied().flatten())
                            .is_some()
                        {
                            (Pos(x + 1, y), Dir::East)
                        } else {
                            (Pos(x, y), dir)
                        }
                    }
                    Dir::East | Dir::West => {
                        if y.checked_sub(1)
                            .and_then(|new_y| {
                                self.grid
                                    .get(new_y)
                                    .and_then(|row| row.get(x).copied().flatten())
                            })
                            .is_some()
                        {
                            (Pos(x, y - 1), Dir::North)
                        } else if self
                            .grid
                            .get(y + 1)
                            .and_then(|row| row.get(x).copied().flatten())
                            .is_some()
                        {
                            (Pos(x, y + 1), Dir::South)
                        } else {
                            (Pos(x, y), dir)
                        }
                    }
                }
            }
            None => (Pos(x, y), dir),
        }
    }
}

impl FromStr for RoutineDiagram {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let grid: Vec<Vec<Option<Tube>>> = s
            .lines()
            .map(|l| l.chars().map(Tube::from_char).collect())
            .collect();

        Ok(RoutineDiagram { grid })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_19.txt").expect("Cannot open input file");
    let diagram: RoutineDiagram = s.parse().unwrap();

    let (word, steps) = diagram.collect_letters();

    println!("Part1: The letters seen by the packet are {word}");
    println!("Part2: The packet travelled {steps} steps");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "     |          
     |  +--+    
     A  |  C    
 F---|----E|--+ 
     |  |  |  D 
     +B-+  +--+
";

    #[test]
    fn part_1_and_2() {
        let diagram: RoutineDiagram = EXAMPLE_1.parse().unwrap();
        assert_eq!(("ABCDEF".to_string(), 38), diagram.collect_letters());
    }
}
