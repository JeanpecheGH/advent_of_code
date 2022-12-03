fn main() {
    let lines = util::file_as_lines("aoc_2022/input/day_03.txt").expect("Cannot open input file");

    let words: Vec<Vec<char>> = lines
        .map(|l| {
            let s: String = l.unwrap();
            s.chars().collect()
        })
        .collect();

    let prio: Vec<usize> = words
        .iter()
        .map(|chars| {
            let (start, end): (&[char], &[char]) = chars.split_at(chars.len() / 2);
            let double: char = start.iter().find(|&c| end.contains(c)).cloned().unwrap();
            char_to_prio(double)
        })
        .collect();

    let prio_sum: usize = prio.iter().sum();
    println!(
        "Part1: The sum of the priority of the misplaced items is {}",
        prio_sum
    );

    let badges: Vec<usize> = words
        .chunks(3)
        .map(|trio| {
            let triple: char = trio[0]
                .iter()
                .find(|&c| trio[1].contains(c) && trio[2].contains(c))
                .cloned()
                .unwrap();
            char_to_prio(triple)
        })
        .collect();
    let badge_sum: usize = badges.iter().sum();
    println!(
        "Part2: The sum of the priority of the lost badges is {}",
        badge_sum
    );
}

fn char_to_prio(c: char) -> usize {
    match c {
        ('a'..='z') => (c as u8 - b'a' + 1) as usize,
        ('A'..='Z') => (c as u8 - b'A' + 27) as usize,
        _ => 0,
    }
}
