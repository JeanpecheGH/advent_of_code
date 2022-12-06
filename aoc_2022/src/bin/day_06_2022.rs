use itertools::Itertools;

const FOUR: usize = 4;
const FOURTEEN: usize = 14;

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_06.txt").expect("Cannot open input file");

    let chars: Vec<char> = s.chars().collect();
    let idx = chars
        .windows(FOUR)
        .enumerate()
        .find_map(|(idx, four)| {
            if four.iter().all_unique() {
                Some(idx + FOUR)
            } else {
                None
            }
        })
        .unwrap();

    println!("Part1: The first start-of-packet is after {} chars", idx);

    let idx_2 = chars
        .windows(FOURTEEN)
        .enumerate()
        .find_map(|(idx, fourteen)| {
            if fourteen.iter().all_unique() {
                Some(idx + FOURTEEN)
            } else {
                None
            }
        })
        .unwrap();

    println!("Part2: The first start-of-message is after {} chars", idx_2);
}
