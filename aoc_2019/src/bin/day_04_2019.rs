const MIN: usize = 165432;
const MAX: usize = 707912;

fn main() {
    let now = std::time::Instant::now();
    let (part_1, part_2) = number_valid_pwd(MIN, MAX);
    println!("Part1: There are {part_1} valid passwords");
    println!("Part2: There are now {part_2} valid passwords");
    println!("Computing time: {:?}", now.elapsed());
}

fn number_valid_pwd(min: usize, max: usize) -> (usize, usize) {
    let mut nb_part1: usize = 0;
    let mut nb_part2: usize = 0;

    //We know any number over 700000 will be invalid
    for one in 1..=6 {
        for two in one..=9 {
            for three in two..=9 {
                for four in three..=9 {
                    for five in four..=9 {
                        for six in five..=9 {
                            if one == two
                                || two == three
                                || three == four
                                || four == five
                                || five == six
                            {
                                let n: usize = one * 100_000
                                    + two * 10_000
                                    + three * 1_000
                                    + four * 100
                                    + five * 10
                                    + six;
                                if n >= min && n <= max {
                                    nb_part1 += 1;
                                    if (one == two && two != three)
                                        || (two == three && three != four && one != two)
                                        || (three == four && four != five && two != three)
                                        || (four == five && five != six && three != four)
                                        || (five == six && four != five)
                                    {
                                        nb_part2 += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    (nb_part1, nb_part2)
}
