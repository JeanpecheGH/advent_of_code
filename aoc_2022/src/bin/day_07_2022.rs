use std::collections::HashMap;

fn main() {
    let s = util::file_as_string("aoc_2022/input/day_07.txt").expect("Cannot open input file");

    let mut dirs_map: HashMap<String, usize> = HashMap::new();
    let mut current_path: String = "".to_string();

    s.lines().for_each(|l| {
        let words: Vec<&str> = l.split_whitespace().collect();
        match (words[0], words[1]) {
            ("$", "cd") => current_path = move_path(&current_path, words[2], &mut dirs_map),
            (nbr, _) if nbr.parse::<usize>().is_ok() => {
                let n: usize = nbr.parse::<usize>().unwrap();
                add_to_path(&current_path, n, &mut dirs_map);
            }
            //We only care about the current directory or the files sizes
            _ => (),
        }
    });

    //Filter and sum dirs with size under 1000000
    let sum: usize = dirs_map.values().filter(|&&v| v <= 100000).sum();
    println!("Part1: The sum of data in folders containing 100000 bytes or less is {sum}");

    //Compute space that needs to be freed, and get the smallest dir that can be freed to attain that goal
    let used_space: usize = dirs_map.get("/").cloned().unwrap();
    let min_to_free: usize = used_space - 40000000;
    let to_free: usize = dirs_map
        .values()
        .filter(|&&v| v >= min_to_free)
        .min()
        .cloned()
        .unwrap();
    println!(
        "Part2: The smallest dir that can be free to get to 30000000 free space has a size of {to_free}"
    );
}

fn move_path(path: &str, dir: &str, dirs_map: &mut HashMap<String, usize>) -> String {
    if dir.eq("..") {
        let end = path.rfind('/').unwrap();
        let p = &path[..end];
        if p.is_empty() {
            '/'.to_string()
        } else {
            p.to_string()
        }
    } else {
        let mut new_path = path.to_string();
        if path.len() > 1 {
            new_path.push('/');
        }
        new_path.push_str(dir);
        dirs_map.insert(new_path.clone(), 0);
        new_path
    }
}

fn add_to_path(path: &str, size: usize, dirs_map: &mut HashMap<String, usize>) {
    dirs_map.iter_mut().for_each(|(p, s)| {
        if path.starts_with(p) {
            *s += size;
        }
    })
}
