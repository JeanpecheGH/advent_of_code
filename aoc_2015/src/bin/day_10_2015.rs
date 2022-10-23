use std::char::from_digit;

fn main() {
    let input = "1113122113".to_string();

    let n_40 = look_and_say(input.clone(), 40).len();
    let n_50 = look_and_say(input, 50).len();

    println!("Part1: Result length is {}", n_40);
    println!("Part2: Result length is {}", n_50);
}

fn look_and_say(str: String, n: u16) -> String {
    if n == 0 {
        return str;
    }
    let mut v: Vec<char> = Vec::new();

    let mut it = str.chars().peekable();
    let mut store: u32 = 0;
    while let Some(c) = it.next() {
        match it.peek() {
            Some(&c_n) if c == c_n => store += 1,
            _ => {
                v.push(from_digit(store + 1, 10).unwrap());
                v.push(c);
                store = 0;
            }
        }
    }
    look_and_say(v.iter().collect(), n - 1)
}
