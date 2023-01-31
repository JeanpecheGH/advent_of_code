use util::intcode::IntCode;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_05.txt").expect("Cannot open input file");
    let mut intcode: IntCode = s.lines().next().unwrap().parse().unwrap();
    intcode.compute(1);
    println!(
        "Part1: With input 1, the diagnostic code is {}",
        intcode.output.last().unwrap()
    );
    intcode.reset();
    intcode.compute(5);
    println!(
        "Part2: With input 5, the diagnostic code is now {}",
        intcode.output.last().unwrap()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_part_1() {
        let input: isize = 55;
        let mut code: IntCode = "3,0,4,0,99".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "55,0,4,0,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![input]);
    }

    #[test]
    fn test_2_part_1() {
        let mut code: IntCode = "1002,4,3,4,33".parse().unwrap();
        code.compute(0);
        let expect: IntCode = "1002,4,3,4,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
    }

    #[test]
    fn test_1_part_2() {
        let input: isize = 8;
        let mut code: IntCode = "3,9,8,9,10,9,4,9,99,-1,8".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,9,8,9,10,9,4,9,99,1,8".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![1]);
    }

    #[test]
    fn test_2_part_2() {
        let input: isize = 12;
        let mut code: IntCode = "3,9,7,9,10,9,4,9,99,-1,8".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,9,7,9,10,9,4,9,99,0,8".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![0]);
    }

    #[test]
    fn test_3_part_2() {
        let input: isize = 12;
        let mut code: IntCode = "3,3,1108,-1,8,3,4,3,99".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,3,1108,0,8,3,4,3,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![0]);
    }

    #[test]
    fn test_4_part_2() {
        let input: isize = 5;
        let mut code: IntCode = "3,3,1107,-1,8,3,4,3,99".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,3,1107,1,8,3,4,3,99".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![1]);
    }

    #[test]
    fn test_5_part_2() {
        let input: isize = 0;
        let mut code: IntCode = "3,12,6,12,15,1,13,14,13,4,13,99,-1,0,1,9".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,12,6,12,15,1,13,14,13,4,13,99,0,0,1,9".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![0]);
    }

    #[test]
    fn test_6_part_2() {
        let input: isize = 7;
        let mut code: IntCode = "3,3,1105,-1,9,1101,0,0,12,4,12,99,1".parse().unwrap();
        code.compute(input);
        let expect: IntCode = "3,3,1105,7,9,1101,0,0,12,4,12,99,1".parse().unwrap();
        assert_eq!(code.ops, expect.ops);
        assert_eq!(code.output, vec![1]);
    }

    const INPUT: &str = "3,21,1008,21,8,20,1005,20,22,107,8,21,20,1006,20,31,1106,0,36,98,0,0,1002,21,125,20,4,20,1105,1,46,104,999,1105,1,46,1101,1000,1,20,4,20,1105,1,46,98,99";

    #[test]
    fn test_7_part_2() {
        let mut code: IntCode = INPUT.parse().unwrap();
        code.compute(7);
        assert_eq!(code.output, vec![999]);
        code.reset();
        code.compute(8);
        assert_eq!(code.output, vec![1000]);
        code.reset();
        code.compute(9);
        assert_eq!(code.output, vec![1001]);
    }
}
