fn main() {
    let min_presents: u32 = 34000000;

    let mut part_1_house: Option<u32> = None;

    let mut n = 3;
    loop {
        let divs = divisors(n);
        //Part1
        let nb_presents_1: u32 = divs.iter().sum();
        if nb_presents_1 >= min_presents / 10 && part_1_house.is_none() {
            part_1_house = Some(n)
        }
        //Part2
        let nb_presents_2: u32 = divs.into_iter().filter(|&d| n <= 50 * d).sum();
        if nb_presents_2 * 11 >= min_presents {
            break;
        }
        n += 1;
    }

    println!(
        "Part1: The first house to receive at least {min_presents} presents is house {}",
        part_1_house.unwrap()
    );
    println!("Part2: The first house to receive at least {min_presents} presents is now house {n}");
}

fn divisors(n: u32) -> Vec<u32> {
    let sq = approximated_sqrt(n);
    let mut v: Vec<u32> = vec![1, n];
    for i in 2..sq {
        if n.is_multiple_of(i) {
            v.push(i);
            v.push(n / i);
        }
    }
    v
}

//Taken from https://docs.rs/divisors/latest/src/divisors/lib.rs.html#1-113
fn approximated_sqrt(n: u32) -> u32 {
    let mut num_bits = (std::mem::size_of::<u32>() << 3) - 1;
    while ((n >> num_bits) & 1) == 0 {
        num_bits -= 1;
    }

    1 << ((num_bits >> 1) + 1)
}
