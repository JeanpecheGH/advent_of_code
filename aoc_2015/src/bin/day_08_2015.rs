fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_08.txt").expect("Cannot open input file");

    let part1_diff: Option<(u16, u16)> = lines
        .map(|l| {
            let line = l.unwrap();
            let current_size = line.len() as u16;
            (
                current_size - memory_size(&line, 0) + 2,
                encoded_size(&line) - current_size,
            )
        })
        .reduce(|a, b| (a.0 + b.0, a.1 + b.1));

    println!("Part1: The difference is {}", part1_diff.unwrap().0);
    println!("Part2: The difference is now {}", part1_diff.unwrap().1);
}

fn memory_size(s: &str, acc: u16) -> u16 {
    match s.len() {
        0 => acc,
        x if x > 1 => {
            let mut chars = s.chars();
            match (chars.next(), chars.next()) {
                (Some('\\'), Some('\\')) => memory_size(&s[2..], acc + 1),
                (Some('\\'), Some('\"')) => memory_size(&s[2..], acc + 1),
                (Some('\\'), Some('x')) => memory_size(&s[4..], acc + 1),
                _ => memory_size(&s[1..], acc + 1),
            }
        }
        _ => memory_size(&s[1..], acc + 1),
    }
}

fn encoded_size(s: &str) -> u16 {
    s.chars()
        .map(|c| match c {
            '\\' | '\"' => 2,
            _ => 1,
        })
        .sum::<u16>()
        + 2
}
