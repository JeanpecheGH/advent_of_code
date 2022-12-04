struct TilesRow {
    tiles: Vec<bool>,
}

impl TilesRow {
    fn next_row(&self) -> Self {
        //First tile
        let mut tiles: Vec<bool> = vec![self.tiles[1]];
        let mut mid_tiles: Vec<bool> = self
            .tiles
            .windows(3)
            .map(|trio| trio[0] != trio[2])
            .collect();
        tiles.append(&mut mid_tiles);
        //Last tile
        tiles.push(self.tiles[self.tiles.len() - 2]);
        TilesRow { tiles }
    }

    fn safe_tiles(&self) -> usize {
        self.tiles.iter().filter(|&&t| !t).count()
    }

    fn print_row(&self) {
        let s: String = self
            .tiles
            .iter()
            .map(|&t| if t { '^' } else { '.' })
            .collect();
        println!("{s}");
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_18.txt").expect("Cannot open input file");
    let tiles: Vec<bool> = s.chars().map(|c| c == '^').collect();

    let now = std::time::Instant::now();
    let mut row = TilesRow {
        tiles: tiles.clone(),
    };
    println!("Starting row :");
    row.print_row();

    let mut safe_tiles = row.safe_tiles();
    for _ in 1..40 {
        row = row.next_row();
        safe_tiles += row.safe_tiles();
    }
    println!(
        "Part1: there are {} safe tiles in a room with 40 rows, found in {:?}",
        safe_tiles,
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let mut row = TilesRow { tiles };

    let mut safe_tiles = row.safe_tiles();
    for _ in 1..400000 {
        row = row.next_row();
        safe_tiles += row.safe_tiles();
    }
    println!(
        "Part2: there are {} safe tiles in a room with 400000 rows, found in {:?}",
        safe_tiles,
        now.elapsed()
    );
}
