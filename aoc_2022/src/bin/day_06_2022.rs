use itertools::Itertools;

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_06.txt").expect("Cannot open input file");

    let chars: Vec<char> = s.chars().collect();
    let idx = first_marker(&chars, 4);

    println!("Part1: The first start-of-packet is after {} chars", idx);

    let idx_2 = first_marker(&chars, 14);

    println!("Part2: The first start-of-message is after {} chars", idx_2);
}

fn first_marker(chars: &[char], marker_size: usize) -> usize {
    chars
        .windows(marker_size)
        .enumerate()
        .find_map(|(idx, four)| {
            if four.iter().all_unique() {
                Some(idx + marker_size)
            } else {
                None
            }
        })
        .unwrap()
}
