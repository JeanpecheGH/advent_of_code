struct Seat {
    row: usize,
    col: usize,
}
impl Seat {
    fn new(path: &str) -> Self {
        let row_path: &str = &path[0..7];
        let col_path: &str = &path[7..];
        Seat {
            row: Self::n_from_path(row_path),
            col: Self::n_from_path(col_path),
        }
    }

    fn n_from_path(path: &str) -> usize {
        let mut min: usize = 0;
        let mut max: usize = 2usize.pow(path.len() as u32) - 1;
        for c in path.chars() {
            match c {
                'F' | 'L' => max = (min + max) / 2,
                'B' | 'R' => min = (min + max) / 2 + 1,
                _ => (),
            }
        }
        min
    }

    fn id(&self) -> usize {
        self.row * 8 + self.col
    }
}

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_05.txt").expect("Cannot open input file");

    let seats: Vec<Seat> = s.lines().map(Seat::new).collect();

    let ids: Vec<usize> = seats.iter().map(|seat| seat.id()).collect();
    let highest_id: usize = ids.iter().max().cloned().unwrap();
    println!("Part1: The highest ID is {highest_id}");

    let lowest_id: usize = ids.iter().min().cloned().unwrap();
    let missing_ids: Vec<usize> = (lowest_id..=highest_id)
        .filter(|id| !ids.contains(id))
        .collect();
    println!("Part2: All missing IDs in this flight {missing_ids:?}");
}
