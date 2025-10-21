use std::cmp::Ordering;
use std::collections::HashMap;
use std::str::FromStr;

const CARGO: usize = 1_000_000_000_000;

#[derive(Debug, Eq, PartialEq)]
struct Chemical {
    name: String,
    quantity: usize,
}

impl Chemical {
    fn is(&self, other: &Chemical) -> bool {
        other.name == self.name
    }
}
impl FromStr for Chemical {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let words: Vec<&str> = s.split_whitespace().collect();
        let quantity: usize = words[0].parse().unwrap();
        let name: String = words[1].to_string();
        Ok(Self { name, quantity })
    }
}

#[derive(Debug, Eq, PartialEq)]
struct Reaction {
    input: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn input_contains(&self, other: &Chemical) -> bool {
        self.input.iter().any(|chem| chem.is(other))
    }
}

impl FromStr for Reaction {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(" => ").collect();
        let inputs: Vec<&str> = parts[0].split(", ").collect();
        let input: Vec<Chemical> = inputs.iter().map(|s| s.parse().unwrap()).collect();
        let output: Chemical = parts[1].parse().unwrap();

        Ok(Self { input, output })
    }
}

impl PartialOrd for Reaction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.input_contains(&other.output) {
            Some(Ordering::Less)
        } else if other.input_contains(&self.output) {
            Some(Ordering::Greater)
        } else {
            None
        }
    }
}

struct NanoFactory {
    reactions: Vec<Reaction>,
}

impl NanoFactory {
    fn max_fuel(&self, cargo: usize) -> usize {
        let mut min: usize = self.needed_ore(1);
        min = cargo / min;

        let mut max: usize = min + min / 3;
        let mut curr: usize = (min + max) / 2;

        while min != max - 1 {
            let res: usize = self.needed_ore(curr);
            if res < cargo {
                min = curr;
                curr = (curr + max) / 2;
            } else {
                max = curr;
                curr = (curr + min) / 2;
            }
        }

        min
    }

    fn needed_ore(&self, target: usize) -> usize {
        let mut chem_map: HashMap<String, usize> = HashMap::new();
        chem_map.insert("FUEL".to_string(), target);

        for reaction in self.reactions.iter() {
            let target: usize = chem_map.get(&reaction.output.name).copied().unwrap();
            let quantity: usize = reaction.output.quantity;
            let nb_react: usize = if target.is_multiple_of(quantity) {
                target / quantity
            } else {
                target / quantity + 1
            };
            for input in reaction.input.iter() {
                let entry = chem_map.entry(input.name.clone()).or_insert(0);
                *entry += input.quantity * nb_react;
            }
        }
        chem_map.get("ORE").copied().unwrap()
    }

    fn sort(&mut self) {
        let mut result: Vec<Reaction> = Vec::new();
        while let Some(r) = self.reactions.pop() {
            if self
                .reactions
                .iter()
                .any(|reaction| r.partial_cmp(reaction) == Some(Ordering::Greater))
            {
                self.reactions.push(r);
                self.reactions.rotate_right(1);
            } else {
                result.push(r);
            }
        }
        self.reactions = result;
    }
}

