use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

const ROOT: &str = "root";
const HUMAN: &str = "humn";

#[derive(Debug, Clone)]
enum MonkeyValue {
    Val(isize),
    Add((String, String)),
    Sub((String, String)),
    Mul((String, String)),
    Div((String, String)),
}

#[derive(Debug, Clone)]
struct Monkey {
    name: String,
    value: MonkeyValue,
}

impl Monkey {
    fn value(&self) -> Option<isize> {
        match self.value {
            MonkeyValue::Val(v) => Some(v),
            MonkeyValue::Add(_) => None,
            MonkeyValue::Sub(_) => None,
            MonkeyValue::Mul(_) => None,
            MonkeyValue::Div(_) => None,
        }
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&[' ', ':']).collect();
        let name: String = words[0].to_string();
        if words.len() == 3 {
            let value: isize = words[2].parse().unwrap();
            Ok(Monkey {
                name,
                value: MonkeyValue::Val(value),
            })
        } else {
            let pair: (String, String) = (words[2].to_string(), words[4].to_string());
            match words[3] {
                "+" => Ok(Monkey {
                    name,
                    value: MonkeyValue::Add(pair),
                }),
                "-" => Ok(Monkey {
                    name,
                    value: MonkeyValue::Sub(pair),
                }),
                "*" => Ok(Monkey {
                    name,
                    value: MonkeyValue::Mul(pair),
                }),
                _ => Ok(Monkey {
                    name,
                    value: MonkeyValue::Div(pair),
                }),
            }
        }
    }
}

#[derive(Debug, Clone)]
struct MonkeyGroup {
    solved: HashMap<String, Monkey>,
    to_solve: VecDeque<Monkey>,
}

impl MonkeyGroup {
    fn solve(&mut self) {
        while !self.to_solve.is_empty() {
            let m: Monkey = self.to_solve.pop_front().unwrap();
            if let Some(solved) = self.solve_monkey(&m) {
                self.solved.insert(solved.name.clone(), solved);
            } else {
                self.to_solve.push_back(m);
            }
        }
    }

    fn solve_until_human(&mut self) {
        self.solved.remove(HUMAN);
        let mut size = self.to_solve.len();
        let mut count: usize = 0;
        while !self.to_solve.is_empty() {
            let m: Monkey = self.to_solve.pop_front().unwrap();
            if let Some(solved) = self.solve_monkey(&m) {
                self.solved.insert(solved.name.clone(), solved);
                count = 0;
                size = self.to_solve.len();
            } else {
                self.to_solve.push_back(m);
                count += 1;
            }
            if count == size {
                break;
            }
        }
    }

    fn solve_part_2(&mut self) -> isize {
        while !self.to_solve.is_empty() {
            let m: Monkey = self.to_solve.pop_front().unwrap();
            if m.name == ROOT {
                if let Some((name, value)) = self.solve_reverse_root(&m) {
                    self.solved.insert(
                        name.clone(),
                        Monkey {
                            name: name.clone(),
                            value: MonkeyValue::Val(value),
                        },
                    );
                } else {
                    self.to_solve.push_back(m);
                }
            } else if let Some((name, value)) = self.solve_reverse_monkey(&m) {
                self.solved.insert(
                    name.clone(),
                    Monkey {
                        name: name.clone(),
                        value: MonkeyValue::Val(value),
                    },
                );
            } else {
                self.to_solve.push_back(m);
            }
        }
        let humn: &Monkey = self.solved.get(HUMAN).unwrap();
        humn.value().unwrap()
    }

    fn get_monkeys(&self, left: &str, right: &str) -> Option<(isize, isize)> {
        let l = self.solved.get(left);
        let r = self.solved.get(right);
        match (l, r) {
            (Some(m_l), Some(m_r)) => match (m_l.value(), m_r.value()) {
                (Some(v_l), Some(v_r)) => Some((v_l, v_r)),
                _ => None,
            },
            _ => None,
        }
    }

    fn get_monkey_value(&self, name: &str) -> Option<isize> {
        if let Some(m) = self.solved.get(name) {
            m.value()
        } else {
            None
        }
    }

