struct Rule {
    min: usize,
    max: usize,
    c: char,
    pwd: String,
}

impl Rule {
    fn is_valid(&self) -> bool {
        let nb = self.pwd.chars().filter(|&c| c == self.c).count();
        nb >= self.min && nb <= self.max
    }

    fn is_valid_2(&self) -> bool {
        let chars: Vec<char> = self.pwd.chars().collect();
        (chars[self.min - 1] == self.c) ^ (chars[self.max - 1] == self.c)
    }
}

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_02.txt").expect("Cannot open input file");

    let rules: Vec<Rule> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            let limits: Vec<&str> = words[0].split('-').collect();
            let min: usize = limits[0].parse().unwrap();
            let max: usize = limits[1].parse().unwrap();
            let c: char = words[1].chars().next().unwrap();
            let pwd = words[2].to_string();
            Rule { min, max, c, pwd }
        })
        .collect();

    let nb_valid: usize = rules.iter().filter(|r| r.is_valid()).count();
    println!("Part1: The number of valid password is {nb_valid}");

    let nb_valid_2: usize = rules.iter().filter(|r| r.is_valid_2()).count();
    println!("Part1: With the new rule, the number of valid password is {nb_valid_2}");
}
