use std::collections::HashSet;

#[derive(Copy, Clone)]
enum Op {
    Acc(isize),
    Jmp(isize),
    Nop(isize),
}

struct Computer {
    acc: isize,
    ops: Vec<Op>,
    idx: isize,
    op_log: HashSet<isize>,
    stuck: bool,
    finished: bool,
}

impl Computer {
    fn compute(&mut self) {
        self.stuck = !self.op_log.insert(self.idx);
        if self.stuck {
            return;
        }
        if self.idx as usize >= self.ops.len() {
            self.finished = true;
            return;
        }
        let op = &self.ops[self.idx as usize];
        match op {
            Op::Acc(n) => {
                self.acc += n;
                self.idx += 1;
            }
            Op::Jmp(off) => self.idx += off,
            Op::Nop(_) => self.idx += 1,
        }
    }

    fn get_acc(&self) -> isize {
        self.acc
    }
}

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_08.txt").expect("Cannot open input file");

    let ops: Vec<Op> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            match words[0] {
                "acc" => Op::Acc(words[1].parse().unwrap()),
                "jmp" => Op::Jmp(words[1].parse().unwrap()),
                "nop" => Op::Nop(words[1].parse().unwrap()),
                _ => Op::Nop(0),
            }
        })
        .collect();

    let mut comp = Computer {
        acc: 0,
        ops: ops.clone(),
        idx: 0,
        op_log: HashSet::default(),
        stuck: false,
        finished: false,
    };

    while !comp.stuck {
        comp.compute();
    }

    println!(
        "Part1: When we start looping, the accumulator has value {}",
        comp.get_acc()
    );

    for i in 0..ops.len() {
        //Change 1 jmp to nop or nop to jmp
        let mut mod_ops = ops.clone();
        match mod_ops[i] {
            Op::Acc(_) => (),
            Op::Jmp(n) => mod_ops[i] = Op::Nop(n),
            Op::Nop(n) => mod_ops[i] = Op::Jmp(n),
        }
        let mut mod_comp = Computer {
            acc: 0,
            ops: mod_ops,
            idx: 0,
            op_log: HashSet::default(),
            stuck: false,
            finished: false,
        };
        while !mod_comp.stuck && !mod_comp.finished {
            mod_comp.compute();
        }
        if mod_comp.finished {
            println!("Part2: When modifying the program so it does not loop, it ends with value {} in the accumulator", mod_comp.get_acc());
        }
    }
}
