use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    let input: &str = "ahsbgdzn";

    let now = std::time::Instant::now();
    let part1: usize = find_nth_key(input, 1, 63);
    let elapsed = now.elapsed();
    println!("Part1: The 64th key was found at index {part1} in {elapsed:?}");

    let now = std::time::Instant::now();
    let part2: usize = find_nth_key(input, 2017, 63);
    let elapsed = now.elapsed();
    println!("Part2: The 64th key was found at index {part2} in {elapsed:?}");
}

const HEX_ARRAY: &[u8; 16] = b"0123456789abcdef";
fn bytes_to_hex(bytes: [u8; 16]) -> [u8; 32] {
    let mut hex: [u8; 32] = [0; 32];
    for i in 0..bytes.len() {
        let v: usize = bytes[i] as usize;
        hex[i * 2] = HEX_ARRAY[v >> 4];
        hex[i * 2 + 1] = HEX_ARRAY[v & 0x0F];
    }
    hex
}

fn hash(word: String, times: usize) -> String {
    let mut digest = md5::compute(word);
    (0..times - 1).for_each(|_| digest = md5::compute(bytes_to_hex(digest.0)));
    format!("{digest:x}")
}

fn find_nth_key(input: &str, times_hash: usize, n: usize) -> usize {
    let mut keys_index: Vec<usize> = Vec::new();
    let mut candidates: HashMap<usize, char> = HashMap::new();
    let mut i: usize = 0;
    let mut end_trigger = usize::MAX;
    loop {
        let my_word = format!("{input}{i}");
        let digest_str = hash(my_word, times_hash);
        let chars: Vec<char> = digest_str.chars().collect();
        let opt_char: Option<char> = chars.windows(3).find_map(|trio| {
            if trio.iter().all_equal() {
                Some(trio[0])
            } else {
                None
            }
        });
        if let Some(c) = opt_char {
            //Since we have 3 of a kind, search 5 of a kind and promote valid candidates
            chars
                .windows(5)
                .filter_map(|quintet| {
                    if quintet.iter().all_equal() {
                        Some(quintet[0])
                    } else {
                        None
                    }
                })
                .for_each(|c| {
                    candidates.retain(|index, ch| {
                        if c == *ch {
                            keys_index.push(*index);
                            false
                        } else {
                            true
                        }
                    })
                });
            candidates.insert(i, c);
        }

        //Remove obsolete candidates
        if i >= 1000 {
            candidates.remove(&(i - 1000));
        }
        // We can only end ~1000 hash after we discovered the first 64 hash
        // because some valid hash could still be waiting for its 5 char validator
        if keys_index.len() >= 64 && end_trigger == usize::MAX {
            end_trigger = i + 1000;
        }
        if i >= end_trigger {
            break;
        }
        i += 1;
    }

    keys_index.sort();
    keys_index[n]
}
