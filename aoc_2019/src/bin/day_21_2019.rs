use util::intcode::IntCode;

struct SpringDroid {
    intcode: IntCode,
}

impl SpringDroid {
    fn walk(&mut self) -> isize {
        self.intcode.compute(Vec::new());
        print!("{}", self.intcode.read_prompt().unwrap());

        // Given we see only 4 steps ahead, we have to jump every time there is a hole (in A, B or C) and D is present
        // Else, we could be in a case where E is a hole, and we could not jump once it's too late
        // !(A ^ B ^ C)^D
        let instructions: &str = "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nWALK";
        println!("{}", instructions);
        self.intcode.write_cmd(instructions);
        let damage: isize = self.intcode.output.pop().unwrap();
        print!("{}", self.intcode.read_prompt().unwrap());
        self.intcode.reset();
        damage
    }

    fn run(&mut self) -> isize {
        self.intcode.compute(Vec::new());
        print!("{}", self.intcode.read_prompt().unwrap());

        // We keep the instructions from the first part
        // but we add the condition that either we can walk a step after the first jump (E is present)
        // or we can jump again (H is present)
        // !(A ^ B ^ C) ^ D ^ (E v H)
        let instructions: &str =
            "OR A T\nAND B T\nAND C T\nNOT T J\nAND D J\nOR E T\nOR H T\nAND T J\nRUN";
        println!("{}", instructions);
        self.intcode.write_cmd(instructions);
        let damage: isize = self.intcode.output.pop().unwrap();
        print!("{}", self.intcode.read_prompt().unwrap());
        damage
    }
}
fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_21.txt").expect("Cannot open input file");
    let intcode: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut droid: SpringDroid = SpringDroid { intcode };

    println!("Part1:");
    let damage: isize = droid.walk();
    println!(
        "The droid is reporting {} hull damage\n----------------------------------",
        damage
    );

    println!("Part2:");
    let damage: isize = droid.run();
    println!("The droid is now reporting {} hull damage", damage);

    println!("Computing time: {:?}", now.elapsed());
}
