use std::collections::HashMap;
use util::registry::Registry;

#[derive(Debug, Copy, Clone)]
enum Instruction<'a> {
    CopyReg(&'a Registry, &'a Registry),
    CopyVal(isize, &'a Registry),
    Increment(&'a Registry),
    Decrement(&'a Registry),
    JumpIfNotZeroRegVal(&'a Registry, isize),
    JumpIfNotZeroValReg(isize, &'a Registry),
    JumpIfNotZeroValVal(isize, isize),
    JumpIfNotZeroRegReg(&'a Registry, &'a Registry),
    ToggleReg(&'a Registry),
    DoNothingRegVal(&'a Registry, isize),
    DoNothingValVal(isize, isize),
    Mul(&'a Registry, &'a Registry, &'a Registry),
}

#[derive(Debug, Clone)]
struct Computer<'a> {
    ops: [Instruction<'a>; 26],
    idx: isize,
}

impl Computer<'_> {
    fn compute(&mut self) {
        match self.ops[self.idx as usize] {
            Instruction::CopyVal(n, reg) => {
                reg.set(n);
                self.idx += 1;
            }
            Instruction::CopyReg(source, target) => {
                target.set(source.get());
                self.idx += 1;
            }
            Instruction::Increment(reg) => {
                reg.incr();
                self.idx += 1;
            }
            Instruction::Decrement(reg) => {
                reg.decr();
                self.idx += 1;
            }
            Instruction::JumpIfNotZeroValVal(n, offset) => {
                if n != 0 {
                    self.idx += offset;
                } else {
                    self.idx += 1;
                }
            }
            Instruction::JumpIfNotZeroRegVal(reg, offset) => {
                if !reg.is_zero() {
                    self.idx += offset;
                } else {
                    self.idx += 1;
                }
            }
            Instruction::JumpIfNotZeroValReg(n, reg) => {
                if n != 0 {
                    self.idx += reg.get();
                } else {
                    self.idx += 1;
                }
            }
            Instruction::JumpIfNotZeroRegReg(reg_1, reg_2) => {
                if !reg_1.is_zero() {
                    self.idx += reg_2.get();
                } else {
                    self.idx += 1;
                }
            }
            Instruction::ToggleReg(reg) => {
                self.toggle(reg.get() + self.idx);
                self.idx += 1
            }
            Instruction::DoNothingRegVal(_, _) | Instruction::DoNothingValVal(_, _) => {
                self.idx += 1
            }
            Instruction::Mul(src_1, src_2, tgt) => {
                let prod = src_1.get() * src_2.get();
                tgt.set(tgt.get() + prod);
                self.idx += 1
            }
        }
    }

    fn toggle(&mut self, idx: isize) {
        if !(0..26).contains(&idx) {
            return;
        }
        self.ops[idx as usize] = match self.ops[idx as usize] {
            Instruction::Increment(reg) => Instruction::Decrement(reg),
            Instruction::Decrement(reg) => Instruction::Increment(reg),
            Instruction::ToggleReg(reg) => Instruction::Increment(reg),
            Instruction::JumpIfNotZeroRegVal(reg, n) => Instruction::DoNothingRegVal(reg, n),
            Instruction::JumpIfNotZeroValReg(n, reg) => Instruction::CopyVal(n, reg),
            Instruction::JumpIfNotZeroValVal(m, n) => Instruction::DoNothingValVal(m, n),
            Instruction::JumpIfNotZeroRegReg(src, tgt) => Instruction::CopyReg(src, tgt),
            Instruction::CopyReg(src, tgt) => Instruction::JumpIfNotZeroRegReg(src, tgt),
            Instruction::CopyVal(n, reg) => Instruction::JumpIfNotZeroValReg(n, reg),
            Instruction::DoNothingRegVal(reg, n) => Instruction::JumpIfNotZeroRegVal(reg, n),
            Instruction::DoNothingValVal(m, n) => Instruction::JumpIfNotZeroValVal(m, n),
            Instruction::Mul(a, b, c) => Instruction::Mul(a, b, c),
        };
    }
}

fn main() {
    // let s = util::file_as_string("aoc_2016/input/day_23.txt").expect("Cannot open input file");
    let s =
        util::file_as_string("aoc_2016/input/day_23_shortcut.txt").expect("Cannot open input file");
    let a = Registry::default();
    let b = Registry::default();
    let c = Registry::default();
    let d = Registry::default();
    let reg_map: HashMap<&str, &Registry> =
        HashMap::from([("a", &a), ("b", &b), ("c", &c), ("d", &d)]);

    let mut ops: [Instruction; 26] = [Instruction::JumpIfNotZeroValVal(0, 0); 26];
    s.lines().enumerate().for_each(|(i, s)| {
        let words: Vec<&str> = s.split(' ').collect();
        match (words[0], words[1].parse::<isize>().ok()) {
            ("inc", _) => ops[i] = Instruction::Increment(reg_map.get(words[1]).unwrap()),
            ("dec", _) => ops[i] = Instruction::Decrement(reg_map.get(words[1]).unwrap()),
            ("cpy", Some(v)) => ops[i] = Instruction::CopyVal(v, reg_map.get(words[2]).unwrap()),
            ("cpy", None) => {
                ops[i] = Instruction::CopyReg(
                    reg_map.get(words[1]).unwrap(),
                    reg_map.get(words[2]).unwrap(),
                )
            }
            ("jnz", Some(v)) => match words[2].parse::<isize>().ok() {
                Some(v_2) => ops[i] = Instruction::JumpIfNotZeroValVal(v, v_2),
                None => {
                    ops[i] = Instruction::JumpIfNotZeroValReg(v, reg_map.get(words[2]).unwrap())
                }
            },
            ("jnz", None) => {
                ops[i] = Instruction::JumpIfNotZeroRegVal(
                    reg_map.get(words[1]).unwrap(),
                    words[2].parse::<isize>().unwrap(),
                )
            }
            ("tgl", None) => ops[i] = Instruction::ToggleReg(reg_map.get(words[1]).unwrap()),
            ("mul", None) => {
                ops[i] = Instruction::Mul(
                    reg_map.get(words[1]).unwrap(),
                    reg_map.get(words[2]).unwrap(),
                    reg_map.get(words[3]).unwrap(),
                )
            }
            _ => (),
        }
    });

    //Part 1
    let now = std::time::Instant::now();
    a.set(7);
    let mut computer: Computer = Computer { ops, idx: 0 };
    while (0..26).contains(&computer.idx) {
        computer.compute();
    }

    println!(
        "Part1: When starting at 7, reg A contains {}, found in {:?}",
        a.get(),
        now.elapsed()
    );

    //Part 2
    let now = std::time::Instant::now();
    a.set(12);
    b.set(0);
    c.set(0);
    d.set(0);
    let mut computer: Computer = Computer { ops, idx: 0 };
    while (0..26).contains(&computer.idx) {
        computer.compute();
    }

    println!(
        "Part2: When starting at 12, reg A contains {}, found in {:?}",
        a.get(),
        now.elapsed()
    );
}
