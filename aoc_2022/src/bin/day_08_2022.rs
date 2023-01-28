const SIZE: usize = 99;

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_08.txt").expect("Cannot open input file");

    let tree_grid: Vec<Vec<i8>> = s
        .lines()
        .map(|s| s.chars().map(|c| c.to_digit(10).unwrap() as i8).collect())
        .collect();

    let mut visible_grid: [[bool; SIZE]; SIZE] = [[false; SIZE]; SIZE];

    //From left/right
    for j in 0..SIZE {
        let mut max_left: i8 = -1;
        for i in 0..SIZE {
            if tree_grid[j][i] > max_left {
                max_left = tree_grid[j][i];
                visible_grid[j][i] = true;
            }
        }
        let mut max_right: i8 = -1;
        for i in (0..SIZE).rev() {
            if tree_grid[j][i] > max_right {
                max_right = tree_grid[j][i];
                visible_grid[j][i] = true;
            }
        }
    }

    //From top/bottom
    for i in 0..SIZE {
        let mut max_top: i8 = -1;
        for j in 0..SIZE {
            if tree_grid[j][i] > max_top {
                max_top = tree_grid[j][i];
                visible_grid[j][i] = true;
            }
        }
        let mut max_bot: i8 = -1;
        for j in (0..SIZE).rev() {
            if tree_grid[j][i] > max_bot {
                max_bot = tree_grid[j][i];
                visible_grid[j][i] = true;
            }
        }
    }

    let visible_count: usize = visible_grid
        .iter()
        .map(|row| row.iter().filter(|&&b| b).count())
        .sum();
    println!("Part1: The biggest number of tree visible is {visible_count}");

    //Transpose the tree grid to help searching in columns
    let mut transposed_trees: Vec<Vec<i8>> = vec![vec![0; SIZE]; SIZE];
    for (j, row) in tree_grid.iter().enumerate() {
        for (i, &tree) in row.iter().enumerate() {
            transposed_trees[i][j] = tree
        }
    }

    //Compute scenic scores for every tree
    let mut scenic_scores: [[usize; SIZE]; SIZE] = [[0; SIZE]; SIZE];
    for (j, row) in tree_grid.iter().enumerate() {
        for (i, &tree) in row.iter().enumerate() {
            let left: usize = nb_visible(&row[0..i], tree, true);
            let right: usize = nb_visible(&row[i + 1..SIZE], tree, false);
            let top: usize = nb_visible(&transposed_trees[i][0..j], tree, true);
            let bot: usize = nb_visible(&transposed_trees[i][j + 1..SIZE], tree, false);
            scenic_scores[j][i] = left * right * top * bot;
        }
    }

    let max_scenic_score: &usize = scenic_scores
        .iter()
        .map(|row| row.iter().max().unwrap())
        .max()
        .unwrap();
    println!("Part2: The best scenic score is {max_scenic_score}");
}

fn nb_visible(trees: &[i8], height: i8, reverse: bool) -> usize {
    let opt = if reverse {
        trees
            .iter()
            .rposition(|&tree| tree >= height)
            .map(|p| trees.len() - p)
    } else {
        trees.iter().position(|&tree| tree >= height).map(|p| p + 1)
    };
    opt.unwrap_or(trees.len())
}
