use std::collections::HashMap;

#[derive(Debug, Clone)]
struct BagRule {
    color: String,
    contains: Vec<(String, usize)>,
}

impl BagRule {
    fn contains(&self, color_cache: &mut HashMap<String, bool>, other_color: &str) -> Option<bool> {
        if self.direct_contains(other_color) {
            color_cache.insert(self.color.clone(), true);
            Some(true)
        } else {
            let opt_b: Option<bool> = self
                .contains
                .iter()
                .map(|(c, _)| color_cache.get(c))
                .try_fold(false, |acc, opt| match (acc, opt) {
                    (true, _) => Some(true),
                    (_, Some(true)) => Some(true),
                    (_, None) => None,
                    _ => Some(false),
                });
            opt_b.iter().for_each(|&b| {
                color_cache.insert(self.color.clone(), b);
            });
            opt_b
        }
    }

    fn direct_contains(&self, other_color: &str) -> bool {
        self.contains.iter().any(|(c, _)| c.eq(other_color))
    }

    fn nb_contains(&self, count_cache: &mut HashMap<String, usize>) -> Option<usize> {
        let opt_count: Option<usize> = self
            .contains
            .iter()
            .map(|(c, n)| (count_cache.get(c), n))
            .try_fold(0, |acc, (opt, n)| match (acc, opt) {
                (a, Some(&b)) => Some(a + n * (b + 1)),
                _ => None,
            });
        opt_count.iter().for_each(|&c| {
            count_cache.insert(self.color.clone(), c);
        });
        opt_count
    }
}

const COLOR: &str = "shiny gold";

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_07.txt").expect("Cannot open input file");

    let mut bags: Vec<BagRule> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            let color: String = format!("{} {}", words[0], words[1]);
            match words.len() {
                7 => BagRule {
                    color,
                    contains: Vec::new(),
                },
                n => {
                    let mut contains: Vec<(String, usize)> = Vec::new();
                    for i in 1..n / 4 {
                        let nb: usize = words[4 * i].parse().unwrap();
                        let sub_color: String =
                            format!("{} {}", words[4 * i + 1], words[4 * i + 2]);
                        contains.push((sub_color, nb));
                    }
                    BagRule { color, contains }
                }
            }
        })
        .collect();

    let mut bags_part1 = bags.clone();
    let mut color_cache: HashMap<String, bool> = HashMap::new();
    let mut color_count: usize = 0;
    while let Some(bag) = bags_part1.pop() {
        if let Some(b) = bag.contains(&mut color_cache, COLOR) {
            if b {
                color_count += 1;
            }
        } else {
            bags_part1.push(bag);
            bags_part1.rotate_right(1);
        }
    }

    println!("Part1: {color_count} different bags can contain a {COLOR} bag");

    let mut count_cache: HashMap<String, usize> = HashMap::new();

    while let Some(bag) = bags.pop() {
        // if let Some(c) = bag.nb_contains(&mut count_cache) {
        //     count_count += c;
        if bag.nb_contains(&mut count_cache).is_none() {
            bags.push(bag);
            bags.rotate_right(1);
        }
    }

    println!(
        "Part2: A {COLOR} bag contains {} total bags",
        count_cache.get(COLOR).unwrap()
    );
}
