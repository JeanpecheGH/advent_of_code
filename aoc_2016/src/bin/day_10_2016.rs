use std::collections::HashMap;

#[derive(Copy, Clone, Debug)]
enum Target {
    Bot(u16),
    Output(u16),
}

struct Out {
    value: u16,
    target: Target,
}

#[derive(Debug)]
struct Bot {
    id: u16,
    left: Option<u16>,
    right: Option<u16>,
    high: Target,
    low: Target,
}

impl Bot {
    fn distribute(&mut self) -> Option<(Out, Out)> {
        match (self.left, self.right) {
            (Some(a), Some(b)) => {
                if a == 17 && b == 61 || a == 61 && b == 17 {
                    println!("Part1: I'm bot {}, comparing {} and {}", self.id, a, b);
                }
                self.left = None;
                self.right = None;
                Some((
                    Out {
                        value: a,
                        target: self.high,
                    },
                    Out {
                        value: b,
                        target: self.low,
                    },
                ))
            }
            _ => None,
        }
    }

    fn is_full(&self) -> bool {
        self.left.is_some() && self.right.is_some()
    }

    fn receive(&mut self, n: u16) {
        match (self.left, self.right) {
            (Some(_), Some(_)) => (),
            (Some(a), _) => {
                if a > n {
                    self.right = Some(n);
                } else {
                    self.right = Some(a);
                    self.left = Some(n);
                }
            }
            _ => self.left = Some(n),
        }
    }
}

#[derive(Debug)]
struct Factory {
    bots: HashMap<u16, Bot>,
    outputs: HashMap<u16, u16>,
}

impl Factory {
    fn process(&mut self) {
        while let Some(bot) = self.bots.values_mut().find(|b| b.is_full()) {
            if let Some((high, low)) = bot.distribute() {
                self.give_to_target(high);
                self.give_to_target(low);
            }
        }
    }

    fn add_bot(&mut self, bot: Bot) {
        self.bots.insert(bot.id, bot);
    }

    fn give_to_target(&mut self, out: Out) {
        match out.target {
            Target::Bot(i) => self.give_to_bot(i, out.value),
            Target::Output(i) => self.give_to_output(i, out.value),
        }
    }
    fn give_to_bot(&mut self, i: u16, n: u16) {
        if let Some(bot) = self.bots.get_mut(&i) {
            bot.receive(n);
        }
    }

    fn give_to_output(&mut self, i: u16, n: u16) {
        self.outputs.insert(i, n);
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_10.txt").expect("Cannot open input file");
    let (bots_lines, input_lines): (Vec<String>, Vec<String>) = lines
        .map(|l| l.unwrap())
        .partition(|s| s.starts_with("bot"));

    let mut factory: Factory = bots_lines
        .iter()
        .map(|l| {
            let words: Vec<&str> = l.split_whitespace().collect();
            //ex: bot 109 gives low to output 9 and high to bot 126
            Bot {
                id: words[1].parse().unwrap(),
                left: None,
                right: None,
                high: build_target(words[10], words[11]),
                low: build_target(words[5], words[6]),
            }
        })
        .fold(
            Factory {
                bots: HashMap::new(),
                outputs: HashMap::new(),
            },
            |mut acc, bot| {
                acc.add_bot(bot);
                acc
            },
        );

    input_lines.iter().for_each(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        //ex: value 13 goes to bot 169
        factory.give_to_bot(words[5].parse().unwrap(), words[1].parse().unwrap())
    });

    factory.process();

    let product: u16 = factory.outputs.get(&0).unwrap()
        * factory.outputs.get(&1).unwrap()
        * factory.outputs.get(&2).unwrap();
    println!("Part2: The product is {product}");
}

fn build_target(target_type: &str, target_id: &str) -> Target {
    let id: u16 = target_id.parse().unwrap();
    match target_type {
        "bot" => Target::Bot(id),
        _ => Target::Output(id),
    }
}
