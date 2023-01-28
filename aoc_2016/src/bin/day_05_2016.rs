fn main() {
    let input = "abbhdwsy";

    let mut first_pwd = "".to_string();
    let mut second_pwd: [char; 8] = ['?'; 8];
    let mut i: u64 = 1;
    loop {
        let my_word = format!("{input}{i}");
        let digest = md5::compute(my_word);
        if digest[0] == 0 && digest[1] == 0 && digest[2] < 16 {
            println!("Digest: {digest:?}");
            let s = format!("{digest:?}");
            let c = s.chars().nth(5).unwrap();
            if first_pwd.len() < 8 {
                first_pwd.push(c);
            }
            let pos = c.to_digit(16).unwrap() as usize;
            if pos < 8 && second_pwd[pos] == '?' {
                let c2 = s.chars().nth(6).unwrap();
                second_pwd[pos] = c2;
                if second_pwd.iter().all(|&c| c != '?') {
                    break;
                }
            }
        }
        i += 1;
    }

    println!("Part1: The password is {first_pwd}");
    let pwd: String = second_pwd.iter().collect();
    println!("Part2: The new password is {pwd}");
}
