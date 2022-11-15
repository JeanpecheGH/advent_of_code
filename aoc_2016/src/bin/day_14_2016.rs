use std::collections::HashMap;

fn main() {
    let input: &str = "ahsbgdzn";

    let now = std::time::Instant::now();
    let part1: usize = find_64th_key(input, 1);
    let elapsed = now.elapsed();
    println!(
        "Part1: The 64th key was found at index {} in {:?}",
        part1, elapsed
    );

    let now = std::time::Instant::now();
    let part2: usize = find_64th_key(input, 2017);
    let elapsed = now.elapsed();
    println!(
        "Part2: The 64th key was found at index {} in {:?}",
        part2, elapsed
    );
}

fn hash(word: String, times: usize) -> String {
    let mut digest = md5::compute(word);
    (0..times - 1).for_each(|_| digest = md5::compute(format!("{:x}", digest)));
    format!("{:x}", digest)
}

fn find_64th_key(input: &str, times_hash: usize) -> usize {
    let mut keys_index: Vec<usize> = Vec::new();
    let mut candidates: HashMap<usize, char> = HashMap::new();
    let mut i: usize = 1;
    loop {
        let my_word = format!("{input}{i}");
        let digest_str = hash(my_word, times_hash);
        let chars: Vec<char> = digest_str.chars().collect();
        match chars.windows(3).find_map(|trio| {
            if trio[0] == trio[1] && trio[0] == trio[2] {
                Some(trio[0])
            } else {
                None
            }
        }) {
            None => (),
            Some(c) => {
                //Since we have 3 of a kind, search 5 of a kind and promote valid candidates
                chars
                    .windows(5)
                    .filter_map(|quintet| {
                        if quintet[1..].iter().all(|&c| c == quintet[0]) {
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
        }

        //Remove obsolete candidates
        if i >= 1000 {
            candidates.remove(&(i - 1000));
        }
        if keys_index.len() >= 64 {
            break;
        }
        i += 1;
    }

    keys_index.sort();
    keys_index.into_iter().nth(63).unwrap()
}
