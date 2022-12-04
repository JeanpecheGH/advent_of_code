use std::cell::Cell;

#[derive(Debug, Copy, Clone)]
enum Instruction<'a> {
    Half(&'a Registry),
    Triple(&'a Registry),
    Increment(&'a Registry),
    Jump(isize),
    JumpIfEven(&'a Registry, isize),
    JumpIfOne(&'a Registry, isize),
}

impl Instruction<'_> {
    fn compute(&self, i: isize) -> isize {
        match self {
            Instruction::Half(reg) => {
                reg.half();
                i + 1
            }
            Instruction::Triple(reg) => {
                reg.triple();
                i + 1
            }
            Instruction::Increment(reg) => {
                reg.incr();
                i + 1
            }
            Instruction::Jump(offset) => i + offset,
            Instruction::JumpIfEven(reg, offset) => {
                if reg.is_even() {
                    i + offset
                } else {
                    i + 1
                }
            }
            Instruction::JumpIfOne(reg, offset) => {
                if reg.is_one() {
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
    value: Cell<u64>,
}

impl Registry {
    fn new(n: u64) -> Self {
        Registry {
            value: Cell::new(n),
        }
    }

    fn is_one(&self) -> bool {
        self.value.get() == 1
    }

    fn is_even(&self) -> bool {
        self.value.get() % 2 == 0
    }

    fn incr(&self) {
        self.value.set(self.value.get() + 1);
    }

    fn half(&self) {
        self.value.set(self.value.get() >> 1);
    }

    fn triple(&self) {
        self.value.set(self.value.get() * 3);
    }

    fn set(&self, n: u64) {
        self.value.set(n)
    }
}

fn main() {
    let a = Registry::new(0);
    let b = Registry::new(0);
    let mut instructions: [Instruction; 48] = [Instruction::Jump(0); 48];
    let s = util::file_as_string("aoc_2015/input/day_23.txt").expect("Cannot open input file");
    s.lines().enumerate().for_each(|(i, s)| {
        let words: Vec<&str> = s.split(' ').collect();
        match (words[0], words[1]) {
            ("hlf", _) => instructions[i] = Instruction::Half(&a),
            ("tpl", _) => instructions[i] = Instruction::Triple(&a),
            ("inc", "a") => instructions[i] = Instruction::Increment(&a),
            ("inc", "b") => instructions[i] = Instruction::Increment(&b),
            ("jmp", offset) => {
                let of = offset.parse::<isize>().unwrap();
                instructions[i] = Instruction::Jump(of);
            }
            ("jie", _) => {
                let of = words[2].parse::<isize>().unwrap();
                instructions[i] = Instruction::JumpIfEven(&a, of);
            }
            ("jio", _) => {
                let of = words[2].parse::<isize>().unwrap();
                instructions[i] = Instruction::JumpIfOne(&a, of);
            }
            _ => (),
        }
    });
    let mut i: isize = 0;
    let instr_size: isize = instructions.len() as isize;
    while i >= 0 && i < instr_size {
        let instr = instructions[i as usize];
        i = instr.compute(i);
    }

    println!(
        "Part1: Registry B contains {} when starting with 0 in registry A",
        b.value.get()
    );

    i = 0;
    a.set(1);
    b.set(0);
    while i >= 0 && i < instr_size {
        let instr = instructions[i as usize];
        i = instr.compute(i);
    }

    println!(
        "Part2: Registry B contains {} when starting with 1 in registry A",
        b.value.get()
    );
}
