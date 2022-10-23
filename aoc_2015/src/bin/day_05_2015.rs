fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_05.txt").expect("Cannot open input file");

    let nb_nice_strings: Option<(u16, u16)> = lines
        .map(|res_l| {
            let l = res_l.unwrap();
            (is_nice(l.as_str()) as u16, is_nice_2(l.as_str()) as u16)
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1));

    println!(
        "Part1: There are {} nice strings",
        nb_nice_strings.unwrap().0
    );
    println!(
        "Part2: There are now {} nice strings",
        nb_nice_strings.unwrap().1
    );
}

//Part 1
fn is_nice(s: &str) -> bool {
    has_three_vowels(s) && contains_double(s) && !has_naughty_pair(s)
}

fn has_three_vowels(s: &str) -> bool {
    let vowels = "aeiou";
    s.chars().filter(|&c| vowels.contains(c)).count() >= 3
}

fn contains_double(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    v.windows(2).any(|pair| pair[0] == pair[1])
}

fn has_naughty_pair(s: &str) -> bool {
    let naughty_pairs = ["ab", "cd", "pq", "xy"];
    naughty_pairs.iter().any(|&p| s.contains(p))
}

//Part 2
fn is_nice_2(s: &str) -> bool {
    double_double(s) && separated_pair(s)
}

fn double_double(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    v.windows(2).enumerate().any(|(index, pair)| {
        let pair_str = format!("{}{}", pair[0], pair[1]);
        s[(index + 2)..].contains(pair_str.as_str())
    })
}

fn separated_pair(s: &str) -> bool {
    let v: Vec<char> = s.chars().collect();
    v.windows(3).any(|pair| pair[0] == pair[2])
}
