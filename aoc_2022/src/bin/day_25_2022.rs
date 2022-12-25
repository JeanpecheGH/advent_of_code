fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2022/input/day_25.txt").expect("Cannot open input file");
    let sum: isize = s.lines().map(snafu_to_dec).sum();
    println!(
        "Part1: The sum of all the fuel requirements is {}, in snafu: {}",
        sum,
        dec_to_snafu(sum)
    );
    println!("Computing time: {:?}", now.elapsed());
}

fn snafu_to_dec(snafu: &str) -> isize {
    snafu.chars().fold(0, |mut acc, c| {
        acc *= 5;
        match c {
            '2' => acc += 2,
            '1' => acc += 1,
            '-' => acc -= 1,
            '=' => acc -= 2,
            _ => (),
        }
        acc
    })
}

fn dec_to_snafu(input: isize) -> String {
    let mut snafu: Vec<char> = Vec::new();
    let mut rest = input;
    while rest > 0 {
        let fivimal: isize = rest % 5;
        let c: char = match fivimal {
            1 => '1',
            2 => '2',
            3 => {
                rest += 2;
                '='
            }
            4 => {
                rest += 1;
                '-'
            }
            _ => '0',
        };
        snafu.push(c);
        rest /= 5;
    }
    snafu.iter().rev().collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122";

    #[test]
    fn part_1() {
        let sum: isize = INPUT.lines().map(|l| snafu_to_dec(l)).sum();
        assert_eq!(sum, 4890);
        assert_eq!(dec_to_snafu(sum), "2=-1=0");
    }

    #[test]
    fn test_big_number() {
        assert_eq!(dec_to_snafu(314159265), "1121-1110-1=0");
    }
}
