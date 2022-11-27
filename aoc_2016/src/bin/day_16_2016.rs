use std::cmp::{max, min};

fn main() {
    let input: &str = "11100010111110100";

    println!("Input: {}", input);
    println!("Reverse: {}", reverse(input));

    let now = std::time::Instant::now();
    //Part1 optimized
    let target_size: usize = 272;
    let mut chunk_size = 1;
    while target_size % (chunk_size * 2) == 0 {
        chunk_size *= 2;
    }
    let cs: String = (0..target_size / chunk_size)
        .map(|i| checksum_chunk(input, i * chunk_size, chunk_size))
        .collect();
    println!(
        "Part1: The checksum for target size {} is {}, found in {:?}",
        target_size,
        cs,
        now.elapsed()
    );

    let now = std::time::Instant::now();
    //Part2 optimized
    let target_size: usize = 35651584;
    let mut chunk_size = 1;
    while target_size % (chunk_size * 2) == 0 {
        chunk_size *= 2;
    }
    let cs: String = (0..target_size / chunk_size)
        .map(|i| checksum_chunk(input, i * chunk_size, chunk_size))
        .collect();
    println!(
        "Part2: The checksum for target size {} is {}, found in {:?}",
        target_size,
        cs,
        now.elapsed()
    );
}

fn checksum_chunk(input: &str, chunk_start: usize, chunk_size: usize) -> char {
    let len: usize = input.len() + 1;
    let rev: String = reverse(input);
    let chunk_end: usize = chunk_start + chunk_size;
    let start_pairs: usize = if chunk_start % (len * 2) == 0 {
        chunk_start
    } else {
        chunk_start + len * 2 - chunk_start % (len * 2)
    };
    let end_pairs: usize = max(chunk_end - chunk_end % (len * 2), start_pairs);
    let offset: usize = if end_pairs > (chunk_end + 1) {
        end_pairs - (chunk_end + 1)
    } else {
        0
    };
    //Count nb of 1 in prefix
    let nb_prefix = affix_ones(
        input,
        &rev,
        min(start_pairs as isize - 1, chunk_end as isize) - chunk_start as isize,
        offset,
        true,
        len,
    );
    //Count nb of input/rev pairs (this parity is the same as the number of ones in all pairs combined)
    let nb_pairs = (end_pairs - start_pairs) / (len * 2) * (len - 1);
    //Count nb of 1 in delimiters
    let nb_delim = delimiters_ones(chunk_start, chunk_end, len);
    //Count nb of 1 in suffix
    let suffix_size: usize = if chunk_end >= end_pairs {
        chunk_end - end_pairs
    } else {
        0
    };
    let nb_suffix = affix_ones(input, &rev, suffix_size as isize, 0, false, len);

    if (nb_prefix + nb_pairs + nb_delim + nb_suffix) % 2 == 0 {
        '1'
    } else {
        '0'
    }
}

fn affix_ones(
    input: &str,
    rev: &str,
    size: isize,
    offset: usize,
    from_end: bool,
    len: usize,
) -> usize {
    if size <= 0 {
        return 0;
    }
    let actual_size = if (size as usize + offset) >= len {
        size as usize - 1
    } else {
        size as usize
    };
    let affixes = format!("{}{}", input, rev);
    let mut chars: Vec<char> = affixes.chars().collect();
    if from_end {
        chars = chars.into_iter().rev().collect();
    }
    chars
        .into_iter()
        .skip(offset)
        .take(actual_size)
        .filter(|&c| c == '1')
        .count()
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

fn delimiters_ones(start: usize, end: usize, div: usize) -> usize {
    let s = if (start + 1) % div == 0 {
        (start + 1) / div
    } else {
        ((start + 1) / div) + 1
    };
    let e: usize = ((end + 1) / div) + 1;
    (s..e)
        .map(|n| nth_delimiter(n as isize))
        .filter(|&d| d)
        .count()
}

//First delimiter has index 1
fn nth_delimiter(n: isize) -> bool {
    (((-n & n) << 1) & n) != 0
}
