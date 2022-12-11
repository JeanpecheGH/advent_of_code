use itertools::Itertools;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
enum OpType {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
enum Operand {
    Val(usize),
    SelfRef,
}

#[derive(Debug, Clone)]
struct Monkey {
    items: VecDeque<usize>,
    op_type: OpType,
    operand: Operand,
    divisor: usize,
    true_target: usize,
    false_target: usize,
    nb_inspected: usize,
}

impl Monkey {
    fn throw_all(&mut self, modulo: Option<usize>) -> Vec<(usize, usize)> {
        let nb_items: usize = self.items.len();
        self.nb_inspected += nb_items;
        (0..nb_items).map(|_| self.throw(modulo)).collect()
    }

    fn throw(&mut self, modulo: Option<usize>) -> (usize, usize) {
        let it: usize = self.items.pop_front().unwrap();
        let v: usize = match (&self.op_type, &self.operand) {
            (OpType::Add, Operand::Val(n)) => it + n,
            (OpType::Add, Operand::SelfRef) => it + it,
            (OpType::Mul, Operand::Val(n)) => it * n,
            (OpType::Mul, Operand::SelfRef) => it * it,
        };
        let v = match modulo {
            Some(n) => v % n,
            None => v / 3,
        };
        if (v) % self.divisor == 0 {
            (v, self.true_target)
        } else {
            (v, self.false_target)
        }
    }

    fn receive(&mut self, it: usize) {
        self.items.push_back(it);
    }
}

#[derive(Debug)]
struct MonkeyTroop {
    monkeys: Vec<Monkey>,
}

impl MonkeyTroop {
    fn round(&mut self, div: bool) {
        (0..self.monkeys.len()).for_each(|i| {
            let modulo = if div { None } else { Some(self.div_prod()) };
            let thrown: Vec<(usize, usize)> = self.monkeys[i].throw_all(modulo);
            thrown
                .iter()
                .for_each(|&(it, target)| self.monkeys[target].receive(it))
        });
    }

    fn div_prod(&self) -> usize {
        self.monkeys.iter().map(|m| m.divisor).product()
    }

    fn business(&self) -> usize {
        self.monkeys
            .iter()
            .map(|m| m.nb_inspected)
            .sorted()
            .rev()
            .take(2)
            .product()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_11.txt").expect("Cannot open input file");

    let lines: Vec<&str> = s.lines().collect();
    let monkeys: Vec<Monkey> = lines.split(|l| l.is_empty()).map(parse_monkey).collect();

    let now = std::time::Instant::now();
    let mut troop: MonkeyTroop = MonkeyTroop {
        monkeys: monkeys.clone(),
    };
    for _ in 0..20 {
        troop.round(true);
    }
    println!(
        "Part1: The business level after 20 round is {} (found in {:?})",
        troop.business(),
        now.elapsed()
    );

    let now = std::time::Instant::now();
    let mut troop_2: MonkeyTroop = MonkeyTroop { monkeys };
    for _ in 0..10000 {
        troop_2.round(false);
    }
    println!("Part2: The business level after 10000 round and without dividing the worry level is {} (found in {:?})", troop_2.business(), now.elapsed());
}

fn parse_monkey(lines: &[&str]) -> Monkey {
    //Line 1 for items
    let w1: Vec<&str> = lines[1].split_whitespace().collect();
    let items: VecDeque<usize> = (2..w1.len())
        .map(|i| w1[i].trim_end_matches(',').parse().unwrap())
        .collect();

    //Line 2 for operation
    let w2: Vec<&str> = lines[2].split_whitespace().collect();
    let op_type = match w2[4].parse().unwrap() {
        '+' => OpType::Add,
        '*' => OpType::Mul,
        _ => OpType::Add,
    };
    let op = w2[5].parse();
    let operand = if let Ok(n) = op {
        Operand::Val(n)
    } else {
        Operand::SelfRef
    };

    //Line 3 for test
    let w3: Vec<&str> = lines[3].split_whitespace().collect();
    let divisor = w3[3].parse().unwrap();

    //Line 4 for true_target
    let w4: Vec<&str> = lines[4].split_whitespace().collect();
    let true_target = w4[5].parse().unwrap();

    //Line 5 for false_target
    let w5: Vec<&str> = lines[5].split_whitespace().collect();
    let false_target = w5[5].parse().unwrap();

    Monkey {
        items,
        op_type,
        operand,
        divisor,
        true_target,
        false_target,
        nb_inspected: 0,
    }
}
