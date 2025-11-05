use std::str::FromStr;

use fxhash::FxHashMap;
use util::coord::PosI;

type PathDict = FxHashMap<(char, char), KeypadPath>;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum KeypadMove {
    Up,
    Down,
    Left,
    Right,
    Push,
}

impl KeypadMove {
    fn as_char(&self) -> char {
        match self {
            KeypadMove::Up => '^',
            KeypadMove::Down => 'v',
            KeypadMove::Left => '<',
            KeypadMove::Right => '>',
            KeypadMove::Push => 'A',
        }
    }

    fn from_char(c: char) -> Self {
        match c {
            '^' => KeypadMove::Up,
            'v' => KeypadMove::Down,
            '<' => KeypadMove::Left,
            '>' => KeypadMove::Right,
            'A' => KeypadMove::Push,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct KeypadPath {
    moves: Vec<KeypadMove>,
}

impl KeypadPath {
    fn from_moves(moves: &[KeypadMove]) -> Self {
        KeypadPath {
            moves: moves.to_vec(),
        }
    }

    fn len(&self) -> usize {
        self.moves.len()
    }

    fn extend(&mut self, other: KeypadPath) {
        self.moves.extend(other.moves);
    }

    fn from_slice(s: &str) -> Self {
        let moves: Vec<KeypadMove> = s.chars().map(KeypadMove::from_char).collect();
        KeypadPath { moves }
    }

    fn from_diff(diff: PosI) -> Vec<KeypadPath> {
        // One or two optimal paths are possible.
        // Either up/down then left/right or left/right then up/down.
        let horizontal: Vec<KeypadMove> = if diff.0 > 0 {
            vec![KeypadMove::Right; diff.0 as usize]
        } else {
            vec![KeypadMove::Left; diff.0.unsigned_abs()]
        };

        let mut vertical: Vec<KeypadMove> = if diff.1 > 0 {
            vec![KeypadMove::Down; diff.1 as usize]
        } else {
            vec![KeypadMove::Up; diff.1.unsigned_abs()]
        };

        let mut paths: Vec<KeypadPath> = Vec::new();
        let mut horizontal_first = horizontal.clone();
        horizontal_first.extend_from_slice(&vertical);
        horizontal_first.push(KeypadMove::Push);
        paths.push(KeypadPath::from_moves(&horizontal_first));

        if !horizontal.is_empty() && !vertical.is_empty() {
            vertical.extend(horizontal);
            vertical.push(KeypadMove::Push);
            paths.push(KeypadPath::from_moves(&vertical));
        }
        paths
    }
}

struct Keypad {
    buttons: FxHashMap<PosI, char>,
}

impl Keypad {
    fn new_directional() -> Self {
        let mut buttons: FxHashMap<PosI, char> = FxHashMap::default();
        buttons.insert(PosI(1, 0), '^');
        buttons.insert(PosI(2, 0), 'A');
        buttons.insert(PosI(0, 1), '<');
        buttons.insert(PosI(1, 1), 'v');
        buttons.insert(PosI(2, 1), '>');
        Keypad { buttons }
    }

    fn new_numerical() -> Self {
        let mut buttons: FxHashMap<PosI, char> = FxHashMap::default();
        buttons.insert(PosI(0, 0), '7');
        buttons.insert(PosI(1, 0), '8');
        buttons.insert(PosI(2, 0), '9');
        buttons.insert(PosI(0, 1), '4');
        buttons.insert(PosI(1, 1), '5');
        buttons.insert(PosI(2, 1), '6');
        buttons.insert(PosI(0, 2), '1');
        buttons.insert(PosI(1, 2), '2');
        buttons.insert(PosI(2, 2), '3');
        buttons.insert(PosI(1, 3), '0');
        buttons.insert(PosI(2, 3), 'A');
        Keypad { buttons }
    }

    fn paths(&self, from: char, to: char) -> Vec<KeypadPath> {
        let from_pos: PosI = *self.buttons.iter().find(|&(_, &v)| v == from).unwrap().0;
        let to_pos: PosI = *self.buttons.iter().find(|&(_, &v)| v == to).unwrap().0;

        let diff: PosI = to_pos.sub(from_pos);
        // Remove the path if it goes over a gap
        let paths: Vec<KeypadPath> = KeypadPath::from_diff(diff)
            .into_iter()
            .filter(|p| {
                let mut pos: PosI = from_pos;
                for m in &p.moves {
                    match m {
                        KeypadMove::Up => pos = pos.add(PosI(0, -1)),
                        KeypadMove::Down => pos = pos.add(PosI(0, 1)),
                        KeypadMove::Left => pos = pos.add(PosI(-1, 0)),
                        KeypadMove::Right => pos = pos.add(PosI(1, 0)),
                        KeypadMove::Push => (),
                    }
                    if !self.buttons.contains_key(&pos) {
                        return false;
                    }
                }
                true
            })
            .collect();
        paths
    }
}

struct KeypadsSystem {
    codes: Vec<String>,
}
impl KeypadsSystem {
    fn solve(&self, times: usize) -> usize {
        let mut dict: PathDict = Self::build_path_dict();
        Self::add_num_paths(&mut dict);

        self.codes
            .iter()
            .map(|c| {
                let numeric_part: usize = c[0..3].parse().unwrap();
                let shortest_sequence: usize = self.shortest_sequence(c, times, &dict);
                numeric_part * shortest_sequence
            })
            .sum()
    }

    fn shortest_sequence(&self, code: &str, times: usize, cache: &PathDict) -> usize {
        let mut modified_code: Vec<char> = vec!['A'];
        modified_code.extend(code.chars());

        let mut result_cache: FxHashMap<(char, char), usize> = FxHashMap::default();

        // Init the pairs from the code
        for pair in modified_code.windows(2) {
            *result_cache.entry((pair[0], pair[1])).or_insert(0) += 1;
        }

        // Get the shortest path for each pair of buttons
        for _ in 0..=times {
            let mut new_cache: FxHashMap<(char, char), usize> = FxHashMap::default();
            for (&(from, to), &count) in result_cache.iter() {
                let path: &KeypadPath = cache.get(&(from, to)).unwrap();
                let mut x: Vec<KeypadMove> = vec![KeypadMove::Push];
                x.extend_from_slice(&path.moves);

                for pair in x.windows(2) {
                    *new_cache
                        .entry((pair[0].as_char(), pair[1].as_char()))
                        .or_insert(0) += count;
                }
            }
            result_cache = new_cache;
        }
        result_cache.values().sum()
    }

    fn add_num_paths(cache: &mut PathDict) {
        let keypad: Keypad = Keypad::new_numerical();
        let nums: Vec<char> = keypad.buttons.values().copied().collect();
        for &from in &nums {
            for &to in &nums {
                let paths: Vec<KeypadPath> = keypad.paths(from, to);
                let mut best_map: Vec<(KeypadPath, KeypadPath)> =
                    paths.into_iter().map(|p| (p.clone(), p)).collect();
                while best_map.len() > 1 {
                    let min_size: usize = best_map.iter().map(|(_, v)| v.len()).min().unwrap();
                    best_map.retain(|(_, v)| v.len() == min_size);
                    best_map.iter_mut().for_each(|(_, path)| {
                        *path = Self::next_step(path, cache);
                    });
                }
                cache.insert((from, to), best_map.pop().unwrap().0);
            }
        }
    }

    fn build_path_dict() -> PathDict {
        let keypad: Keypad = Keypad::new_directional();
        let dirs: Vec<char> = keypad.buttons.values().copied().collect();
        //Build all possible dict (a path between two keys has 1 or 2 path only)
        let mut dicts: Vec<PathDict> = vec![FxHashMap::default()];
        for &from in &dirs {
            for &to in &dirs {
                let mut new_dicts: Vec<PathDict> = Vec::new();

                let paths: Vec<KeypadPath> = keypad.paths(from, to);
                for path in paths {
                    for dict in &dicts {
                        let mut n_dict: PathDict = dict.clone();
                        n_dict.insert((from, to), path.clone());
                        new_dicts.push(n_dict);
                    }
                }
                dicts = new_dicts;
            }
        }

        //Use an arbitrary path to find the best dict
        let path: KeypadPath = KeypadPath::from_slice("<^>vA");
        let mut best_map: Vec<(PathDict, KeypadPath)> =
            dicts.into_iter().map(|d| (d, path.clone())).collect();
        while best_map.len() > 1 {
            let min_size: usize = best_map.iter().map(|(_, v)| v.len()).min().unwrap();
            best_map.retain(|(_, v)| v.len() == min_size);
            best_map.iter_mut().for_each(|(dict, path)| {
                *path = Self::next_step(path, dict);
            });
        }

        best_map.pop().unwrap().0
    }

    fn next_step(path: &KeypadPath, dict: &PathDict) -> KeypadPath {
        let mut from: char = 'A';

        let mut new_path: KeypadPath = KeypadPath { moves: Vec::new() };
        for m in &path.moves {
            let to: char = m.as_char();
            let new_section: KeypadPath = dict.get(&(from, to)).cloned().unwrap();
            new_path.extend(new_section);
            from = to;
        }
        new_path
    }
}

impl FromStr for KeypadsSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let codes: Vec<String> = s.lines().map(|l| l.to_string()).collect();
        Ok(KeypadsSystem { codes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_21.txt").expect("Cannot open input file");
    let keypads: KeypadsSystem = s.parse().unwrap();
    println!(
        "Part1: The sum of the complexities of the codes is {}",
        keypads.solve(2)
    );
    println!(
        "Part2: With 25 intermediates robots, the sum is now {}",
        keypads.solve(25)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "029A
980A
179A
456A
379A
";

    #[test]
    fn part_1() {
        let keypads: KeypadsSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!(keypads.solve(2), 126384);
    }
}
