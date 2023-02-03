use util::intcode::IntCode;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_09.txt").expect("Cannot open input file");
    let mut code: IntCode = s.lines().next().unwrap().parse().unwrap();
    code.compute(&mut vec![1]);
    println!("Part1: The BOOST keycode is {}", code.output[0]);
    code.reset();
    code.compute(&mut vec![2]);
    println!(
        "Part2: The coordinates of the distress signal are {}",
        code.output[0]
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_1_part_1() {
        let mut code: IntCode = "109,1,204,-1,1001,100,1,100,1008,100,16,101,1006,101,0,99"
            .parse()
            .unwrap();
        code.compute(&mut Vec::new());
        assert_eq!(
            code.output,
            vec![109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99]
        );
    }
    #[test]
    fn test_2_part_1() {
        let mut code: IntCode = "1102,34915192,34915192,7,4,7,99,0".parse().unwrap();
        code.compute(&mut Vec::new());
        assert_eq!(code.output[0], 1219070632396864);
    }
    #[test]
    fn test_3_part_1() {
        let mut code: IntCode = "104,1125899906842624,99".parse().unwrap();
        code.compute(&mut Vec::new());
        assert_eq!(code.output[0], 1125899906842624);
    }
}
