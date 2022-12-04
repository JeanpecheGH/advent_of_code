fn main() {
    let s = util::file_as_string("aoc_2015/input/day_18.txt").expect("Cannot open input file");

    let grid: Vec<Vec<bool>> = s
        .lines()
        .map(|s| s.chars().map(|c| matches!(c, '#')).collect())
        .collect();

    let mut grid_1 = grid.clone();
    (0..100).for_each(|_| {
        grid_1 = step(&grid_1);
    });
    let nb_lights_1: usize = grid_1
        .into_iter()
        .map(|row| row.into_iter().filter(|&b| b).count())
        .sum();
    println!("Part1: There are {} lights on ", nb_lights_1);

    let mut grid_2 = grid;
    grid_2[0][0] = true;
    grid_2[0][99] = true;
    grid_2[99][0] = true;
    grid_2[99][99] = true;
    (0..100).for_each(|_| {
        grid_2 = step_fixed_corner(&grid_2);
    });
    let nb_lights_2: usize = grid_2
        .into_iter()
        .map(|row| row.into_iter().filter(|&b| b).count())
        .sum();
    println!("Part2: There are now {} lights on ", nb_lights_2);
}

fn step(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let new_grid: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &b)| {
                    matches!((nb_neighboors((x, y), grid), b), (2..=3, true) | (3, false))
                })
                .collect()
        })
        .collect();
    new_grid
}

fn step_fixed_corner(grid: &[Vec<bool>]) -> Vec<Vec<bool>> {
    let new_grid: Vec<Vec<bool>> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .map(|(x, &b)| {
                    matches!(
                        (x, y, nb_neighboors((x, y), grid), b),
                        (0, 0, _, _)
                            | (0, 99, _, _)
                            | (99, 0, _, _)
                            | (99, 99, _, _)
                            | (_, _, 2..=3, true)
                            | (_, _, 3, false)
                    )
                })
                .collect()
        })
        .collect();
    new_grid
}

fn nb_neighboors((x, y): (usize, usize), grid: &[Vec<bool>]) -> usize {
    neighboors((x as isize, y as isize), 0, grid.len())
        .iter()
        .map(|&(i, j)| grid[j][i])
        .filter(|&b| b)
        .count()
}

#[rustfmt::skip]
fn neighboors((x,y): (isize ,isize), min: usize, max: usize) -> Vec<(usize,usize)> {
    let nbrs = [
        (x-1, y+1), (x, y+1), (x+1, y+1),
        (x-1, y),               (x+1, y),
        (x-1, y-1), (x, y-1), (x+1, y-1),
    ];
    nbrs
        .into_iter()
        .map(|(x, y)| (x as usize, y as usize))
        .filter(|&(x,y)| x >= min && x < max && y >= min && y < max)
        .collect()
}
