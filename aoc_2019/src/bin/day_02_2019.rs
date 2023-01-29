use util::intcode::IntCode;

const TARGET: isize = 19690720;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_02.txt").expect("Cannot open input file");
    let mut code: IntCode = s.parse().unwrap();
    code.set(1, 12);
    code.set(2, 2);
    code.compute();
    println!(
        "Part1: After running the program, we have {} at position 0",
        code.pos(0)
    );
    code.reset();
    let mut res: isize = 0;
    for noun in 0..100 {
        for verb in 0..100 {
            code.set(1, noun);
            code.set(2, verb);
            code.compute();
            if code.pos(0) == TARGET {
                res = 100 * noun + verb;
                break;
            }
            code.reset();
        }
        if res > 0 {
            break;
        }
    }
    println!("Part2: In order to get {TARGET} in the output, the vern/noun code should be {res}");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "1,9,10,3,2,3,11,0,99,30,40,50";
    const INPUT_2: &str = "1,0,0,0,99";
    const INPUT_3: &str = "2,3,0,3,99";
    const INPUT_4: &str = "2,4,4,5,99,0";
    const INPUT_5: &str = "1,1,1,4,99,5,6,0,99";

    #[test]
    fn test_1_part_1() {
        let mut code: IntCode = INPUT_1.parse().unwrap();
        code.compute();
        let expect: IntCode = "3500,9,10,70,2,3,11,0,99,30,40,50".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }
    #[test]
    fn test_2_part_1() {
        let mut code: IntCode = INPUT_2.parse().unwrap();
        code.compute();
        let expect: IntCode = "2,0,0,0,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }
    #[test]
    fn test_3_part_1() {
        let mut code: IntCode = INPUT_3.parse().unwrap();
        code.compute();
        let expect: IntCode = "2,3,0,6,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }
    #[test]
    fn test_4_part_1() {
        let mut code: IntCode = INPUT_4.parse().unwrap();
        code.compute();
        let expect: IntCode = "2,4,4,5,99,9801".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }
    #[test]
    fn test_5_part_1() {
        let mut code: IntCode = INPUT_5.parse().unwrap();
        code.compute();
        let expect: IntCode = "30,1,1,4,2,5,6,0,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }
}
