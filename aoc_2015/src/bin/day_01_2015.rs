use util;

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_01.txt").expect("Cannot open input file");

    let santa: (i32, Option<usize>) =
        s.chars()
            .enumerate()
            .fold((0, None), |(floor, opt_pos), (index, c)| {
                match (floor, opt_pos, c) {
                    (0, None, ')') => (-1, Some(index + 1)),
                    (f, _, '(') => (f + 1, opt_pos),
                    (f, _, ')') => (f - 1, opt_pos),
                    _ => (floor, opt_pos),
                }
            });

    println!("Part1: Santa ends up at floor n°{}", santa.0);
    println!(
        "Part2: Santa enters basement at position {}",
        santa.1.unwrap()
    );
}
