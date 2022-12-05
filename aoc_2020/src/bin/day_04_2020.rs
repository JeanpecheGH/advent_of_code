use std::collections::HashMap;

#[derive(Debug)]
struct Document {
    infos: HashMap<String, String>,
}

impl Document {
    fn contains_fields(&self, fields: &[&str]) -> bool {
        fields.iter().all(|&f| self.infos.contains_key(f))
    }

    fn is_valid(&self, fields: &[&str]) -> bool {
        self.contains_fields(fields)
            && self
                .infos
                .iter()
                .all(|(key, value)| Self::is_valid_field(key, value))
    }

    fn is_valid_field(key: &str, value: &str) -> bool {
        match key {
            "byr" => {
                let y: usize = value.parse().unwrap_or(0);
                (1920..=2002).contains(&y)
            }
            "iyr" => {
                let y: usize = value.parse().unwrap_or(0);
                (2010..=2020).contains(&y)
            }
            "eyr" => {
                let y: usize = value.parse().unwrap_or(0);
                (2020..=2030).contains(&y)
            }
            "hgt" => {
                let len = value.len();
                match &value[len - 2..] {
                    "cm" => {
                        let cms: usize = value[..len - 2].parse().unwrap_or(0);
                        (150..=193).contains(&cms)
                    }
                    "in" => {
                        let ins: usize = value[..len - 2].parse().unwrap_or(0);
                        (59..=76).contains(&ins)
                    }
                    _ => false,
                }
            }
            "hcl" => {
                let code: Vec<char> = value.strip_prefix('#').unwrap_or("bad").chars().collect();
                code.iter().all(|c| c.is_ascii_hexdigit()) && code.len() == 6
            }
            "ecl" => ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&value),
            "pid" => value.chars().all(|c| c.is_ascii_digit()) && value.len() == 9,
            _ => true,
        }
    }
}

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_04.txt").expect("Cannot open input file");
    let lines: Vec<&str> = s.lines().collect();

    let docs: Vec<Document> = lines
        .split(|l| l.is_empty())
        .map(|group| {
            let mut infos: HashMap<String, String> = HashMap::new();
            group.iter().for_each(|line| {
                let words: Vec<&str> = line.split_whitespace().collect();
                words.iter().for_each(|word| {
                    let pair: Vec<&str> = word.split(':').collect();
                    infos.insert(pair[0].to_string(), pair[1].to_string());
                });
            });
            Document { infos }
        })
        .collect();

    let mandatory_fields: Vec<&str> = vec!["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

    let nb_valid_docs: usize = docs
        .iter()
        .filter(|doc| doc.contains_fields(&mandatory_fields))
        .count();
    println!(
        "Part1: There are {} documents with the right fields",
        nb_valid_docs
    );

    let nb_valid_docs_2: usize = docs
        .iter()
        .filter(|doc| doc.is_valid(&mandatory_fields))
        .count();
    println!("Part2: There are {} valid documents", nb_valid_docs_2);
}
