use std::cmp::Ordering;
use util::intcode::IntCode;

const HEIGHT: usize = 22;
const WIDTH: usize = 37;

struct Arkanoid {
    code: IntCode,
    grid: [[u8; WIDTH]; HEIGHT],
    score: isize,
}

impl Arkanoid {
    fn from_code(code: IntCode) -> Self {
        let grid: [[u8; WIDTH]; HEIGHT] = [[0; WIDTH]; HEIGHT];
        Self {
            code,
            grid,
            score: 0,
        }
    }

    fn play(&mut self) -> isize {
        self.insert_coins();
        self.frame(0);
        while self.nb_blocks() > 0 {
            let js: isize = self.joystick();
            self.frame(js);
            //Print game !
            // let ten_millis = Duration::from_millis(30);
            //
            // thread::sleep(ten_millis);
            // print!("\x1B[2J\x1B[1;1H");
            // self.print();
        }
        self.score
    }

    fn frame(&mut self, input: isize) {
        self.code.compute(vec![input]);
        self.code.output.chunks(3).for_each(|chunk| {
            let [x, y, id]: [_; 3] = chunk.try_into().unwrap();
            if x == -1 {
                self.score = id;
            } else {
                self.grid[y as usize][x as usize] = id as u8;
            }
        });
        self.code.output.clear();
    }

    fn nb_blocks(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|id| **id == 2).count())
            .sum()
    }

    fn joystick(&self) -> isize {
        let ball_x: usize = self
            .grid
            .iter()
            .find_map(|row| {
                row.iter()
                    .enumerate()
                    .find_map(|(x, pixel)| if *pixel == 4 { Some(x) } else { None })
            })
            .unwrap();

        let paddle_x = self.grid[20]
            .iter()
            .enumerate()
            .find_map(|(x, pixel)| if *pixel == 3 { Some(x) } else { None })
            .unwrap();

        match paddle_x.cmp(&ball_x) {
            Ordering::Less => 1,
            Ordering::Equal => 0,
            Ordering::Greater => -1,
        }
    }

    fn insert_coins(&mut self) {
        self.code.ops[0] = 2;
    }

    fn print(&self) {
        for row in self.grid {
            for p in row {
                let c: char = match p {
                    1 => 'X',
                    2 => '█',
                    3 => '_',
                    4 => 'O',
                    _ => ' ',
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_13.txt").expect("Cannot open input file");
    let code: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut game: Arkanoid = Arkanoid::from_code(code.clone());
    game.frame(0);
    game.print();
    println!(
        "Part1: There are {} block tiles on the screen",
        game.nb_blocks()
    );
    let mut game_2: Arkanoid = Arkanoid::from_code(code);
    let score: isize = game_2.play();
    println!("Part2: The final score is {score}");
    println!("Computing time: {:?}", now.elapsed());
}
