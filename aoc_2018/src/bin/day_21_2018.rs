use fxhash::FxHashSet;

// #ip 2
//00 - seti 123 0 5
//01 - bani 5 456 5
//02 - eqri 5 72 5
//03 - addr 5 2 2
//04 - seti 0 0 2               Looping to 00 until 123 & 456 = 72
//05 - seti 0 4 5               0 -> R5
//06 - bori 5 65536 4           R5 | 65536 -> R4
//07 - seti 15466939 9 5        15466939 -> R5
//08 - bani 4 255 3             R4 & 255 -> R3
//09 - addr 5 3 5               R3 + R5 -> R5
//10 - bani 5 16777215 5        R5 & 16777215 -> R5
//11 - muli 5 65899 5           R5 * 65899 -> R5
//12 - bani 5 16777215 5        R5 & 16777215 -> R5
//13 - gtir 256 4 3             if 256 > R4
//14 - addr 3 2 2               then Jump to 16 (then 28)
//15 - addi 2 1 2               else Jump to 17
//16 - seti 27 8 2
//17 - seti 0 7 3               0 -> R3
//18 - addi 3 1 1               R3 + 1 -> R1
//19 - muli 1 256 1             R1 * 256 -> R1
//20 - gtrr 1 4 1               if R1 > R4
//21 - addr 1 2 2               Jump to 23 (then 26)
//22 - addi 2 1 2               else Jump to 24
//23 - seti 25 2 2              Jump to 26
//24 - addi 3 1 3               R3 + 1 -> R3
//25 - seti 17 7 2              Jump to 18
//26 - setr 3 7 4               R3 -> R4
//27 - seti 7 3 2               Jump to 8
//28 - eqrr 5 0 3               If R5 == R0
//29 - addr 3 2 2               END
//30 - seti 5 9 2               else Jump to 6
#[warn(unused_assignments)]
fn last_int() -> (usize, usize) {
    let mut _three: usize = 0;
    let mut _four: usize = 0;
    let mut five: usize = 0;
    let mut pairs: FxHashSet<(usize, usize)> = FxHashSet::default();
    let mut five_set: FxHashSet<usize> = FxHashSet::default();
    let mut first_end: usize = 0;
    let mut ret: usize = 0;

    loop {
        //Line 6
        _four = five | 65536;
        five = 15466939;
        loop {
            //Line 8
            _three = _four & 255;
            five = (((five + _three) & 16777215) * 65899) & 16777215;

            if 256 > _four {
                break;
            }

            _three = 0;
            while _four >= (_three + 1) * 256 {
                _three += 1;
            }
            _four = _three;
        }

        if first_end == 0 {
            first_end = five
        }

        if !pairs.insert((_four, five)) {
            return (first_end, ret);
        }
        if five_set.insert(five) {
            ret = five;
        }
    }
}

fn main() {
    let now = std::time::Instant::now();
    //Reproducing the loop without emulation

    //let s = util::file_as_string("aoc_2018/input/day_21.txt").expect("Cannot open input file");
    //let mut device: WristDevice = s.parse().unwrap();
    // device.set_reg_value(0, 15615244);
    // device.apply_all_with_pointer(false);

    let (shortest, longest): (usize, usize) = last_int();

    println!("Part1: If we set {shortest} to the register 0, the program ends after the fewest instructions");
    println!("Part2: If we set {longest} to the register 0, the program ends after the most instructions");
    println!("Computing time: {:?}", now.elapsed());
}
