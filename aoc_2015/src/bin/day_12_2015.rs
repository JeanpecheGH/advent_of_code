use serde_json::Value;

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_12.txt").expect("Cannot open input file");

    let json: Value = serde_json::from_str(&s).unwrap();

    let res_1 = sum_json(&json);
    println!("Part1: The sum of all number in the JSON is {res_1}");

    let res_2 = sum_json_no_red(&json);
    println!(
        "Part2: The sum of all number in the JSON with no red Object is {}",
        res_2.unwrap()
    );
}

fn sum_json(data: &Value) -> i64 {
    match data {
        Value::Null | Value::Bool(_) => 0,
        Value::Number(a) => a.as_i64().unwrap(),
        Value::String(_) => 0,
        Value::Array(v) => v.iter().map(sum_json).sum(),
        Value::Object(m) => m.values().map(sum_json).sum(),
    }
}

fn sum_json_no_red(data: &Value) -> Option<i64> {
    match data {
        Value::Null | Value::Bool(_) => Some(0),
        Value::Number(a) => Some(a.as_i64().unwrap()),
        Value::String(r) if r.eq("red") => None,
        Value::String(_) => Some(0),
        Value::Array(v) => Some(v.iter().flat_map(sum_json_no_red).sum::<i64>()),
        Value::Object(m) => {
            let vals: Vec<Option<i64>> = m.values().map(sum_json_no_red).collect();
            if vals.iter().all(|opt| opt.is_some()) {
                Some(vals.iter().flatten().sum())
            } else {
                Some(0)
            }
        }
    }
}
