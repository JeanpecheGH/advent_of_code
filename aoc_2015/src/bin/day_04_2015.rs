fn main() {
    let input = "yzbqklnj";

    let mut i: u64 = 1;
    loop {
        let my_word = format!("{input}{i}");
        let digest = md5::compute(my_word);
        let hexa = format!("{digest:x}");
        if hexa.starts_with("00000") {
            println!("Part1: {i}");
            break;
        }
        i += 1;
    }

    i = 1;
    loop {
        let my_word = format!("{input}{i}");
        let digest = md5::compute(my_word);
        let hexa = format!("{digest:x}");
        if hexa.starts_with("000000") {
            println!("Part2: {i}");
            break;
        }
        if i.is_multiple_of(1_000_000) {
            println!("{i}");
        }
        i += 1;
    }
}
