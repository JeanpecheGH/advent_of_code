use std::collections::HashSet;

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_03.txt").expect("Cannot open input file");

    let mut houses: HashSet<(i32, i32)> = HashSet::new();
    let mut pos: (i32, i32) = (0, 0);

    houses.insert(pos);
    for c in s.chars() {
        pos = match c {
            '<' => (pos.0 - 1, pos.1),
            '>' => (pos.0 + 1, pos.1),
            '^' => (pos.0, pos.1 + 1),
            'v' => (pos.0, pos.1 - 1),
            _ => pos,
        };
        houses.insert(pos);
    }

    println!("Part1: Santa visited {} houses", houses.len());

    houses.clear();
    let mut santa_pos: (i32, i32) = (0, 0);
    let mut robot_pos: (i32, i32) = (0, 0);

    houses.insert(santa_pos);
    for (i, c) in s.chars().enumerate() {
        if i % 2 == 0 {
            santa_pos = match c {
                '<' => (santa_pos.0 - 1, santa_pos.1),
                '>' => (santa_pos.0 + 1, santa_pos.1),
                '^' => (santa_pos.0, santa_pos.1 + 1),
                'v' => (santa_pos.0, santa_pos.1 - 1),
                _ => santa_pos,
            };
            houses.insert(santa_pos);
        } else {
            robot_pos = match c {
                '<' => (robot_pos.0 - 1, robot_pos.1),
                '>' => (robot_pos.0 + 1, robot_pos.1),
                '^' => (robot_pos.0, robot_pos.1 + 1),
                'v' => (robot_pos.0, robot_pos.1 - 1),
                _ => robot_pos,
            };
            houses.insert(robot_pos);
        }
    }

    println!(
        "Part2: Santa and Robot Santa visited {} houses",
        houses.len()
    )
}
