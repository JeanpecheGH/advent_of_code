enum Command {
    Rect(usize, usize),
    RotateRow(usize, usize),
    RotateCol(usize, usize),
}

struct TinyLCD {
    pixels: [[bool; 50]; 6],
}

impl TinyLCD {
    fn process_command(&mut self, c: &Command) {
        match c {
            Command::Rect(x, y) => self.light_rect(*x, *y),
            Command::RotateRow(y, n) => self.rotate_row(*y, *n),
            Command::RotateCol(x, n) => self.rotate_col(*x, *n),
        }
    }

    fn light_rect(&mut self, x: usize, y: usize) {
        for row in self.pixels.iter_mut().take(y) {
            for pixel in row.iter_mut().take(x) {
                *pixel = true
            }
        }
    }

    fn rotate_row(&mut self, y: usize, n: usize) {
        self.pixels[y].rotate_right(n)
    }

    fn rotate_col(&mut self, x: usize, n: usize) {
        let mut col: [bool; 6] = [false; 6];
        for (index, row) in self.pixels.iter().enumerate() {
            col[index] = row[x];
        }
        col.rotate_right(n);
        for (index, row) in self.pixels.iter_mut().enumerate() {
            row[x] = col[index];
        }
    }

    fn lit_pixels(&self) -> usize {
        self.pixels
            .iter()
            .map(|row| row.iter().filter(|&&p| p).count())
            .sum()
    }

    fn display(&self) {
        self.pixels.iter().for_each(|row| {
            row.iter().enumerate().for_each(|(index, &p)| {
                if index % 5 == 0 {
                    print!("  ");
                };
                let c = if p { '#' } else { ' ' };
                print!("{c}");
            });
            println!();
        })
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_08.txt").expect("Cannot open input file");

    let commands: Vec<Command> = lines
        .filter_map(|l| {
            let s = l.unwrap();
            let words: Vec<&str> = s.split_whitespace().collect();
            match (words[0], words[1]) {
                ("rect", r) => {
                    let coords: Vec<&str> = r.split('x').collect();
                    Some(Command::Rect(
                        coords[0].parse().unwrap(),
                        coords[1].parse().unwrap(),
                    ))
                }
                ("rotate", "row") => {
                    let row_words: Vec<&str> = words[2].split('=').collect();
                    Some(Command::RotateRow(
                        row_words[1].parse().unwrap(),
                        words[4].parse().unwrap(),
                    ))
                }
                ("rotate", "column") => {
                    let col_words: Vec<&str> = words[2].split('=').collect();
                    Some(Command::RotateCol(
                        col_words[1].parse().unwrap(),
                        words[4].parse().unwrap(),
                    ))
                }
                _ => None,
            }
        })
        .collect();

    let mut lcd = TinyLCD {
        pixels: [[false; 50]; 6],
    };
    commands.iter().for_each(|c| lcd.process_command(c));

    println!("Part1: There are {} lit pixels", lcd.lit_pixels());
    println!("Part2:");
    lcd.display();
}
