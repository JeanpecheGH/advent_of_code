fn main() {
    let lines = util::file_as_lines("aoc_2015/input/day_24.txt").expect("Cannot open input file");

    let packages: Vec<u64> = lines.map(|l| l.unwrap().parse().unwrap()).collect();

    let weight_sum: u64 = packages.iter().sum();

    let res1 = fill_packages(packages.clone(), weight_sum / 3, 6, Vec::new());
    let qes1: Option<u64> = res1.iter().map(|v| v.iter().product()).min();
    println!(
        "Part1: The lowest Quantum Entanglement whe dividing in 3 groups is {:?}",
        qes1.unwrap()
    );

    let res2 = fill_packages(packages, weight_sum / 4, 4, Vec::new());
    let qes2: Option<u64> = res2.iter().map(|v| v.iter().product()).min();
    println!(
        "Part2: The lowest Quantum Entanglement whe dividing in 4 groups is {:?}",
        qes2.unwrap()
    );
}

fn fill_packages(
    mut packages: Vec<u64>,
    target: u64,
    max_packages: usize,
    current_set: Vec<u64>,
) -> Vec<Vec<u64>> {
    if current_set.len() > max_packages {
        return Vec::new();
    }
    match (packages.pop(), target) {
        (Some(p), t) if t >= p => {
            let mut new_set = current_set.clone();
            new_set.push(p);
            let mut resp = fill_packages(packages.clone(), t - p, max_packages, new_set);
            resp.extend(fill_packages(packages, t, max_packages, current_set));
            resp
        }
        (Some(_), t) => fill_packages(packages, t, max_packages, current_set),
        (None, 0) => vec![current_set],
        _ => Vec::new(),
    }
}
