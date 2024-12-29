use std::{collections::VecDeque, str::FromStr};

use fxhash::FxHashMap;
use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, char},
    sequence::{preceded, separated_pair},
    IResult,
};
use util::{basic_parser::parse_usize, split_blocks};

#[derive(Copy, Clone, Debug)]
enum LogicOp {
    AND,
    OR,
    XOR,
}

impl FromStr for LogicOp {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(LogicOp::AND),
            "OR" => Ok(LogicOp::OR),
            "XOR" => Ok(LogicOp::XOR),
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
            let new_name = match self.op {
                LogicOp::AND => format!("a{}", index),
                LogicOp::OR => format!("o{}", index),
                LogicOp::XOR => format!("r{}", index),
            };

            translate.insert(self.output.clone(), new_name);
        }
        // COmpute gate output
        let left_opt = inputs.get(&self.left);
        let right_opt = inputs.get(&self.right);
        match (left_opt, right_opt) {
            (Some(&left), Some(&right)) => {
                let out: bool = match self.op {
                    LogicOp::AND => left && right,
                    LogicOp::OR => left || right,
                    LogicOp::XOR => left != right,
                };
                inputs.insert(self.output.clone(), out);
                true
            }
            _ => false,
        }
    }

    fn show(&self, i: &str, translate: &FxHashMap<String, String>) {
        let left = translate
            .get(&self.left)
            .map(|s| format!("{}({})", self.left, s))
            .unwrap_or(self.left.clone());
        let right = translate
            .get(&self.right)
            .map(|s| format!("{}({})", self.right, s))
            .unwrap_or(self.right.clone());
        let output = translate
            .get(&self.output)
            .map(|s| format!("{}({})", self.output, s))
            .unwrap_or(self.output.clone());
        if left.contains(i) || right.contains(i) || output.contains(i) {
            println!("{} {:?} {} -> {}", left, self.op, right, output);
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
        let mut swap: FxHashMap<String, String> = FxHashMap::default();
        swap.insert("vcf".to_string(), "z10".to_string());
        swap.insert("z10".to_string(), "vcf".to_string());

        swap.insert("fsq".to_string(), "dvb".to_string());
        swap.insert("dvb".to_string(), "fsq".to_string());

        swap.insert("tnc".to_string(), "z39".to_string());
        swap.insert("z39".to_string(), "tnc".to_string());

        swap.insert("fhg".to_string(), "z17".to_string());
        swap.insert("z17".to_string(), "fhg".to_string());
        let mut remaining_gates: VecDeque<LogicGate> = self
            .gates
            .iter()
            .cloned()
            .map(|gate| {
                let output = swap.get(&gate.output).unwrap_or(&gate.output).clone();
                LogicGate {
                    left: gate.left,
                    right: gate.right,
                    output,
                    op: gate.op,
                }
            })
            .collect();
        let mut translate: FxHashMap<String, String> = FxHashMap::default();

        while let Some(gate) = remaining_gates.pop_front() {
            if !gate.emulate(&mut values, &mut translate) {
                remaining_gates.push_back(gate);
            }
        }

        for i in 0..=45 {
            let i_str: String = format!("{:0>2}", i);
            println!("Cycle: {}", i_str);
            let new_gates: Vec<LogicGate> = self
                .gates
                .iter()
                .cloned()
                .map(|gate| {
                    let output = swap.get(&gate.output).unwrap_or(&gate.output).clone();
                    LogicGate {
                        left: gate.left,
                        right: gate.right,
                        output,
                        op: gate.op,
                    }
                })
                .collect();
            for g in &new_gates {
                g.show(&i_str, &translate);
            }
        }

        Self::numeric_value("z", &values)
    }

    fn expected(&self) -> String {
        let x: usize = Self::numeric_value("x", &self.inputs);
        let y: usize = Self::numeric_value("y", &self.inputs);
        println!("x: {}, y: {} -> z: {}", x, y, x + y);
        format!("{}", x + y)
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
    println!("Part1: {}", wires.z_output());
    println!("Part2: {}", wires.expected());
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
    fn part_2_test_2() {
        let wires: CrossedWires = EXAMPLE_2.parse().unwrap();
        assert_eq!(wires.z_output(), 2024);
    }
}
