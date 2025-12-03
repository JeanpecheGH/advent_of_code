use nom::bytes::complete::tag;
use nom::sequence::preceded;
use nom::Parser;
use util::basic_parser::parse_usize;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_23.txt").expect("Cannot open input file");
    let reg_b: usize = parse_reg_b(s.lines().next().unwrap());

    let (nb_mul, reg_h) = optimized_loop(reg_b);
    println!("Part1: The mul operation is invoked {} times", nb_mul);
    println!(
        "Part2: After optimizing the program, register h contains value {}",
        reg_h
    );
    println!("Computing time: {:?}", now.elapsed());
}

fn parse_reg_b(s: &str) -> usize {
    preceded(tag("set b "), parse_usize).parse(s).unwrap().1
}

//The program is just counting the number of non-prime numbers between two value
//All results are only depending on the first value assigned to register 'b'
fn optimized_loop(reg_b: usize) -> (usize, usize) {
    let start: usize = 100 * reg_b + 100_000;
    let end: usize = start + 17_000;

    //Both inside loops start at 2 and end at reg_b - 1
    let nb_mul: usize = (reg_b - 2) * (reg_b - 2);
    let reg_h: usize = (start..=end)
        .step_by(17)
        .filter(|&i| {
            let sqrt: usize = (i as f64).sqrt() as usize;
            (2..=sqrt).any(|div| i % div == 0)
        })
        .count();
    (nb_mul, reg_h)
}
