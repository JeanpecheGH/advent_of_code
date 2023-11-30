use util::intcode::IntCode;

struct Droid {
    intcode: IntCode,
}

impl Droid {
    fn write(&mut self) {
        if let Some(p) = self.intcode.read_prompt() {
            println!("{}", p);
        }
    }

    fn take(&mut self, item: &str) {
        self.intcode.write_cmd(&format!("take {item}"));
        self.write();
    }

    #[allow(dead_code)]
    fn drop(&mut self, item: &str) {
        self.intcode.write_cmd(&format!("drop {item}"));
        self.write();
    }

    #[allow(dead_code)]
    fn inv(&mut self) {
        self.intcode.write_cmd("inv");
        self.write();
    }
    fn north(&mut self) {
        self.intcode.write_cmd("north");
        self.write();
    }
    fn south(&mut self) {
        self.intcode.write_cmd("south");
        self.write();
    }
    fn west(&mut self) {
        self.intcode.write_cmd("west");
        self.write();
    }
    fn east(&mut self) {
        self.intcode.write_cmd("east");
        self.write();
    }
}
fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_25.txt").expect("Cannot open input file");
    let intcode: IntCode = s.lines().next().unwrap().parse().unwrap();
    let mut droid: Droid = Droid { intcode };
    droid.write();

    //There are 13 total items
    // 5 of them must not be picked up for they kill/block you: infinite loop, giant electromagnet, escape pod, molten lava, photons
    // 2 are too heavy on their own: spool of cat 6, dark matter
    // Without 2 of them you are too light: space heater, semiconductor.
    // Among the 4 remaining, we test every possibility
    //We need to get 4 items: space heater, hypercube, festive hat, semiconductor
    droid.south();
    droid.east();
    droid.take("space heater");
    droid.west();
    droid.north();
    droid.east();
    droid.north();
    droid.north();
    droid.take("hypercube");
    droid.south();
    droid.south();
    droid.west();
    droid.north();
    droid.take("festive hat");
    droid.west();
    droid.north();
    droid.east();
    droid.take("semiconductor");
    droid.east();
    droid.north();
    droid.west();
    println!("Computing time: {:?}", now.elapsed());
}
