enum Op {
    Add(isize),
    Noop,
}

struct Computer {
    x: isize,
    cycle: isize,
    idx: usize,
    ops: Vec<Op>,
    signal_strength: Vec<isize>,
    pixels: Vec<bool>,
    finished: bool,
}

impl Computer {
    fn compute(&mut self) {
        self.draw_pixel();
        self.cycle += 1;
        self.add_signal_strength();
        match self.ops[self.idx] {
            Op::Noop => self.idx += 1,
            Op::Add(n) => {
                self.draw_pixel();
                self.cycle += 1;
                self.add_signal_strength();
                self.x += n;
                self.idx += 1;
            }
        }
        //Stop at 240 cycles
        if self.cycle == 240 {
            self.finished = true;
        }
    }

    fn add_signal_strength(&mut self) {
        if self.cycle % 40 == 20 {
            self.signal_strength.push(self.cycle * self.x);
        }
    }

    fn draw_pixel(&mut self) {
        let r = self.x - 1..=self.x + 1;
        let b: bool = r.contains(&(self.cycle % 40));
        self.pixels.push(b);
    }

    fn print_screen(&self) {
        for row in 0..6 {
            for p in 0..40 {
                let c = if self.pixels[row * 40 + p] {
                    "██"
                } else {
                    "  "
                };
                print!("{c}");
            }
            println!();
        }
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_10.txt").expect("Cannot open input file");

    let ops: Vec<Op> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            match words[0] {
                "addx" => Op::Add(words[1].parse().unwrap()),
                _ => Op::Noop,
            }
        })
        .collect();

    let mut comp = Computer {
        x: 1,
        cycle: 0,
        idx: 0,
        ops,
        signal_strength: Vec::new(),
        pixels: Vec::new(),
        finished: false,
    };

    while !comp.finished {
        comp.compute();
    }

    println!(
        "Part1: The signal strengths computed are {:?}, their sum is {}",
        comp.signal_strength,
        comp.signal_strength.iter().sum::<isize>()
    );

    println!("Part2: Look at the LCD!");
    comp.print_screen();
}
