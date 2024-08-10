use util::duet_tablet::DuetTablet;

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_18.txt").expect("Cannot open input file");
    let duet: DuetTablet = s.parse().unwrap();

    println!("Part1: The recovered frenquency is {}", duet.play_solo());
    println!(
        "Part2: The program with ID 1 sent {} packets",
        duet.play_duo()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "set a 1
add a 2
mul a a
mod a 5
snd a
set a 0
rcv a
jgz a -1
set a 1
jgz a -2
";

    const EXAMPLE_2: &str = "snd 1
snd 2
snd p
rcv a
rcv b
rcv c
rcv d
";

    #[test]
    fn part_1() {
        let duet: DuetTablet = EXAMPLE_1.parse().unwrap();
        assert_eq!(4, duet.play_solo());
    }

    #[test]
    fn part_2() {
        let duet: DuetTablet = EXAMPLE_2.parse().unwrap();
        assert_eq!(3, duet.play_duo());
    }
}
