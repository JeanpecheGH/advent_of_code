fn main() {
    let mut input: [u8; 8] = ['c', 'q', 'j', 'x', 'j', 'n', 'd', 's'].map(|c| c as u8);
    println!("{:?}", input);

    while !is_valid(input) {
        input = next(input);
    }
    println!(
        "Part1: The next password is {:?}",
        input.iter().map(|&n| n as char).collect::<String>()
    );

    input = next(input);
    while !is_valid(input) {
        input = next(input);
    }
    println!(
        "Part2: The next next password is {:?}",
        input.iter().map(|&n| n as char).collect::<String>()
    );
}

fn next(mut input: [u8; 8]) -> [u8; 8] {
    let mut i = 7;
    loop {
        match input[i] as char {
            'z' => {
                input[i] = b'a';
                i -= 1;
            }
            _ => {
                input[i] += 1;
                break;
            }
        }
    }
    input
}

fn is_valid(input: [u8; 8]) -> bool {
    contains_suite(input) && !has_invalid_chars(input) && has_two_doubles(input)
}

fn contains_suite(input: [u8; 8]) -> bool {
    input
        .windows(3)
        .any(|triple| triple[1] == triple[0] + 1 && triple[2] == triple[1] + 1)
}

fn has_invalid_chars(input: [u8; 8]) -> bool {
    let invalids: [u8; 3] = ['i', 'o', 'l'].map(|c| c as u8);
    input.iter().any(|c| invalids.contains(c))
}

fn has_two_doubles(input: [u8; 8]) -> bool {
    let pair_indexes: Vec<usize> = input
        .windows(2)
        .enumerate()
        .filter_map(|(i, p)| if p[0] == p[1] { Some(i) } else { None })
        .collect();
    pair_indexes.windows(2).any(|pair| pair[1] > pair[0] + 1)
}
