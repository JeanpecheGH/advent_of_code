fn main() {
    let now = std::time::Instant::now();
    let card_key: usize = 15_335_876;
    let door_key: usize = 15_086_442;
    let card_loops: usize = count_loops(7, card_key);
    let encryption_key: usize = apply_loops(door_key, card_loops);
    //This sides use way more loops, don't compute it
    // let door_loops: usize = count_loops(7, door_key);
    // let encryption_key: usize = apply_loops(card_key, door_loops);
    println!("Part1: The computed encryption key is {}", encryption_key);
    println!("Computing time: {:?}", now.elapsed());
}

fn transform(value: usize, subject: usize) -> usize {
    (value * subject) % 20_201_227
}

fn count_loops(subject: usize, target: usize) -> usize {
    let mut nb_loop: usize = 0;
    let mut value: usize = 1;
    while value != target {
        value = transform(value, subject);
        nb_loop += 1;
    }
    nb_loop
}

fn apply_loops(subject: usize, loops: usize) -> usize {
    let mut value = 1;
    for _ in 0..loops {
        value = transform(value, subject);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    const CARD_KEY: usize = 5_764_801;
    const DOOR_KEY: usize = 17_807_724;

    #[test]
    fn test() {
        let card_loops: usize = count_loops(7, CARD_KEY);
        assert_eq!(card_loops, 8);
        let door_loops: usize = count_loops(7, DOOR_KEY);
        assert_eq!(door_loops, 11);
        let encryption_key: usize = apply_loops(DOOR_KEY, card_loops);
        let encryption_key_2: usize = apply_loops(CARD_KEY, door_loops);
        assert_eq!(encryption_key, encryption_key_2);
        assert_eq!(encryption_key, 14_897_079);
    }
}
