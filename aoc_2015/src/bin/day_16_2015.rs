use std::collections::HashMap;

#[derive(Clone, Debug)]
struct AuntSue {
    id: u32,
    attributes: HashMap<String, u32>,
}

impl AuntSue {
    fn is_valid(&self, other: &AuntSue) -> bool {
        self.attributes
            .iter()
            .all(|(attr, &val)| other.attributes[attr] == val)
    }
    fn is_valid_2(&self, other: &AuntSue) -> bool {
        self.attributes
            .iter()
            .all(|(attr, &val)| match attr.as_str() {
                "cats" | "trees" => other.attributes[attr] < val,
                "pomeranians" | "goldfish" => other.attributes[attr] > val,
                _ => other.attributes[attr] == val,
            })
    }
}

fn main() {
    let s = util::file_as_string("aoc_2015/input/day_16.txt").expect("Cannot open input file");

    let aunts: Vec<AuntSue> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();

            let id: u32 = words[1].strip_suffix(':').unwrap().parse::<u32>().unwrap();
            let att_1: &str = words[2].strip_suffix(':').unwrap();
            let val_1: u32 = words[3].strip_suffix(',').unwrap().parse::<u32>().unwrap();
            let att_2: &str = words[4].strip_suffix(':').unwrap();
            let val_2: u32 = words[5].strip_suffix(',').unwrap().parse::<u32>().unwrap();
            let att_3: &str = words[6].strip_suffix(':').unwrap();
            let val_3: u32 = words[7].parse::<u32>().unwrap();

            let mut attributes = HashMap::new();
            attributes.insert(att_1.to_string(), val_1);
            attributes.insert(att_2.to_string(), val_2);
            attributes.insert(att_3.to_string(), val_3);

            AuntSue { id, attributes }
        })
        .collect();
    let mut target_sue_attributes = HashMap::new();
    target_sue_attributes.insert("children".to_string(), 3);
    target_sue_attributes.insert("cats".to_string(), 7);
    target_sue_attributes.insert("samoyeds".to_string(), 2);
    target_sue_attributes.insert("pomeranians".to_string(), 3);
    target_sue_attributes.insert("akitas".to_string(), 0);
    target_sue_attributes.insert("vizslas".to_string(), 0);
    target_sue_attributes.insert("goldfish".to_string(), 5);
    target_sue_attributes.insert("trees".to_string(), 3);
    target_sue_attributes.insert("cars".to_string(), 2);
    target_sue_attributes.insert("perfumes".to_string(), 1);
    let target_sue = AuntSue {
        id: 0,
        attributes: target_sue_attributes,
    };

    let mut valid_aunts: Vec<AuntSue> = aunts
        .iter()
        .filter(|&aunt| aunt.is_valid(&target_sue))
        .cloned()
        .collect();
    println!(
        "Part1: The good Sue is number {:?}",
        valid_aunts.pop().unwrap().id
    );

    let mut valid_aunts_2: Vec<AuntSue> = aunts
        .iter()
        .filter(|&aunt| aunt.is_valid_2(&target_sue))
        .cloned()
        .collect();
    println!(
        "Part2: The good Sue is finally number {:?}",
        valid_aunts_2.pop().unwrap().id
    );
}
