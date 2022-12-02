use std::cell::Cell;
use std::collections::HashMap;

#[derive(Debug, Copy, Clone)]
enum Instruction<'a> {
    CopyReg(&'a Registry, &'a Registry),
    CopyVal(isize, &'a Registry),
    Increment(&'a Registry),
    Decrement(&'a Registry),
    JumpIfNotZeroReg(&'a Registry, isize),
    JumpIfNotZeroVal(isize, isize),
    Out(&'a Registry),
    Mul(&'a Registry, &'a Registry, &'a Registry),
}

#[derive(Debug, Clone)]
struct Computer<'a> {
    ops: [Instruction<'a>; 30],
    idx: isize,
    out: Vec<isize>,
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
                reg.inc();
                self.idx += 1;
            }
            Instruction::Decrement(reg) => {
                reg.dec();
                self.idx += 1;
            }
            Instruction::JumpIfNotZeroVal(n, offset) => {
                if n != 0 {
                    self.idx += offset
                } else {
                    self.idx += 1;
                }
            }
            Instruction::JumpIfNotZeroReg(reg, offset) => {
                if !reg.is_zero() {
                    self.idx += offset
                } else {
                    self.idx += 1;
                }
            }
            Instruction::Out(reg) => {
                self.out.push(reg.get());
                self.idx += 1;
            }
            Instruction::Mul(src_1, src_2, tgt) => {
                let prod = src_1.get() * src_2.get();
                tgt.set(tgt.get() + prod);
                self.idx += 1
            }
        }
    }

    fn reset(&mut self) {
        self.idx = 0;
        self.out.clear();
    }
}

#[derive(Debug)]
struct Registry {
    value: Cell<isize>,
}

impl Registry {
    fn new(n: isize) -> Self {
        Registry {
            value: Cell::new(n),
        }
    }

    fn is_zero(&self) -> bool {
        self.value.get() == 0
    }

    fn inc(&self) {
        self.value.set(self.value.get() + 1);
    }

    fn dec(&self) {
        self.value.set(self.value.get() - 1);
    }

    fn set(&self, n: isize) {
        self.value.set(n)
    }

    fn get(&self) -> isize {
        self.value.get()
    }
}

fn main() {
    let lines =
        util::file_as_lines("aoc_2016/input/day_25_shortcut.txt").expect("Cannot open input file");
    let a = Registry::new(0);
    let b = Registry::new(0);
    let c = Registry::new(0);
    let d = Registry::new(0);
    let reg_map: HashMap<&str, &Registry> =
        HashMap::from([("a", &a), ("b", &b), ("c", &c), ("d", &d)]);

    let mut instructions: [Instruction; 30] = [Instruction::JumpIfNotZeroVal(0, 0); 30];
    lines.enumerate().for_each(|(i, l)| {
        let s = l.unwrap();
        let words: Vec<&str> = s.split(' ').collect();
        match (words[0], words[1].parse::<isize>().ok()) {
            ("inc", _) => instructions[i] = Instruction::Increment(reg_map.get(words[1]).unwrap()),
            ("dec", _) => instructions[i] = Instruction::Decrement(reg_map.get(words[1]).unwrap()),
            ("cpy", Some(v)) => {
                instructions[i] = Instruction::CopyVal(v, reg_map.get(words[2]).unwrap())
            }
            ("cpy", None) => {
                instructions[i] = Instruction::CopyReg(
                    reg_map.get(words[1]).unwrap(),
                    reg_map.get(words[2]).unwrap(),
                )
            }
            ("jnz", Some(v)) => {
                instructions[i] =
                    Instruction::JumpIfNotZeroVal(v, words[2].parse::<isize>().unwrap())
            }
            ("jnz", None) => {
                instructions[i] = Instruction::JumpIfNotZeroReg(
                    reg_map.get(words[1]).unwrap(),
                    words[2].parse::<isize>().unwrap(),
                )
            }
            ("out", _) => {
                instructions[i] = Instruction::Out(reg_map.get(words[1]).unwrap());
            }
            ("mul", None) => {
                instructions[i] = Instruction::Mul(
                    reg_map.get(words[1]).unwrap(),
                    reg_map.get(words[2]).unwrap(),
                    reg_map.get(words[3]).unwrap(),
                )
            }
            _ => (),
        }
    });

    let mut computer: Computer = Computer {
        ops: instructions,
        idx: 0,
        out: Vec::new(),
    };

    //What the program is doing :
    // let mut a_reg = 158 + 2572;
    // let b_reg = a_reg;
    // loop {
    //     while a_reg != 0 {
    //         print!("{} ", a_reg % 2);
    //         a_reg /= 2;
    //     }
    //     println!();
    //     a_reg = b_reg;
    // }

    let mut start_a: isize = 0;
    loop {
        a.set(start_a);
        b.set(0);
        c.set(0);
        d.set(0);
        computer.reset();
        loop {
            computer.compute();
            if computer.out.len() >= 20 {
                break;
            }
        }
        if computer.out.eq(&vec![
            0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1, 0, 1,
        ]) {
            println!(
                "We set register A to {} to produce an infinite signal {:?}",
                start_a, computer.out,
            );
            break;
        }
        start_a += 1;
    }
}
