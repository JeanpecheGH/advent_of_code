fn main() {
    let input: &str = "11100010111110100";
    let target_size_part1: usize = 272;

    println!("Input: {}", input);
    let now = std::time::Instant::now();
    let dr: String = dragon_string(input, target_size_part1);
    let cs: String = checksum(&dr);
    let elapsed = now.elapsed();
    println!(
        "Part1: The checksum for target size {} is {}, found in {:?}",
        cs, target_size_part1, elapsed
    );

    let target_size_part2: usize = 35651584;

    let now = std::time::Instant::now();
    let dr: String = dragon_string(input, target_size_part2);
    let cs: String = checksum(&dr);
    let elapsed = now.elapsed();
    println!(
        "Part2: The checksum for target size {} is {}, found in {:?}",
        cs, target_size_part2, elapsed
    );
}

fn checksum(input: &str) -> String {
    let chars: Vec<char> = input.chars().collect();
    let s: String = chars
        .chunks(2)
        .map(|pair| if pair[0] == pair[1] { '1' } else { '0' })
        .collect();
    if s.len() % 2 == 0 {
        checksum(&s)
    } else {
        s
    }
}

fn dragon_string(input: &str, target_size: usize) -> String {
    let s: String = format!("{}0{}", input, reverse(input));
    if s.len() >= target_size {
        s[..target_size].to_owned()
    } else {
        dragon_string(&s, target_size)
    }
}

fn reverse(input: &str) -> String {
    input
        .chars()
        .rev()
        .map(|c| match c {
            '0' => '1',
            '1' => '0',
            _ => ' ',
        })
        .collect()
}
