const HEIGHT: usize = 200;
const WIDTH: usize = 150;

type Pos = (usize, usize);

enum Orient {
    North,
    South,
    East,
    West,
}

impl Orient {
    fn right(&self) -> Self {
        match self {
            Orient::North => Orient::East,
            Orient::South => Orient::West,
            Orient::East => Orient::South,
            Orient::West => Orient::North,
        }
    }

    fn left(&self) -> Self {
        match self {
            Orient::North => Orient::West,
            Orient::South => Orient::East,
            Orient::East => Orient::North,
            Orient::West => Orient::South,
        }
    }
}

struct Labyrinth {
    grid: [[Option<Bool>; WIDTH]; HEIGHT],
    pos: Pos,
    orient: Orient,
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_22.txt").expect("Cannot open input file");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn part_1() {
        assert_eq!(6032, 6032);
    }

    #[test]
    fn part_2() {
        assert_eq!(1, 1);
    }
}
