use regex::Regex;
use std::collections::HashMap;
use std::str::FromStr;
use util;

#[derive(Debug, Copy, Clone)]
enum WireType {
    Val,
    Id,
}

#[derive(Debug, Clone)]
struct Wire {
    w_type: WireType,
    val: Option<u16>,
    id: Option<String>,
}

impl FromStr for Wire {
    type Err = ();

    fn from_str(input: &str) -> Result<Wire, Self::Err> {
        if let Ok(n) = input.parse::<u16>() {
            Ok(Wire {
                w_type: WireType::Val,
                val: Some(n),
                id: None,
            })
        } else if !input.is_empty() {
            Ok(Wire {
                w_type: WireType::Id,
                val: None,
                id: Some(input.to_string()),
            })
        } else {
            Err(())
        }
    }
}

impl Wire {
    fn to_value(self, map: &HashMap<String, u16>) -> Option<u16> {
        match self.w_type {
            WireType::Val => self.val,
            WireType::Id => self.id.map(|id| map.get(&id)).flatten().cloned(),
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum OperationType {
    Equal,
    And,
    Or,
    Lshift,
    Rshift,
    Not,
}

impl FromStr for OperationType {
    type Err = ();

    fn from_str(input: &str) -> Result<OperationType, Self::Err> {
        match input {
            "" => Ok(OperationType::Equal),
            "AND" => Ok(OperationType::And),
            "OR" => Ok(OperationType::Or),
            "LSHIFT" => Ok(OperationType::Lshift),
            "RSHIFT" => Ok(OperationType::Rshift),
            "NOT" => Ok(OperationType::Not),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
struct Operation {
    op: OperationType,
    left: Option<Wire>,
    right: Option<Wire>,
    target: String,
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(input: &str) -> Result<Operation, Self::Err> {
        let re =
            Regex::new(r"^(?:([0-9a-z]*)[ ])?(AND|OR|LSHIFT|RSHIFT|NOT|)[ ]?([0-9a-z]+) -> (\w*)$")
                .unwrap();
        let res = if let Some(cap) = re.captures_iter(input).next() {
            let op: OperationType = OperationType::from_str(&cap[2])?;
            let target = cap[4].to_string();
            match op {
                OperationType::Equal | OperationType::Not => Ok(Operation {
                    op,
                    left: None,
                    right: Some(Wire::from_str(&cap[3])?),
                    target,
                }),
                OperationType::And
                | OperationType::Or
                | OperationType::Lshift
                | OperationType::Rshift => Ok(Operation {
                    op,
                    left: Some(Wire::from_str(&cap[1])?),
                    right: Some(Wire::from_str(&cap[3])?),
                    target,
                }),
            }
        } else {
            Err(())
        };
        res
    }
}

impl Operation {
    fn execute(&self, map: &mut HashMap<String, u16>) {
        if !map.contains_key(self.target.as_str()) {
            match self.op {
                OperationType::Equal => {
                    if let Some(v) = self.right.clone().unwrap().to_value(map) {
                        map.insert(self.target.clone(), v);
                    }
                }
                OperationType::And => {
                    if let Some(v1) = self.left.clone().unwrap().to_value(map) {
                        if let Some(v2) = self.right.clone().unwrap().to_value(map) {
                            map.insert(self.target.clone(), v1 & v2);
                        }
                    }
                }
                OperationType::Or => {
                    if let Some(v1) = self.left.clone().unwrap().to_value(map) {
                        if let Some(v2) = self.right.clone().unwrap().to_value(map) {
                            map.insert(self.target.clone(), v1 | v2);
                        }
                    }
                }
                OperationType::Lshift => {
                    if let Some(v1) = self.left.clone().unwrap().to_value(map) {
                        if let Some(v2) = self.right.clone().unwrap().to_value(map) {
                            map.insert(self.target.clone(), v1 << v2);
                        }
                    }
                }
                OperationType::Rshift => {
                    if let Some(v1) = self.left.clone().unwrap().to_value(map) {
                        if let Some(v2) = self.right.clone().unwrap().to_value(map) {
                            map.insert(self.target.clone(), v1 >> v2);
                        }
                    }
                }
                OperationType::Not => {
                    if let Some(v) = self.right.clone().unwrap().to_value(map) {
                        map.insert(self.target.clone(), !v);
                    }
                }
            }
        }
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_07.txt").expect("Cannot open input file");

    let ops: Vec<Operation> = lines
        .filter_map(|l| Operation::from_str(l.unwrap().as_str()).ok())
        .collect();

    let mut map: HashMap<String, u16> = HashMap::new();

    while !map.contains_key("a") {
        ops.iter().for_each(|op| op.execute(&mut map));
    }

    let first_a = map.get("a").cloned().unwrap();

    println!("Part1: The wire 'a' contains {}", first_a);

    map.clear();
    map.insert("b".to_string(), first_a);

    while !map.contains_key("a") {
        ops.iter().for_each(|op| op.execute(&mut map));
    }

    let second_a = map.get("a").unwrap();
    println!("Part2: The wire 'a' now contains {}", second_a);
}
