struct Order {
    n: usize,
    from: usize,
    to: usize,
}

struct Boat {
    stacks: Vec<Vec<char>>,
}

impl Boat {
    fn move_crates(&mut self, order: &Order) {
        for _ in 0..order.n {
            let c: char = self.stacks[order.from - 1].pop().unwrap();
            self.stacks[order.to - 1].push(c);
        }
    }

    fn move_crates_multiple(&mut self, order: &Order) {
        let len: usize = self.stacks[order.from - 1].len();
        let mut to_move: Vec<char> = self.stacks[order.from - 1].split_off(len - order.n);
        self.stacks[order.to - 1].append(&mut to_move);
    }

    fn top_crates(&self) -> String {
        self.stacks
            .iter()
            .map(|stack| stack.last().unwrap_or(&'?'))
            .collect()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_05.txt").expect("Cannot open input file");

    //Split stacks and orders
    let lines: Vec<&str> = s.lines().collect();
    let groups: Vec<&[&str]> = lines.split(|l| l.is_empty()).collect();

    let stacks: Vec<Vec<char>> = parse_stacks(groups[0]);

    let orders: Vec<Order> = groups[1]
        .iter()
        .map(|s| {
            let words: Vec<&str> = s.split_whitespace().collect();
            let n: usize = words[1].parse().unwrap();
            let from: usize = words[3].parse().unwrap();
            let to: usize = words[5].parse().unwrap();
            Order { n, from, to }
        })
        .collect();

    let mut boat = Boat {
        stacks: stacks.clone(),
    };
    orders.iter().for_each(|order| boat.move_crates(order));
    println!(
        "Part1: When moving crates 1 by 1, we find the crates {} on top",
        boat.top_crates()
    );

    let mut boat = Boat { stacks };
    orders
        .iter()
        .for_each(|order| boat.move_crates_multiple(order));
    println!(
        "Part2: When moving crates all together, we find the crates {} on top",
        boat.top_crates()
    );
}

fn parse_stacks(lines: &[&str]) -> Vec<Vec<char>> {
    //4 chars per line stack, except 3 for last stack
    let nb_stacks = lines[0].len() / 4 + 1;
    let mut stacks: Vec<Vec<char>> = vec![Vec::new(); nb_stacks];

    for line in lines {
        let chars: Vec<char> = line.chars().collect();
        for i in 0..nb_stacks {
            match chars[i * 4 + 1] {
                //The line with the indexes is ignored
                c if c.is_uppercase() => stacks[i].push(c),
                _ => (),
            }
        }
    }
    stacks.iter_mut().for_each(|s| s.reverse());
    stacks
}
