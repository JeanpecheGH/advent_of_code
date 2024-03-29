use std::collections::HashSet;
use std::str::FromStr;
use util::coord::PosI;
use util::orientation::Dir;

struct Move {
    dir: Dir,
    dist: usize,
}

impl FromStr for Move {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let dir: Dir = match words[0] {
            "L" => Dir::West,
            "R" => Dir::East,
            "U" => Dir::North,
            "D" => Dir::South,
            _ => return Err(()), //Should not happen
        };
        let dist: usize = words[1].parse().unwrap();
        Ok(Move { dir, dist })
    }
}

struct Rope {
    knots: Vec<PosI>,
    tail_positions: HashSet<PosI>,
}

impl Rope {
    fn move_rope(&mut self, m: &Move) {
        (0..m.dist).for_each(|_| self.move_one(&m.dir))
    }

    fn move_one(&mut self, dir: &Dir) {
        let PosI(x, y): PosI = self.knots[0];
        match dir {
            Dir::West => self.knots[0] = PosI(x - 1, y),
            Dir::East => self.knots[0] = PosI(x + 1, y),
            Dir::North => self.knots[0] = PosI(x, y + 1),
            Dir::South => self.knots[0] = PosI(x, y - 1),
        }
        for i in 1..self.knots.len() {
            self.follow_tail(i);
        }
    }

    fn follow_tail(&mut self, idx: usize) {
        let PosI(x, y): PosI = self.knots[idx - 1];
        let PosI(i, j): PosI = self.knots[idx];
        match (x.abs_diff(i), y.abs_diff(j)) {
            (2, 2) => self.knots[idx] = PosI((x + i) / 2, (y + j) / 2),
            (2, _) => self.knots[idx] = PosI((x + i) / 2, y),
            (_, 2) => self.knots[idx] = PosI(x, (y + j) / 2),
            _ => (),
        }
        if (idx + 1) == self.knots.len() {
            self.tail_positions.insert(self.knots[idx]);
        }
    }

    fn nb_tail_pos(&self) -> usize {
        self.tail_positions.len()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_09.txt").expect("Cannot open input file");

    let moves: Vec<Move> = s.lines().map(|s| s.parse().unwrap()).collect();

    let mut rope_2: Rope = Rope {
        knots: vec![PosI(0, 0); 2],
        tail_positions: HashSet::new(),
    };

    let mut rope_10: Rope = Rope {
        knots: vec![PosI(0, 0); 10],
        tail_positions: HashSet::new(),
    };

    moves.iter().for_each(|m| {
        rope_2.move_rope(m);
        rope_10.move_rope(m);
    });

    println!(
        "Part1: The tail of the 2 knot rope visited {} nodes",
        rope_2.nb_tail_pos()
    );
    println!(
        "Part2: The tail of the 10 knot rope visited {} nodes",
        rope_10.nb_tail_pos()
    );
}
