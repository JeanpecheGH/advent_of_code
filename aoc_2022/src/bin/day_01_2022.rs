use itertools::Itertools;

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_01.txt").expect("Cannot open input file");
    let words: Vec<&str> = s.lines().collect();

    let elves: Vec<&[&str]> = words.split(|w| w.is_empty()).collect();
    let calories: Vec<usize> = elves
        .iter()
        .map(|&slice| slice.iter().map(|&w| w.parse::<usize>().unwrap()).sum())
        .collect();
    let biggest_elf: usize = calories.iter().max().cloned().unwrap();

    println!("Part1: The elf carrying the more calories has {biggest_elf} calories");

    let biggest_three_elves: usize = calories.iter().sorted().rev().take(3).sum();
    println!(
        "Part2: The three elves carrying the more calories has {biggest_three_elves} calories"
    );
}
