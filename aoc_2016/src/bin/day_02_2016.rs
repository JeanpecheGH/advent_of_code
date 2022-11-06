struct Keypad {
    keys: [[char; 3]; 3],
    x: usize,
    y: usize,
}

impl Keypad {
    fn press(&self) -> char {
        self.keys[self.y][self.x]
    }

    fn move_finger(&mut self, c: char) {
        match (c, self.x, self.y) {
            ('U', _, 1..=2) => self.y -= 1,
            ('D', _, 0..=1) => self.y += 1,
            ('L', 1..=2, _) => self.x -= 1,
            ('R', 0..=1, _) => self.x += 1,
            _ => (),
        }
    }

    fn moves(&mut self, moves: &str) {
        moves.chars().for_each(|c| self.move_finger(c))
    }
}

struct DiamondKeypad {
    keys: [[char; 5]; 5],
    x: usize,
    y: usize,
}

impl DiamondKeypad {
    fn press(&self) -> char {
        self.keys[self.y][self.x]
    }

    fn move_finger(&mut self, c: char) {
        match (c, self.x, self.y) {
            ('U', 2, 1..=4) => self.y -= 1,
            ('U', 1..=3, 2..=4) => self.y -= 1,
            ('D', 2, 0..=3) => self.y += 1,
            ('D', 1..=3, 0..=2) => self.y += 1,
            ('L', 1..=4, 2) => self.x -= 1,
            ('L', 2..=4, 1..=3) => self.x -= 1,
            ('R', 0..=3, 2) => self.x += 1,
            ('R', 0..=2, 1..=3) => self.x += 1,
            _ => (),
        }
    }

    fn moves(&mut self, moves: &str) {
        moves.chars().for_each(|c| self.move_finger(c))
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_02.txt").expect("Cannot open input file");

    let moves: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let mut keypad = Keypad {
        keys: [['1', '2', '3'], ['4', '5', '6'], ['7', '8', '9']],
        x: 1,
        y: 1,
    };

    let first_code: String = moves
        .iter()
        .map(|s| {
            keypad.moves(s);
            keypad.press()
        })
        .collect();

    println!("Part1: The code to the bathroom should be {}", first_code);

    let mut diamond = DiamondKeypad {
        keys: [
            [' ', ' ', '1', ' ', ' '],
            [' ', '2', '3', '4', ' '],
            ['5', '6', '7', '8', '9'],
            [' ', 'A', 'B', 'C', ' '],
            [' ', ' ', 'D', ' ', ' '],
        ],
        x: 0,
        y: 2,
    };

    let second_code: String = moves
        .iter()
        .map(|s| {
            diamond.moves(s);
            diamond.press()
        })
        .collect();

    println!("Part2: The actual code to the bathroom is {}", second_code);
}
