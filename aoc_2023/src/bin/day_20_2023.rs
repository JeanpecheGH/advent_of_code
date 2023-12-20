use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, one_of};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::collections::{HashMap, VecDeque};
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Signal {
    high: bool,
    from: String,
    dest: String,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum ModuleType {
    Broadcast,
    FlipFlop,
    Conjunction,
}

impl ModuleType {
    fn from_char(opt_c: Option<char>) -> ModuleType {
        match opt_c {
            None => ModuleType::Broadcast,
            Some('%') => ModuleType::FlipFlop,
            Some('&') => ModuleType::Conjunction,
            _ => panic!("Invalid Module Type [{opt_c:?}]"),
        }
    }
}

#[derive(Clone, Debug)]
struct Module {
    id: String,
    mod_type: ModuleType,
    on: bool,
    cache: HashMap<String, bool>,
    dest: Vec<String>,
}

impl Module {
    fn process_signal(&mut self, signal: Signal) -> Vec<Signal> {
        match (self.mod_type, signal.high) {
            (ModuleType::Broadcast, _) => self.send_signals(signal.high),
            (ModuleType::FlipFlop, true) => Vec::new(), //High pulse on Flip-flip, nothing to do
            (ModuleType::FlipFlop, false) => {
                self.on = !self.on;
                self.send_signals(self.on)
            }
            (ModuleType::Conjunction, _) => {
                let e = self.cache.entry(signal.from).or_default();
                *e = signal.high;
                let out_high: bool = self.cache.values().any(|&c| !c);
                self.send_signals(out_high)
            }
        }
    }

    fn send_signals(&self, high: bool) -> Vec<Signal> {
        self.dest
            .iter()
            .map(|d| Signal {
                high,
                from: self.id.clone(),
                dest: d.clone(),
            })
            .collect()
    }
}

impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_module(s: &str) -> IResult<&str, Module> {
            let (s, mod_type) = map(opt(one_of("%&")), ModuleType::from_char)(s)?;
            let (s, id) = alpha1(s)?;
            let (s, dest) = preceded(tag(" -> "), separated_list1(tag(", "), alpha1))(s)?;
            let dest: Vec<String> = dest.into_iter().map(|d| d.to_string()).collect();

            Ok((
                s,
                Module {
                    id: id.to_string(),
                    mod_type,
                    on: false,
                    cache: HashMap::new(),
                    dest,
                },
            ))
        }
        Ok(parse_module(s).unwrap().1)
    }
}

#[derive(Clone, Debug)]
struct PulseSystem {
    modules: HashMap<String, Module>,
}

impl PulseSystem {
    fn low_to_module(&mut self, output: &str) -> usize {
        self.reset();
        //Get the direct parent of "output", it is a Conjunction module
        let rx_parent: String = self
            .modules
            .values()
            .find_map(|m| {
                if m.dest.contains(&output.to_string()) {
                    Some(m.id.clone())
                } else {
                    None
                }
            })
            .unwrap();
        //Get the number of parents of the parent of "output".
        let nb_parents: usize = self.modules.get(&rx_parent).unwrap().cache.len();

        //We get the size of the cycle for each of the parent
        let mut cycles: Vec<usize> = Vec::new();
        let mut i: usize = 1;
        while cycles.len() < nb_parents {
            let (_, _, opt_cycle) = self.push_button(i, &rx_parent);
            if let Some(c) = opt_cycle {
                cycles.push(c);
            }
            i += 1;
        }
        //Get the LCM of all the cycles length (4 primes, we just compute the product)
        cycles.into_iter().product()
    }

    fn push_times(&mut self, n: usize) -> usize {
        self.reset();

        let mut low_count: usize = 0;
        let mut high_count: usize = 0;

        for i in 0..n {
            let (l, h, _) = self.push_button(i, "output");
            low_count += l;
            high_count += h;
        }

        low_count * high_count
    }
    fn reset(&mut self) {
        //Set all flip_flop to off
        self.modules.values_mut().for_each(|m| m.on = false);

        //Set all Conjunction caches to low for each input
        //Get all Conjuction Ids
        let conj: Vec<String> = self
            .modules
            .values()
            .filter_map(|m| {
                if m.mod_type == ModuleType::Conjunction {
                    Some(m.id.clone())
                } else {
                    None
                }
            })
            .collect();

        //Get all Inputs for each Conjuction
        let mut inputs: HashMap<String, Vec<String>> = HashMap::new();

        self.modules.values().for_each(|m| {
            conj.iter().for_each(|out| {
                if m.dest.contains(out) {
                    let v = inputs.entry(out.clone()).or_default();
                    v.push(m.id.clone())
                }
            })
        });

        //Set cache to low
        inputs.into_iter().for_each(|(id, ins)| {
            let module = self.modules.get_mut(&id).unwrap();
            ins.into_iter().for_each(|input| {
                let _ = module.cache.insert(input, false);
            });
        })
    }

    fn push_button(&mut self, i: usize, last_and: &str) -> (usize, usize, Option<usize>) {
        let mut low_count: usize = 0;
        let mut high_count: usize = 0;
        let mut cycle: Option<usize> = None;

        let mut to_process: VecDeque<Signal> = VecDeque::new();
        let button_signal: Signal = Signal {
            high: false,
            from: "button".to_string(),
            dest: "broadcaster".to_string(),
        };
        to_process.push_back(button_signal);

        while let Some(signal) = to_process.pop_front() {
            if signal.high {
                high_count += 1;
            } else {
                low_count += 1;
            }

            if last_and == signal.dest && signal.high {
                cycle = Some(i)
            }

            if let Some(module) = self.modules.get_mut(&signal.dest) {
                let new_signals: Vec<Signal> = module.process_signal(signal);

                to_process.extend(new_signals);
            }
        }

        (low_count, high_count, cycle)
    }
}

impl FromStr for PulseSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let modules: HashMap<String, Module> = s
            .lines()
            .map(|l| {
                let module: Module = l.parse().unwrap();
                (module.id.clone(), module)
            })
            .collect();
        Ok(PulseSystem { modules })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_20.txt").expect("Cannot open input file");
    let mut system: PulseSystem = s.parse().unwrap();
    println!(
        "Part1: After pushing the button 1000 times, the product is {}",
        system.push_times(1000)
    );
    println!(
        "Part2: We need to push the button {} times to send a low pulse to the rx module",
        system.low_to_module("rx")
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a
";
    const EXAMPLE_2: &str = "broadcaster -> a
%a -> inv, con
&inv -> b
%b -> con
&con -> output
";

    #[test]
    fn part_1_test_1() {
        let mut system: PulseSystem = EXAMPLE_1.parse().unwrap();
        println!("{:?}", system);
        system.reset();
        println!("{:?}", system);
        assert_eq!(system.push_times(1000), 32000000);
    }
    #[test]
    fn part_1_test_2() {
        let mut system: PulseSystem = EXAMPLE_2.parse().unwrap();
        assert_eq!(system.push_times(1000), 11687500);
    }
}
