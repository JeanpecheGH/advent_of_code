use std::str::FromStr;

#[derive(Debug, Eq, PartialEq, Clone)]
enum Exp {
    Value(usize),
    Mul,
    Add,
    Exp(Vec<Exp>),
}

impl Exp {
    fn value(&self) -> usize {
        match self {
            Exp::Value(n) => *n,
            //Left operation takes precendence
            Exp::Exp(values) => {
                let mut res = values[0].value();
                for i in (1..values.len()).step_by(2) {
                    match values[i] {
                        Exp::Add => res += values[i + 1].value(),
                        Exp::Mul => res *= values[i + 1].value(),
                        _ => (),
                    }
                }
                res
            }
            _ => 0,
        }
    }

    fn advanced_value(&self) -> usize {
        match self {
            Exp::Value(n) => *n,
            //Add operation takes precendence over Mul
            Exp::Exp(values) => {
                let mut copy: Vec<Exp> = values.clone();
                while copy.contains(&Exp::Add) {
                    let pos: usize = copy.iter().position(|e| *e == Exp::Add).unwrap();
                    copy[pos - 1] =
                        Exp::Value(copy[pos - 1].advanced_value() + copy[pos + 1].advanced_value());
                    copy.remove(pos + 1);
                    copy.remove(pos);
                }
                copy.iter()
                    .filter(|e| **e != Exp::Mul)
                    .map(|e| e.advanced_value())
                    .product()
            }
            _ => 0,
        }
    }
}

impl FromStr for Exp {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn exp_end(l: &str) -> usize {
            let mut level: usize = 0;
            let mut end: Option<usize> = None;
            for (i, c) in l.chars().enumerate() {
                match (c, level, end) {
                    ('(', _, _) => level += 1,
                    (')', 1, None) => end = Some(i),
                    (')', _, None) => level -= 1,
                    _ => (),
                }
            }
            end.unwrap()
        }

        let len: usize = s.len();
        let mut idx: usize = 0;
        let mut exps: Vec<Self> = Vec::new();
        while idx < len {
            match s.as_bytes().get(idx) {
                Some(b'(') => {
                    let end = exp_end(&s[idx..]) + idx;
                    exps.push(s[idx + 1..end].parse().unwrap());
                    idx = end + 2;
                }
                Some(b'+') => {
                    exps.push(Exp::Add);
                    idx += 2;
                }
                Some(b'*') => {
                    exps.push(Exp::Mul);
                    idx += 2;
                }
                _ => {
                    let end = s[idx..]
                        .chars()
                        .position(|c| !c.is_ascii_digit())
                        .unwrap_or(len)
                        + idx;
                    let value: usize = if end < len {
                        s[idx..end].parse().unwrap()
                    } else {
                        s[idx..].parse().unwrap()
                    };
                    exps.push(Exp::Value(value));
                    idx = end + 1;
                }
            }
        }
        Ok(Exp::Exp(exps))
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_18.txt").expect("Cannot open input file");
    let exps: Vec<Exp> = s.lines().map(|l| l.parse().unwrap()).collect();
    let sums: usize = exps.iter().map(|e| e.value()).sum();
    println!(
        "Part1: When adding the results of all the lines, we obtain {sums}"

    );
    let sums: usize = exps.iter().map(|e| e.advanced_value()).sum();
    println!(
        "Part2: When adding the results of all the lines following the advanced precedence levels, we obtain {sums}"

    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1() {
        let s: &str = "1 + 2 * 3 + 4 * 5 + 6";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 71);
        assert_eq!(exp.advanced_value(), 231);
    }
    #[test]
    fn test_2() {
        let s: &str = "1 + (2 * 3) + (4 * (5 + 6))";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 51);
        assert_eq!(exp.advanced_value(), 51);
    }
    #[test]
    fn test_3() {
        let s: &str = "2 * 3 + (4 * 5)";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 26);
        assert_eq!(exp.advanced_value(), 46);
    }
    #[test]
    fn test_4() {
        let s: &str = "5 + (8 * 3 + 9 + 3 * 4 * 3)";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 437);
        assert_eq!(exp.advanced_value(), 1445);
    }
    #[test]
    fn test_5() {
        let s: &str = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 12240);
        assert_eq!(exp.advanced_value(), 669060);
    }
    #[test]
    fn test_6() {
        let s: &str = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let exp: Exp = s.parse().unwrap();
        assert_eq!(exp.value(), 13632);
        assert_eq!(exp.advanced_value(), 23340);
    }
}
