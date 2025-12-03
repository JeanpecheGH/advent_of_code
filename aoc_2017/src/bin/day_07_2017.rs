use fxhash::FxHashMap;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{preceded, terminated};
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug, Clone)]
struct Program {
    name: String,
    weight: usize,
    balancing: Vec<String>,
}

impl Program {
    fn is_balancing(&self, other: &str) -> bool {
        self.balancing.contains(&other.to_string())
    }
}

impl FromStr for Program {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_program(s: &str) -> IResult<&str, Program> {
            let (s, name) = map(terminated(alpha1, tag(" (")), |w: &str| w.to_string()).parse(s)?;
            let (s, weight) = terminated(parse_usize, char(')')).parse(s)?;
            let (s, balancing) = opt(preceded(
                tag(" -> "),
                map(separated_list1(tag(", "), alpha1), |l| {
                    l.into_iter()
                        .map(|w: &str| w.to_string())
                        .collect::<Vec<String>>()
                }),
            ))
            .parse(s)?;

            Ok((
                s,
                Program {
                    name,
                    weight,
                    balancing: balancing.unwrap_or(Vec::new()),
                },
            ))
        }
        Ok(parse_program(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct RecursiveCircus {
    programs: FxHashMap<String, Program>,
}

impl RecursiveCircus {
    fn parent(&self, name: &str) -> Option<String> {
        for (n, p) in self.programs.iter() {
            if p.is_balancing(name) {
                return Some(n.clone());
            }
        }
        None
    }

    fn bottom_program(&self) -> String {
        let mut name = self.programs.keys().next().unwrap().clone();
        while let Some(parent) = self.parent(&name) {
            name = parent;
        }
        name
    }

    fn balance_disc(&self, name: &str) -> (Option<usize>, usize, usize) {
        let p: &Program = self.programs.get(name).unwrap();
        if p.balancing.is_empty() {
            //This program is not balancing others, just return its own size
            (None, p.weight, 0)
        } else {
            let results: Vec<(Option<usize>, usize, usize)> =
                p.balancing.iter().map(|n| self.balance_disc(n)).collect();

            if let Some(r) = results.iter().find_map(|(mod_w, _, _)| *mod_w) {
                //We found the rebalanced weight higher in the pile, don't bother computing weights
                (Some(r), 0, 0)
            } else {
                let mut one: Option<(usize, usize)> = None;
                let mut other: Option<(usize, usize)> = None;

                for (_, w, sum) in results.iter().copied() {
                    if one.is_none() {
                        one = Some((w, sum));
                    }

                    if w + sum != one.unwrap().0 + one.unwrap().1 {
                        if other.is_none() {
                            other = Some((w, sum));
                        } else {
                            //The real target is "other"
                            (one, other) = (other, one);
                        }
                    }
                }

                if let Some((_, sum)) = other {
                    //We found the rebalanced weight now, pass along
                    let (target_w, target_sum) = one.unwrap();
                    (Some(target_w + target_sum - sum), 0, 0)
                } else {
                    //We need to compute the weights to pass along
                    let sum: usize = results.iter().map(|(_, w, s)| w + s).sum();
                    (None, p.weight, sum)
                }
            }
        }
    }

    fn balance_bottom(&self) -> (String, usize) {
        //Start from the bottom program
        let bottom: String = self.bottom_program();
        let w: usize = self.balance_disc(&bottom).0.unwrap_or(0);
        (bottom, w)
    }
}

impl FromStr for RecursiveCircus {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let programs: FxHashMap<String, Program> = s
            .lines()
            .map(|l| {
                let p: Program = l.parse().unwrap();
                (p.name.clone(), p)
            })
            .collect();
        Ok(RecursiveCircus { programs })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_07.txt").expect("Cannot open input file");
    let circus: RecursiveCircus = s.parse().unwrap();
    let (name, weight) = circus.balance_bottom();

    println!("Part1: The bottom program is named \"{name}\"");
    println!(
        "Part2: We must change the weight of a program to {weight} to balance all the programs"
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "pbga (66)
xhth (57)
ebii (61)
havc (66)
ktlj (57)
fwft (72) -> ktlj, cntj, xhth
qoyq (66)
padx (45) -> pbga, havc, qoyq
tknk (41) -> ugml, padx, fwft
jptl (61)
ugml (68) -> gyxo, ebii, jptl
gyxo (61)
cntj (57)
";

    #[test]
    fn part_1() {
        let circus: RecursiveCircus = EXAMPLE_1.parse().unwrap();
        assert_eq!("tknk", circus.bottom_program());
    }

    #[test]
    fn part_2() {
        let circus: RecursiveCircus = EXAMPLE_1.parse().unwrap();
        assert_eq!(60, circus.balance_bottom().1);
    }
}
