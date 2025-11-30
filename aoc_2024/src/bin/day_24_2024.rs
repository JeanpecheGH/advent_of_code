use std::{collections::VecDeque, str::FromStr};

use fxhash::FxHashMap;
use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    sequence::{preceded, separated_pair},
    IResult,
};
use util::{basic_parser::parse_usize, split_blocks};

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
enum LogicOp {
    And,
    Or,
    Xor,
}

impl FromStr for LogicOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(LogicOp::And),
            "OR" => Ok(LogicOp::Or),
            "XOR" => Ok(LogicOp::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Clone)]
struct LogicGate {
    left: String,
    right: String,
    output: String,
    op: LogicOp,
}

impl LogicGate {
    fn emulate(
        &self,
        inputs: &mut FxHashMap<String, bool>,
        translate: &mut FxHashMap<String, String>,
    ) -> bool {
        //translate
        if self.left.starts_with("x") || self.left.starts_with("y") {
            let index = &self.left[1..];
            let new_name: String = match self.op {
                LogicOp::And => format!("a{}", index),
                LogicOp::Or => format!("o{}", index),
                LogicOp::Xor => format!("r{}", index),
            };

            translate.insert(self.output.clone(), new_name);
        }
        // Compute gate output
        let left_opt = inputs.get(&self.left);
        let right_opt = inputs.get(&self.right);
        match (left_opt, right_opt) {
            (Some(&left), Some(&right)) => {
                let out: bool = match self.op {
                    LogicOp::And => left && right,
                    LogicOp::Or => left || right,
                    LogicOp::Xor => left != right,
                };
                inputs.insert(self.output.clone(), out);
                true
            }
            _ => false,
        }
    }
}

impl FromStr for LogicGate {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_gate(s: &str) -> IResult<&str, LogicGate> {
            let (s, (left, op)) = separated_pair(alphanumeric1, char(' '), alphanumeric1)(s)?;
            let (s, right) = preceded(char(' '), alphanumeric1)(s)?;
            let (s, output) = preceded(tag(" -> "), alphanumeric1)(s)?;
            Ok((
                s,
                LogicGate {
                    left: left.to_string(),
                    right: right.to_string(),
                    output: output.to_string(),
                    op: op.parse().unwrap(),
                },
            ))
        }
        Ok(parse_gate(s).unwrap().1)
    }
}

struct CrossedWires {
    inputs: FxHashMap<String, bool>,
    gates: Vec<LogicGate>,
}
impl CrossedWires {
    fn numeric_value(start: &str, values: &FxHashMap<String, bool>) -> usize {
        let mut value: usize = 0;
        for (k, &v) in values {
            if k.starts_with(start) {
                let index: u32 = k[1..].parse().unwrap();
                value += 2usize.pow(index) * (v as usize);
            }
        }
        value
    }

    fn z_output(&self) -> usize {
        let mut values: FxHashMap<String, bool> = self.inputs.clone();
        let mut remaining_gates: VecDeque<LogicGate> = self.gates.iter().cloned().collect();
        let mut translate: FxHashMap<String, String> = FxHashMap::default();

        while let Some(gate) = remaining_gates.pop_front() {
            if !gate.emulate(&mut values, &mut translate) {
                remaining_gates.push_back(gate);
            }
        }

        Self::numeric_value("z", &values)
    }

