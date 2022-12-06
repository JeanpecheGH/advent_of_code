use std::collections::HashSet;

fn main() {
    let s = util::file_as_string("aoc_2020/input/day_06.txt").expect("Cannot open input file");

    let lines: Vec<&str> = s.lines().collect();
    let groups: Vec<&[&str]> = lines.split(|l| l.is_empty()).collect();

    let nb_answer: usize = nb_answers(&groups, set_union);
    println!(
        "Part1: The sum of all questions answered at least once in each group is {}",
        nb_answer
    );

    let nb_answer_intersect: usize = nb_answers(&groups, set_intersection);
    println!(
        "Part2: The sum of all questions answered by each passenger of each group is {}",
        nb_answer_intersect
    );
}
fn set_union(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.union(&b).cloned().collect()
}

fn set_intersection(a: HashSet<char>, b: HashSet<char>) -> HashSet<char> {
    a.intersection(&b).cloned().collect()
}

fn nb_answers(groups: &[&[&str]], f: fn(HashSet<char>, HashSet<char>) -> HashSet<char>) -> usize {
    groups
        .iter()
        .map(|group| {
            group
                .iter()
                .map(|passenger| passenger.chars().collect::<HashSet<char>>())
                .reduce(f)
                .unwrap()
                .len()
        })
        .sum()
}
