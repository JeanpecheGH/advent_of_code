use itertools::Itertools;

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_09.txt").expect("Cannot open input file");

    let numbers: Vec<usize> = s.lines().map(|s| s.parse().unwrap()).collect();

    //Part1:
    let (idx, not_sum): (usize, usize) = numbers
        .windows(26)
        .find_position(|slice| {
            let target = slice.last().cloned().unwrap();
            !has_sum(slice, target)
        })
        .map(|(idx, slice)| (idx, slice.last().cloned().unwrap()))
        .unwrap();
    println!(
        "Part1: The first number that is not a sum of two of the 25 numbers before it is {not_sum}, found at index {idx}"
    );

    for i in 0..idx {
        for j in i + 1..idx {
            let sum: usize = numbers[i..=j].iter().sum();
            if sum == not_sum {
                let min: &usize = numbers[i..=j].iter().min().unwrap();
                let max: &usize = numbers[i..=j].iter().max().unwrap();
                println!(
                    "Part2: The range of numbers summing to {not_sum} starts at {i} and ends at {j}. The smallest and largest numbers in this range are {min} and {max}, summing to {}",
                     min+max
                );
            }
        }
    }
}

fn has_sum(slice: &[usize], sum: usize) -> bool {
    slice
        .iter()
        .enumerate()
        .any(|(i, &e_1)| slice[i + 1..].iter().any(|&e_2| e_1 + e_2 == sum))
}
