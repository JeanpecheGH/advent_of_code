enum Op {
    Add(isize),
    Noop,
}

struct Computer {
    x: isize,
    cycle: isize,
    signals: Vec<isize>,
    pixels: Vec<bool>,
}

impl Computer {
    fn compute(&mut self, op: &Op) {
        self.signal_and_pixel();
        match op {
            Op::Noop => (),
            Op::Add(n) => {
                self.signal_and_pixel();
                self.x += n;
            }
        }
    }

    fn signal_and_pixel(&mut self) {
        //Store pixel
        let r = self.x - 1..=self.x + 1;
        let b: bool = r.contains(&(self.cycle % 40));
        self.pixels.push(b);

        self.cycle += 1;
        //Store signal
        self.signals.push(self.cycle * self.x);
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
        signals: Vec::new(),
        pixels: Vec::new(),
    };

    ops.iter().for_each(|op| {
        comp.compute(op);
    });

    let signal_sum: isize = vec![20, 60, 100, 140, 180, 220]
        .iter()
        .map(|n| comp.signals[n - 1])
        .sum();
    println!("Part1: The sum of the computed signal strengths is {signal_sum}");

    println!("Part2: Look at the LCD!");
    comp.print_screen();
}
