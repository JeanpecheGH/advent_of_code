#[derive(Debug)]
enum Operation {
    SwapPositions(usize, usize),
    SwapLetters(char, char),
    RotateLeft(usize),
    RotateRight(usize),
    RotateBased(char),
    ReversePositions(usize, usize),
    MovePositions(usize, usize),
}

#[derive(Debug)]
struct Password {
    letters: Vec<char>,
}
impl Password {
    fn compute(&mut self, op: &Operation) {
        match *op {
            Operation::SwapPositions(i, j) => {
                let a: char = self.letters[i];
                let b: char = self.letters[j];
                self.letters[i] = b;
                self.letters[j] = a;
            }
            Operation::SwapLetters(a, b) => {
                let i: usize = self.letters.iter().position(|&c| c == a).unwrap();
                let j: usize = self.letters.iter().position(|&c| c == b).unwrap();
                self.letters[i] = b;
                self.letters[j] = a;
            }
            Operation::RotateLeft(n) => self.letters.rotate_left(n),
            Operation::RotateRight(n) => self.letters.rotate_right(n),
            Operation::RotateBased(a) => {
                let i: usize = self.letters.iter().position(|&c| c == a).unwrap();
                let nb_rot = if i >= 4 { i + 2 } else { i + 1 };
                self.letters.rotate_right(nb_rot % 8);
            }
            Operation::ReversePositions(i, j) => {
                let mut slice: Vec<char> = self.letters[i..=j].to_vec();
                slice.reverse();
                self.letters.splice(i..=j, slice);
            }
            Operation::MovePositions(i, j) => {
                let c: char = self.letters.remove(i);
                self.letters.insert(j, c);
            }
        }
    }

    fn reverse(&mut self, op: &Operation) {
        match *op {
            Operation::RotateLeft(n) => self.letters.rotate_right(n),
            Operation::RotateRight(n) => self.letters.rotate_left(n),
            Operation::RotateBased(a) => {
                let i: usize = self.letters.iter().position(|&c| c == a).unwrap();
                if i % 2 == 1 {
                    self.letters.rotate_left(i.div_ceil(2));
                } else {
                    let rot = (i / 2 + 3) % 4 + 6;
                    self.letters.rotate_left(rot % 8)
                }
            }
            Operation::MovePositions(i, j) => self.compute(&Operation::MovePositions(j, i)),
            _ => self.compute(op),
        }
    }

    fn get_password(&self) -> String {
        self.letters.iter().collect()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_21.txt").expect("Cannot open input file");
    let input: &str = "abcdefgh";
    let ops: Vec<Operation> = s
        .lines()
        .filter_map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            match (words[0], words[1]) {
                ("swap", "position") => Some(Operation::SwapPositions(
                    words[2].parse().unwrap(),
                    words[5].parse().unwrap(),
                )),
                ("swap", "letter") => Some(Operation::SwapLetters(
                    words[2].chars().next().unwrap(),
                    words[5].chars().next().unwrap(),
                )),
                ("rotate", "left") => Some(Operation::RotateLeft(words[2].parse().unwrap())),
                ("rotate", "right") => Some(Operation::RotateRight(words[2].parse().unwrap())),
                ("rotate", "based") => {
                    Some(Operation::RotateBased(words[6].chars().next().unwrap()))
                }
                ("reverse", "positions") => Some(Operation::ReversePositions(
                    words[2].parse().unwrap(),
                    words[4].parse().unwrap(),
                )),
                ("move", "position") => Some(Operation::MovePositions(
                    words[2].parse().unwrap(),
                    words[5].parse().unwrap(),
                )),
                _ => None,
            }
        })
        .collect();

    let mut pwd = Password {
        letters: input.chars().collect(),
    };
    println!("Part1:\nOriginal password: {}", pwd.get_password());
    ops.iter().for_each(|op| {
        pwd.compute(op);
    });
    println!("Scrambled password is {}", pwd.get_password());

    //Part 2
    let input = "fbgdceah";
    let mut pwd = Password {
        letters: input.chars().collect(),
    };
    println!("Part2:\nScrambled password: {}", pwd.get_password());
    ops.iter().rev().for_each(|op| {
        pwd.reverse(op);
    });
    println!("Unscrambled password is {}", pwd.get_password());
}
