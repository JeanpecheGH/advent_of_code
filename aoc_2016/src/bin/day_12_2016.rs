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
}

impl Instruction<'_> {
    fn compute(&self, i: isize) -> isize {
        match self {
            Instruction::CopyVal(n, reg) => {
                reg.set(*n);
                i + 1
            }
            Instruction::CopyReg(source, target) => {
                target.set(source.get());
                i + 1
            }
            Instruction::Increment(reg) => {
                reg.inc();
                i + 1
            }
            Instruction::Decrement(reg) => {
                reg.dec();
                i + 1
            }
            Instruction::JumpIfNotZeroVal(n, offset) => {
                if *n != 0 {
                    i + offset
                } else {
                    i + 1
                }
            }
            Instruction::JumpIfNotZeroReg(reg, offset) => {
                if !reg.is_zero() {
                    i + offset
                } else {
                    i + 1
                }
            }
        }
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
    let s = util::file_as_string("aoc_2016/input/day_12.txt").expect("Cannot open input file");
    let a = Registry::new(0);
    let b = Registry::new(0);
    let c = Registry::new(0);
    let d = Registry::new(0);
    let reg_map: HashMap<&str, &Registry> =
        HashMap::from([("a", &a), ("b", &b), ("c", &c), ("d", &d)]);

    let mut instructions: [Instruction; 23] = [Instruction::JumpIfNotZeroVal(0, 0); 23];
    s.lines().enumerate().for_each(|(i, s)| {
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
            _ => (),
        }
    });

    //Instruction loop
    let mut i: isize = 0;
    let instr_size: isize = instructions.len() as isize;
    while i >= 0 && i < instr_size {
        let instr = instructions[i as usize];
        i = instr.compute(i);
    }

    println!("Part1: Registry A contains {}", a.get());

    //Part 2 : init C to 1
    a.set(0);
    b.set(0);
    c.set(1);
    d.set(0);
    let mut i: isize = 0;
    let instr_size: isize = instructions.len() as isize;
    while i >= 0 && i < instr_size {
        let instr = instructions[i as usize];
        i = instr.compute(i);
    }

    println!("Part2: Registry A contains {}", a.get());
}
