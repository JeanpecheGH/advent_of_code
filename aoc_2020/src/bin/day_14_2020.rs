use std::collections::HashMap;
use std::str::FromStr;

const SIZE: usize = 36;

#[derive(Debug, Copy, Clone)]
enum Action {
    Mask([Option<bool>; SIZE]),
    Mem(usize, usize),
}

impl FromStr for Action {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split(&['[', ']', ' ']).collect();
        match words[0] {
            "mask" => {
                let mut bits: [Option<bool>; SIZE] = [None; SIZE];
                words[2].chars().enumerate().for_each(|(i, c)| match c {
                    '0' => bits[i] = Some(false),
                    '1' => bits[i] = Some(true),
                    _ => bits[i] = None,
                });
                Ok(Action::Mask(bits))
            }
            "mem" => Ok(Action::Mem(
                words[1].parse().unwrap(),
                words[4].parse().unwrap(),
            )),
            _ => Err(()),
        }
    }
}

#[derive(Debug)]
struct Memory {
    actions: Vec<Action>,
    mask: [Option<bool>; SIZE],
    memory: HashMap<usize, usize>,
}

impl Memory {
    fn actions(&mut self, mask_address: bool) {
        let actions: Vec<Action> = self.actions.clone();
        for ac in actions {
            self.action(&ac, mask_address);
        }
    }

    fn action(&mut self, action: &Action, mask_address: bool) {
        match action {
            Action::Mask(bits) => self.mask.clone_from_slice(bits),
            Action::Mem(addr, value) => {
                if mask_address {
                    let addresses: Vec<usize> = self.masked_adresses(*addr);
                    addresses.into_iter().for_each(|ad| {
                        self.memory.insert(ad, *value);
                    });
                } else {
                    self.memory.insert(*addr, self.masked_value(*value));
                }
            }
        }
    }

    fn masked_adresses(&self, addr: usize) -> Vec<usize> {
        fn compute_mask(
            addr_chars: &[char],
            mask: &[Option<bool>],
            idx: usize,
            current: Vec<char>,
        ) -> Vec<Vec<char>> {
            if idx == addr_chars.len() {
                vec![current]
            } else {
                match mask[idx] {
                    Some(true) => {
                        let mut new_current = current;
                        new_current.push('1');
                        compute_mask(addr_chars, mask, idx + 1, new_current)
                    }
                    Some(false) => {
                        let mut new_current = current;
                        new_current.push(addr_chars[idx]);
                        compute_mask(addr_chars, mask, idx + 1, new_current)
                    }
                    None => {
                        let mut with_zero = current.clone();
                        with_zero.push('0');
                        let mut with_one = current;
                        with_one.push('1');
                        let mut ret_vec: Vec<Vec<char>> =
                            compute_mask(addr_chars, mask, idx + 1, with_zero);
                        ret_vec.extend(compute_mask(addr_chars, mask, idx + 1, with_one));
                        ret_vec
                    }
                }
            }
        }

        let binary: String = format!("{addr:036b}");
        let binary_chars: Vec<char> = binary.chars().collect();
        let char_addresses: Vec<Vec<char>> = compute_mask(&binary_chars, &self.mask, 0, Vec::new());
        char_addresses
            .into_iter()
            .map(|chars| {
                let s: String = chars.iter().collect();
                usize::from_str_radix(&s, 2).unwrap()
            })
            .collect()
    }

    fn masked_value(&self, value: usize) -> usize {
        let binary: String = format!("{value:036b}");
        let masked_binary: String = binary
            .chars()
            .enumerate()
            .map(|(i, c)| match self.mask[i] {
                Some(true) => '1',
                Some(false) => '0',
                None => c,
            })
            .collect();
        usize::from_str_radix(&masked_binary, 2).unwrap()
    }

    fn sum(&self) -> usize {
        self.memory.values().sum()
    }

    fn clear(&mut self) {
        self.memory.clear();
    }
}

impl FromStr for Memory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let actions: Vec<Action> = s.lines().map(|l| l.parse().unwrap()).collect();
        let mask: [Option<bool>; SIZE] = [None; SIZE];
        let memory = HashMap::new();

        Ok(Memory {
            actions,
            mask,
            memory,
        })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2020/input/day_14.txt").expect("Cannot open input file");
    let mut memory: Memory = s.parse().unwrap();
    memory.actions(false);
    println!(
        "Part1: When using masked values, the sum of values in memory is {}",
        memory.sum()
    );
    memory.clear();
    memory.actions(true);
    println!(
        "Part2: When using masked addresses, the sum of values in memory is {}",
        memory.sum()
    );

    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0";

    const INPUT_2: &str = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";

    #[test]
    fn part_1() {
        let mut memory: Memory = INPUT_1.parse().unwrap();
        memory.actions(false);
        assert_eq!(memory.sum(), 165);
    }

    #[test]
    fn part_2() {
        let mut memory: Memory = INPUT_2.parse().unwrap();
        memory.actions(true);
        assert_eq!(memory.sum(), 208);
    }
}