    fn solve_reverse_root(&self, root: &Monkey) -> Option<(String, isize)> {
        match &root.value {
            MonkeyValue::Add((left, right)) => {
                match (self.get_monkey_value(left), self.get_monkey_value(right)) {
                    (Some(v), None) => Some((right.to_string(), v)),
                    (None, Some(v)) => Some((left.to_string(), v)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn solve_monkey(&self, monkey: &Monkey) -> Option<Monkey> {
        match &monkey.value {
            MonkeyValue::Val(_) => None,
            MonkeyValue::Add((left, right)) => {
                if let Some((v_l, v_r)) = self.get_monkeys(left, right) {
                    let v = v_l + v_r;
                    Some(Monkey {
                        name: monkey.name.clone(),
                        value: MonkeyValue::Val(v),
                    })
                } else {
                    None
                }
            }
            MonkeyValue::Sub((left, right)) => {
                if let Some((v_l, v_r)) = self.get_monkeys(left, right) {
                    let v = v_l - v_r;
                    Some(Monkey {
                        name: monkey.name.clone(),
                        value: MonkeyValue::Val(v),
                    })
                } else {
                    None
                }
            }
            MonkeyValue::Mul((left, right)) => {
                if let Some((v_l, v_r)) = self.get_monkeys(left, right) {
                    let v = v_l * v_r;
                    Some(Monkey {
                        name: monkey.name.clone(),
                        value: MonkeyValue::Val(v),
                    })
                } else {
                    None
                }
            }
            MonkeyValue::Div((left, right)) => {
                if let Some((v_l, v_r)) = self.get_monkeys(left, right) {
                    let v = v_l / v_r;
                    Some(Monkey {
                        name: monkey.name.clone(),
                        value: MonkeyValue::Val(v),
                    })
                } else {
                    None
                }
            }
        }
    }
    fn solve_reverse_monkey(&self, monkey: &Monkey) -> Option<(String, isize)> {
        if let Some(v_curr) = self.get_monkey_value(&monkey.name) {
            match &monkey.value {
                MonkeyValue::Val(_) => None,
                MonkeyValue::Add((left, right)) => {
                    match (self.get_monkey_value(left), self.get_monkey_value(right)) {
                        (Some(v), None) => Some((right.to_string(), v_curr - v)),
                        (None, Some(v)) => Some((left.to_string(), v_curr - v)),
                        _ => None,
                    }
                }
                MonkeyValue::Sub((left, right)) => {
                    match (self.get_monkey_value(left), self.get_monkey_value(right)) {
                        (Some(v), None) => Some((right.to_string(), v - v_curr)),
                        (None, Some(v)) => Some((left.to_string(), v + v_curr)),
                        _ => None,
                    }
                }
                MonkeyValue::Mul((left, right)) => {
                    match (self.get_monkey_value(left), self.get_monkey_value(right)) {
                        (Some(v), None) => Some((right.to_string(), v_curr / v)),
                        (None, Some(v)) => Some((left.to_string(), v_curr / v)),
                        _ => None,
                    }
                }
                MonkeyValue::Div((left, right)) => {
                    match (self.get_monkey_value(left), self.get_monkey_value(right)) {
                        (Some(v), None) => Some((right.to_string(), v_curr / v)),
                        (None, Some(v)) => Some((left.to_string(), v * v_curr)),
                        _ => None,
                    }
                }
            }
        } else {
            None
        }
    }
}

impl FromStr for MonkeyGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (solved, to_solve): (Vec<Monkey>, Vec<Monkey>) = s
            .lines()
            .map(|l| l.parse::<Monkey>().unwrap())
            .partition(|m| matches!(m.value, MonkeyValue::Val(_)));

        let solved: HashMap<String, Monkey> =
            solved.into_iter().map(|m| (m.name.clone(), m)).collect();
        let to_solve: VecDeque<Monkey> = to_solve.into_iter().collect();
        Ok(MonkeyGroup { solved, to_solve })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_21.txt").expect("Cannot open input file");
    let mut group: MonkeyGroup = s.parse().unwrap();
    let mut first_group = group.clone();
    first_group.solve();
    let resp = first_group.solved.get(ROOT).unwrap().value().unwrap();
    println!("Part1: The root monkey will yell {resp}");

    group.solve_until_human();
    let resp: isize = group.solve_part_2();
    println!("Part2: You have to yell the number {resp}");
    println!("Computing time: {:?}", now.elapsed());
}
#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32";

    #[test]
    fn part_1() {
        let mut group: MonkeyGroup = INPUT.parse().unwrap();
        group.solve();
        let resp = group.solved.get(ROOT).unwrap().value().unwrap();
        assert_eq!(resp, 152);
    }

    #[test]
    fn part_2() {
        let mut group: MonkeyGroup = INPUT.parse().unwrap();
        group.solve_until_human();
        let resp: isize = group.solve_part_2();
        assert_eq!(resp, 301);
    }
}
