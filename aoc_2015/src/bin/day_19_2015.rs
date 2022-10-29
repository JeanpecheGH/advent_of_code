use std::collections::HashSet;

fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_19.txt").expect("Cannot open input file");

    let mut v: Vec<String> = lines.map(|l| l.unwrap()).collect();

    let molecule: String = v.pop().unwrap();
    v.pop();

    let replace: Vec<(&str, &str)> = v
        .iter()
        .map(|s| {
            let words: Vec<&str> = s.split(' ').collect();
            (words[0], words[2])
        })
        .collect();

    let replace_no_e: Vec<(&str, &str)> = replace
        .clone()
        .into_iter()
        .filter(|&(k, _)| k != "e")
        .collect();

    let result_set = calibrate(&molecule, &replace_no_e);
    println!(
        "Part1: The calibration returned {} different molecules",
        result_set.len()
    );

    /*
    There are only to types of atom replacements :
      - A => BC
      - A => BRn(CY){0,2}DAr

    Which means that for every atom in the first case, it takes one step to remove it (+1).
    In the second case, this means the atoms Rn+D+Ar cost one step to remove, as we are already counting the step to remove D, we can count (+0) for Rn and Ar.
    The Y atom is also cancelled in this step, but it is also cancelling the C atom in front of it, essentially costing (-1) step to remove.
    Finally, as the final step is of the kind :
      - e => BC
    It only cost 1 step to remove the last 2 atoms, meaning we have to subtract 1 to the total previously computed
     */
    let atoms = split_atoms(&molecule);
    let steps: i32 = atoms
        .into_iter()
        .map(|a| match a {
            "Rn" | "Ar" => 0,
            "Y" => -1,
            _ => 1,
        })
        .sum::<i32>()
        - 1;
    println!(
        "Part2: The minimum number of steps to create the molecule is {}",
        steps
    );
}

fn calibrate(molecule: &str, replace: &[(&str, &str)]) -> HashSet<String> {
    let mut set: HashSet<String> = HashSet::new();
    replace.iter().for_each(|pair| {
        let partial_set: HashSet<String> = replace_one(molecule, pair);
        set.extend(partial_set.into_iter());
    });
    set
}

fn replace_one(molecule: &str, (key, value): &(&str, &str)) -> HashSet<String> {
    molecule
        .match_indices(key)
        .map(|(i, _)| {
            let mut s = molecule.to_string();
            s.replace_range(i..(i + key.len()), value);
            s
        })
        .collect()
}

fn split_atoms(molecule: &str) -> Vec<&str> {
    let mut v: Vec<&str> = molecule
        .chars()
        .collect::<Vec<char>>()
        .windows(2)
        .enumerate()
        .filter_map(|(i, pair)| match (pair[0], pair[1]) {
            ('A'..='Z', 'a'..='z') => Some(&molecule[i..i + 2]),
            ('A'..='Z', 'A'..='Z') => Some(&molecule[i..i + 1]),
            _ => None,
        })
        .collect();
    if let Some(c) = molecule.chars().last() {
        if c.is_uppercase() {
            let l = molecule.len();
            v.push(&molecule[l - 1..])
        }
    }
    v
}
