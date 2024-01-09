use std::collections::HashSet;
use std::str::FromStr;

struct Inventory {
    ids: Vec<String>,
}

impl Inventory {
    fn all_common_but_one(a: &str, b: &str) -> Option<String> {
        let mut c_a = a.chars();
        let mut c_b = b.chars();

        let mut pos: Option<usize> = None;

        for i in 0..a.len() {
            match (c_a.next().unwrap(), c_b.next().unwrap(), pos) {
                (x, y, None) if x != y => pos = Some(i),
                (x, y, _) if x != y => return None,
                _ => (),
            }
        }
        let p = pos.unwrap();
        Some(format!("{}{}", &a[..p], &a[p + 1..]))
    }

    fn find_common(&self) -> Option<String> {
        for i in 0..(self.ids.len() - 1) {
            for j in i + 1..self.ids.len() {
                if let Some(common) = Self::all_common_but_one(&self.ids[i], &self.ids[j]) {
                    return Some(common);
                }
            }
        }
        None
    }

    fn double_triple(s: &str) -> (bool, bool) {
        let mut single: HashSet<char> = HashSet::new();
        let mut double: HashSet<char> = HashSet::new();
        let mut triple: HashSet<char> = HashSet::new();
        let mut quad: HashSet<char> = HashSet::new();

        s.chars().for_each(|c| {
            if !single.insert(c) && !double.insert(c) && !triple.insert(c) {
                quad.insert(c);
            }
        });
        let double_no_triple: HashSet<&char> = double.difference(&triple).collect();
        let triple_no_quad: HashSet<&char> = triple.difference(&quad).collect();
        (!double_no_triple.is_empty(), !triple_no_quad.is_empty())
    }
    fn checksum(&self) -> usize {
        let (doubles, triples): (usize, usize) =
            self.ids
                .iter()
                .fold((0, 0), |(d, t), id| match Self::double_triple(id) {
                    (true, true) => (d + 1, t + 1),
                    (true, false) => (d + 1, t),
                    (false, true) => (d, t + 1),
                    (false, false) => (d, t),
                });

        doubles * triples
    }
}

impl FromStr for Inventory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ids: Vec<String> = s.lines().map(|l| l.to_string()).collect();

        Ok(Inventory { ids })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_02.txt").expect("Cannot open input file");
    let inventory: Inventory = s.parse().unwrap();

    println!(
        "Part1: The checksum of the inventory is {}",
        inventory.checksum()
    );
    println!(
        "Part2: The common letters between the two correct box IDs are {}",
        inventory.find_common().unwrap()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "abcdef
bababc
abbcde
abcccd
aabcdd
abcdee
ababab
";

    const EXAMPLE_2: &str = "abcde
fghij
klmno
pqrst
fguij
axcye
wvxyz
";

    #[test]
    fn part_1() {
        let inventory: Inventory = EXAMPLE_1.parse().unwrap();
        assert_eq!(inventory.checksum(), 12);
    }
    #[test]
    fn part_2() {
        let inventory: Inventory = EXAMPLE_2.parse().unwrap();
        assert_eq!(inventory.find_common(), Some("fgij".to_string()));
    }
}
