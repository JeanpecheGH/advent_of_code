use nom::character::complete::char;
use nom::multi::separated_list1;
use nom::sequence::separated_pair;
use nom::IResult;
use std::collections::HashSet;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::coord::Pos3;

#[derive(Copy, Clone, Debug)]
struct Brick {
    start: Pos3,
    end: Pos3,
}

impl Brick {
    fn supports(&self, other: &Brick) -> bool {
        let other_parts = other.parts();
        self.parts()
            .iter()
            .any(|&Pos3(x, y, z)| other_parts.contains(&Pos3(x, y, z + 1)))
    }

    fn lower(&self) -> Brick {
        let Pos3(x, y, z): Pos3 = self.start;
        let Pos3(i, j, k): Pos3 = self.end;

        Brick {
            start: Pos3(x, y, z - 1),
            end: Pos3(i, j, k - 1),
        }
    }

    fn parts(&self) -> Vec<Pos3> {
        let mut parts: Vec<Pos3> = Vec::new();
        for z in self.start.2..=self.end.2 {
            for y in self.start.1..=self.end.1 {
                for x in self.start.0..=self.end.0 {
                    parts.push(Pos3(x, y, z))
                }
            }
        }
        parts
    }
}

impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_pos3(s: &str) -> IResult<&str, Pos3> {
            let (s, l) = separated_list1(char(','), parse_usize)(s)?;
            Ok((s, Pos3(l[0], l[1], l[2])))
        }
        fn parse_brick(s: &str) -> IResult<&str, Brick> {
            let (s, (start, end)) = separated_pair(parse_pos3, char('~'), parse_pos3)(s)?;
            Ok((s, Brick { start, end }))
        }
        Ok(parse_brick(s).unwrap().1)
    }
}
#[derive(Clone, Debug)]
struct BrickPile {
    pile: Vec<Brick>,
}

impl BrickPile {
    fn all_above(supports: &[Vec<usize>], bases: &[usize]) -> Vec<usize> {
        let mut multi_set: HashSet<usize> = HashSet::new();
        //Add all supported by any of our base
        bases.iter().for_each(|&i| {
            supports[i].iter().for_each(|&j| {
                multi_set.insert(j);
            })
        });
        //Remove all supported by any out of our base
        supports.iter().enumerate().for_each(|(i, v)| {
            if !bases.contains(&i) {
                v.iter().for_each(|j| {
                    multi_set.remove(j);
                })
            }
        });

        let mut out: HashSet<usize> = multi_set.into_iter().collect();
        bases.iter().for_each(|&b| {
            out.insert(b);
        });
        let base_set: HashSet<usize> = bases.iter().copied().collect();
        if out != base_set {
            let new_bases: Vec<usize> = out.into_iter().collect();
            Self::all_above(supports, &new_bases)
        } else {
            out.into_iter().collect()
        }
    }
    fn compute(&self) -> (usize, usize) {
        let stacked: Vec<Brick> = self.stack_bricks();

        //Get the id of all bricks directly above the current brick
        let supports: Vec<Vec<usize>> = stacked
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let mut sup: Vec<usize> = Vec::new();
                for (j, other) in stacked.iter().enumerate() {
                    if j != i && b.supports(other) {
                        sup.push(j);
                    }
                }
                sup
            })
            .collect();

        //Get the id of all bricks that are supported only by the current brick
        let supports_alone: Vec<Vec<usize>> = supports
            .iter()
            .enumerate()
            .map(|(i, v)| {
                v.iter()
                    .filter(|s| {
                        supports
                            .iter()
                            .enumerate()
                            .all(|(j, supps)| i == j || !supps.contains(s))
                    })
                    .copied()
                    .collect::<Vec<usize>>()
            })
            .collect();

        //We can disintegrate all bricks that are not supporting any other brick alone
        let can_disintegrate: usize = supports_alone.iter().filter(|v| v.is_empty()).count();

        let score: usize = supports_alone
            .iter()
            .map(|s| match s.len() {
                0 => 0,
                _ => Self::all_above(&supports, s).len(),
            })
            .sum();

        (can_disintegrate, score)
    }

    fn stack_bricks(&self) -> Vec<Brick> {
        let mut sorted_pile = self.pile.clone();
        //Sort by lowest altitude
        sorted_pile.sort_by(|a, b| a.start.2.cmp(&b.start.2));

        let mut all_parts: HashSet<Pos3> = HashSet::new();

        let mut final_pile: Vec<Brick> = Vec::new();
        for b in sorted_pile.iter() {
            let mut br: Brick = *b;
            let mut parts = b.parts();
            while !parts
                .iter()
                .any(|&Pos3(x, y, z)| z == 1 || all_parts.contains(&Pos3(x, y, z - 1)))
            {
                br = br.lower();
                parts = br.parts();
            }
            final_pile.push(br);
            parts.iter().for_each(|&p| {
                all_parts.insert(p);
            });
        }
        final_pile
    }
}

impl FromStr for BrickPile {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let pile: Vec<Brick> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(BrickPile { pile })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_22.txt").expect("Cannot open input file");
    let pile: BrickPile = s.parse().unwrap();
    let (disintegrate, falling_bricks) = pile.compute();
    println!("Part1: We can safely disintegrate {} bricks", disintegrate);
    println!(
        "Part2: The sum of the number of falling bricks is {}",
        falling_bricks
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9
";

    #[test]
    fn part_1() {
        let pile: BrickPile = EXAMPLE_1.parse().unwrap();
        assert_eq!(pile.compute(), (5, 7));
    }
}
