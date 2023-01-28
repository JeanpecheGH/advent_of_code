fn main() {
    let s = util::file_as_string("aoc_2020/input/day_03.txt").expect("Cannot open input file");
    let rows: Vec<Vec<bool>> = s
        .lines()
        .map(|s| s.chars().map(|c| c == '#').collect())
        .collect();

    let nb_trees_3_1: usize = trees_encountered(&rows, 3, 1);
    println!("Part1: We encounter {nb_trees_3_1} trees during the slide");
    let nb_trees_1_1: usize = trees_encountered(&rows, 1, 1);
    let nb_trees_5_1: usize = trees_encountered(&rows, 5, 1);
    let nb_trees_7_1: usize = trees_encountered(&rows, 7, 1);
    let nb_trees_1_2: usize = trees_encountered(&rows, 1, 2);
    let prod: usize = nb_trees_3_1 * nb_trees_1_1 * nb_trees_5_1 * nb_trees_7_1 * nb_trees_1_2;
    println!("Part2: The product of the number of trees encountered during all those different slides is {prod}");
}

fn trees_encountered(rows: &[Vec<bool>], horiz_speed: usize, vert_speed: usize) -> usize {
    let row_len: usize = rows[0].len();
    rows.iter()
        .enumerate()
        .filter(|&(idx, row)| {
            (idx % vert_speed == 0) && row[(idx * horiz_speed / vert_speed) % row_len]
        })
        .count()
}
