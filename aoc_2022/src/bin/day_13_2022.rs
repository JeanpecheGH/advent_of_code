use std::cmp::Ordering;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
enum Elem {
    Value(usize),
    List(Vec<Elem>),
}

impl PartialOrd for Elem {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Elem {
    fn cmp(&self, other: &Self) -> Ordering {
        match (self, other) {
            (Self::Value(a), Self::Value(b)) => a.cmp(b),
            (Self::Value(a), Self::List(_)) => Self::List(vec![Self::Value(*a)]).cmp(other),
            (Self::List(_), Self::Value(b)) => self.cmp(&Self::List(vec![Self::Value(*b)])),
            (Self::List(a), Self::List(b)) => a.cmp(b),
        }
    }
}

impl FromStr for Elem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn split_list(l: &str) -> Vec<&str> {
            let mut commas: Vec<isize> = vec![-1];
            let mut level: usize = 0;
            for (i, c) in l.chars().enumerate() {
                match (c, level) {
                    ('[', _) => level += 1,
                    (']', _) => level -= 1,
                    (',', 0) => commas.push(i as isize),
                    _ => (),
                }
            }
            commas.push(l.len() as isize);
            commas
                .windows(2)
                .map(|pair| {
                    let start = (pair[0] + 1) as usize;
                    let end = pair[1] as usize;
                    &l[start..end]
                })
                .collect()
        }

        let l: usize = s.len();
        if l == 0 {
            return Err(());
        }
        if *s.as_bytes().first().unwrap() == b'[' {
            let ll = split_list(&s[1..l - 1]);
            let values: Vec<Elem> = ll
                .iter()
                .filter(|w| !w.is_empty())
                .map(|w| w.parse().unwrap())
                .collect();
            Ok(Elem::List(values))
        } else {
            let value = s.parse::<usize>().unwrap();
            Ok(Elem::Value(value))
        }
    }
}
fn main() {
    let s = util::file_as_string("aoc_2022/input/day_13.txt").expect("Cannot open input file");

    let lines: Vec<Option<Elem>> = s
        .lines()
        .map(|l| {
            if l.is_empty() {
                None
            } else {
                Some(l.parse().unwrap())
            }
        })
        .collect();

    let pairs: Vec<(Elem, Elem)> = lines
        .split(|opt| opt.is_none())
        .map(|pair| (pair[0].clone().unwrap(), pair[1].clone().unwrap()))
        .collect();

    let sum_indexes: usize = pairs
        .iter()
        .enumerate()
        .filter_map(|(i, (one, two))| match one.cmp(two) {
            Ordering::Less => Some(i + 1),
            _ => None,
        })
        .sum();

    println!("Part1: {}", sum_indexes);

    let mut part_2_elems: Vec<Elem> = s.lines().flat_map(|l| l.parse()).collect();
    let div_1: Elem = "[[2]]".parse().unwrap();
    let div_2: Elem = "[[6]]".parse().unwrap();
    part_2_elems.push(div_1.clone());
    part_2_elems.push(div_2.clone());

    part_2_elems.sort();
    let prod_indexes: usize = part_2_elems
        .iter()
        .enumerate()
        .filter_map(|(i, e)| {
            if *e == div_1 || *e == div_2 {
                Some(i + 1)
            } else {
                None
            }
        })
        .product();

    println!("Part2: {}", prod_indexes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_vecs() {
        let a: Elem = "[1,1,3,1,1]".parse().unwrap();
        let b: Elem = "[1,1,5,1,1]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn nested_vecs() {
        let a: Elem = "[[1],[2,3,4]]".parse().unwrap();
        let b: Elem = "[[1],4]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn reverse_nested_vecs() {
        let a: Elem = "[9]".parse().unwrap();
        let b: Elem = "[[6,7,8]]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn shorter_vec() {
        let a: Elem = "[[4,4],4,4]".parse().unwrap();
        let b: Elem = "[[4,4],4,4,4]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn longer_vec() {
        let a: Elem = "[7,7,7,7]".parse().unwrap();
        let b: Elem = "[7,7,7]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn empty_vec() {
        let a: Elem = "[]".parse().unwrap();
        let b: Elem = "[3]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Less);
    }

    #[test]
    fn nested_empty_vecs() {
        let a: Elem = "[[[]]]".parse().unwrap();
        let b: Elem = "[[]]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }

    #[test]
    fn real_case() {
        let a: Elem = "[1,[2,[3,[4,[5,6,7]]]],8,9]".parse().unwrap();
        let b: Elem = "[1,[2,[3,[4,[5,6,0]]]],8,9]".parse().unwrap();
        assert_eq!(a.cmp(&b), Ordering::Greater);
    }
}
