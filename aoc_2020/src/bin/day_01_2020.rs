fn main() {
    let s = util::file_as_string("aoc_2020/input/day_01.txt").expect("Cannot open input file");
    let target_sum = 2020;

    let expenses: Vec<usize> = s.lines().map(|s| s.parse().unwrap()).collect();

    for (idx, &ex_1) in expenses.iter().enumerate() {
        for &ex_2 in expenses[idx..].iter() {
            if ex_1 + ex_2 == target_sum {
                println!(
                    "Part1: The expenses with a sum of {} are {} and {}. Their product is {}",
                    target_sum,
                    ex_1,
                    ex_2,
                    ex_1 * ex_2
                );
                break;
            }
        }
    }

    for (idx_1, &ex_1) in expenses.iter().enumerate() {
        for (idx_2, &ex_2) in expenses[idx_1..].iter().enumerate() {
            for &ex_3 in expenses[idx_2..].iter() {
                if ex_1 + ex_2 + ex_3 == target_sum {
                    println!(
                    "Part1: The three expenses with a sum of {} are {}, {} and {}. Their product is {}",
                    target_sum,
                    ex_1,
                    ex_2,
                    ex_3,
                    ex_1 * ex_2 * ex_3
                );
                    break;
                }
            }
        }
    }
}