impl FromStr for NanoFactory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let reactions: Vec<Reaction> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Self { reactions })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_14.txt").expect("Cannot open input file");
    let mut factory: NanoFactory = s.parse().unwrap();
    factory.sort();
    println!(
        "Part1: The minimum amount of ORE required to produce 1 FUEL is {}",
        factory.needed_ore(1)
    );
    println!(
        "Part2: The maximum amount of FUEL produced with {CARGO} ORE is {}",
        factory.max_fuel(CARGO)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = "10 ORE => 10 A
1 ORE => 1 B
7 A, 1 B => 1 C
7 A, 1 C => 1 D
7 A, 1 D => 1 E
7 A, 1 E => 1 FUEL";

    const INPUT_2: &str = "9 ORE => 2 A
8 ORE => 3 B
7 ORE => 5 C
3 A, 4 B => 1 AB
5 B, 7 C => 1 BC
4 C, 1 A => 1 CA
2 AB, 3 BC, 4 CA => 1 FUEL";

    const INPUT_3: &str = "157 ORE => 5 NZVS
165 ORE => 6 DCFZ
44 XJWVT, 5 KHKGT, 1 QDVJ, 29 NZVS, 9 GPVTF, 48 HKGWZ => 1 FUEL
12 HKGWZ, 1 GPVTF, 8 PSHF => 9 QDVJ
179 ORE => 7 PSHF
177 ORE => 5 HKGWZ
7 DCFZ, 7 PSHF => 2 XJWVT
165 ORE => 2 GPVTF
3 DCFZ, 7 NZVS, 5 HKGWZ, 10 PSHF => 8 KHKGT";

    const INPUT_4: &str = "2 VPVL, 7 FWMGM, 2 CXFTF, 11 MNCFX => 1 STKFG
17 NVRVD, 3 JNWZP => 8 VPVL
53 STKFG, 6 MNCFX, 46 VJHF, 81 HVMC, 68 CXFTF, 25 GNMV => 1 FUEL
22 VJHF, 37 MNCFX => 5 FWMGM
139 ORE => 4 NVRVD
144 ORE => 7 JNWZP
5 MNCFX, 7 RFSQX, 2 FWMGM, 2 VPVL, 19 CXFTF => 3 HVMC
5 VJHF, 7 MNCFX, 9 VPVL, 37 CXFTF => 6 GNMV
145 ORE => 6 MNCFX
1 NVRVD => 8 CXFTF
1 VJHF, 6 MNCFX => 4 RFSQX
176 ORE => 6 VJHF";

    const INPUT_5: &str = "171 ORE => 8 CNZTR
7 ZLQW, 3 BMBT, 9 XCVML, 26 XMNCP, 1 WPTQ, 2 MZWV, 1 RJRHP => 4 PLWSL
114 ORE => 4 BHXH
14 VRPVC => 6 BMBT
6 BHXH, 18 KTJDG, 12 WPTQ, 7 PLWSL, 31 FHTLT, 37 ZDVW => 1 FUEL
6 WPTQ, 2 BMBT, 8 ZLQW, 18 KTJDG, 1 XMNCP, 6 MZWV, 1 RJRHP => 6 FHTLT
15 XDBXC, 2 LTCX, 1 VRPVC => 6 ZLQW
13 WPTQ, 10 LTCX, 3 RJRHP, 14 XMNCP, 2 MZWV, 1 ZLQW => 1 ZDVW
5 BMBT => 4 WPTQ
189 ORE => 9 KTJDG
1 MZWV, 17 XDBXC, 3 XCVML => 2 XMNCP
12 VRPVC, 27 CNZTR => 2 XDBXC
15 KTJDG, 12 BHXH => 5 XCVML
3 BHXH, 2 VRPVC => 7 MZWV
121 ORE => 7 VRPVC
7 XCVML => 6 RJRHP
5 BHXH, 4 VRPVC => 5 LTCX";

    #[test]
    fn test_1_part_1() {
        let mut factory: NanoFactory = INPUT_1.parse().unwrap();
        factory.sort();
        assert_eq!(factory.needed_ore(1), 31);
    }

    #[test]
    fn test_2_part_1() {
        let mut factory: NanoFactory = INPUT_2.parse().unwrap();
        factory.sort();
        assert_eq!(factory.needed_ore(1), 165);
    }

    #[test]
    fn test_3_part_1() {
        let mut factory: NanoFactory = INPUT_3.parse().unwrap();
        factory.sort();
        assert_eq!(factory.needed_ore(1), 13312);
        assert_eq!(factory.max_fuel(CARGO), 82892753);
    }

    #[test]
    fn test_4_part_1() {
        let mut factory: NanoFactory = INPUT_4.parse().unwrap();
        factory.sort();
        assert_eq!(factory.needed_ore(1), 180697);
        assert_eq!(factory.max_fuel(CARGO), 5586022);
    }

    #[test]
    fn test_5_part_1() {
        let mut factory: NanoFactory = INPUT_5.parse().unwrap();
        factory.sort();
        assert_eq!(factory.needed_ore(1), 2210736);
        assert_eq!(factory.max_fuel(CARGO), 460664);
    }
}