    fn output_with_swap(
        &self,
        a_ref: &str,
        b_ref: &str,
        op: LogicOp,
        swap_map: &mut FxHashMap<String, String>,
    ) -> &str {
        let a: String = swap_map
            .get(a_ref)
            .unwrap_or(&a_ref.to_string())
            .to_string();
        let b: String = swap_map
            .get(b_ref)
            .unwrap_or(&b_ref.to_string())
            .to_string();
        for gate in &self.gates {
            let o: LogicOp = gate.op;
            if o == op {
                if (gate.left == a && gate.right == b) || (gate.left == b && gate.right == a) {
                    return &gate.output;
                } else if gate.left == a {
                    // b swap r
                    swap_map.insert(b.to_string(), gate.right.clone());
                    swap_map.insert(gate.right.clone(), b.to_string());
                    return &gate.output;
                } else if gate.left == b {
                    // a swap r
                    swap_map.insert(a.to_string(), gate.right.clone());
                    swap_map.insert(gate.right.clone(), a.to_string());
                    return &gate.output;
                } else if gate.right == a {
                    // b swap l
                    swap_map.insert(b.to_string(), gate.left.clone());
                    swap_map.insert(gate.left.clone(), b.to_string());
                    return &gate.output;
                } else if gate.right == b {
                    // a swap l
                    swap_map.insert(a.to_string(), gate.left.clone());
                    swap_map.insert(gate.left.clone(), a.to_string());
                    return &gate.output;
                }
            }
        }
        "This should not happen"
    }

    fn swapped(&self) -> String {
        let mut swap_map: FxHashMap<String, String> = FxHashMap::default();

        let _z00: &str = self.output_with_swap("x00", "y00", LogicOp::Xor, &mut swap_map);
        let mut carry: &str = self.output_with_swap("x00", "y00", LogicOp::And, &mut swap_map);
        //Loop for each input pair
        for i in 1..(self.inputs.len() / 2) {
            let x: String = format!("x{:0>2}", i);
            let y: String = format!("y{:0>2}", i);
            //There will always be XOR & AND for the inputs
            let xor: &str = self.output_with_swap(&x, &y, LogicOp::Xor, &mut swap_map);
            let and: &str = self.output_with_swap(&x, &y, LogicOp::And, &mut swap_map);
            //Check that we get the z output.
            let z: &str = self.output_with_swap(xor, carry, LogicOp::Xor, &mut swap_map);
            let target_z: String = format!("z{:0>2}", i);
            if z != target_z {
                swap_map.insert(z.to_string(), target_z.to_string());
                swap_map.insert(target_z.to_string(), z.to_string());
            }
            let other_and: &str = self.output_with_swap(xor, carry, LogicOp::And, &mut swap_map);
            carry = self.output_with_swap(and, other_and, LogicOp::Or, &mut swap_map);
        }
        //The last carry should be the last z output
        let keys: Vec<String> = swap_map.keys().sorted().cloned().collect();
        keys.join(",")
    }
}

impl FromStr for CrossedWires {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_input(s: &str) -> IResult<&str, (String, bool)> {
            let (s, (name, b)) = separated_pair(alphanumeric1, tag(": "), parse_usize)(s)?;
            Ok((s, (name.to_string(), b == 1)))
        }

        let blocks: Vec<&str> = split_blocks(s);
        let inputs: FxHashMap<String, bool> = blocks[0]
            .lines()
            .map(|l| parse_input(l).unwrap().1)
            .collect();
        let gates: Vec<LogicGate> = blocks[1].lines().map(|l| l.parse().unwrap()).collect();

        Ok(CrossedWires { inputs, gates })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_24.txt").expect("Cannot open input file");
    let wires: CrossedWires = s.parse().unwrap();
    println!("Part1: The system outputs the number {}", wires.z_output());
    println!(
        "Part2: The 8 wires involved in a swap are {}",
        wires.swapped()
    );
    // dvb,fhg,fsq,tnc,vcf,z10,z17,z39
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01
x02 OR y02 -> z02
";

    const EXAMPLE_2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
";

    #[test]
    fn part_1_test_1() {
        let wires: CrossedWires = EXAMPLE_1.parse().unwrap();
        assert_eq!(wires.z_output(), 4);
    }

    #[test]
    fn part_1_test_2() {
        let wires: CrossedWires = EXAMPLE_2.parse().unwrap();
        assert_eq!(wires.z_output(), 2024);
    }
}
