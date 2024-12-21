use std::str::FromStr;

use fxhash::FxHashMap;
use util::coord::PosI;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
enum KeypadMove {
    Up,
    Down,
    Left,
    Right,
    Push,
}

impl KeypadMove {
    fn to_char(&self) -> char {
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
    fn len(&self) -> usize {
        self.moves.len()
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

        let vertical: Vec<KeypadMove> = if diff.1 > 0 {
            vec![KeypadMove::Down; diff.1 as usize]
        } else {
            vec![KeypadMove::Up; diff.1.unsigned_abs()]
        };

        let mut paths: Vec<KeypadPath> = Vec::new();
        if !horizontal.is_empty() {
            let mut horizontal_first: Vec<KeypadMove> = horizontal.clone();
            horizontal_first.extend_from_slice(&vertical);
            horizontal_first.push(KeypadMove::Push);
            paths.push(KeypadPath {
                moves: horizontal_first,
            });
        }

        if !vertical.is_empty() {
            let mut vertical_first: Vec<KeypadMove> = vertical.clone();
            vertical_first.extend_from_slice(&horizontal);
            vertical_first.push(KeypadMove::Push);
            paths.push(KeypadPath {
                moves: vertical_first,
            });
        }
        if vertical.is_empty() && horizontal.is_empty() {
            paths.push(KeypadPath {
                moves: vec![KeypadMove::Push],
            });
        }
        paths
    }
}

struct DirectionalKeypad {
    buttons: FxHashMap<PosI, char>,
}

impl DirectionalKeypad {
    fn new() -> Self {
        let mut buttons: FxHashMap<PosI, char> = FxHashMap::default();
        buttons.insert(PosI(1, 0), '^');
        buttons.insert(PosI(2, 0), 'A');
        buttons.insert(PosI(0, 1), '<');
        buttons.insert(PosI(1, 1), 'v');
        buttons.insert(PosI(2, 1), '>');
        DirectionalKeypad { buttons }
    }

    fn paths(
        &self,
        from: char,
        to: char,
        cache: &mut FxHashMap<(char, char), Vec<KeypadPath>>,
    ) -> Vec<KeypadPath> {
        if let Some(path) = cache.get(&(from, to)) {
            return path.clone();
        }
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

        cache.insert((from, to), paths.clone());
        paths
    }
}

struct NumericKeypad {
    buttons: FxHashMap<PosI, char>,
}

impl NumericKeypad {
    fn new() -> Self {
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
        NumericKeypad { buttons }
    }

    fn paths(
        &self,
        from: char,
        to: char,
        cache: &mut FxHashMap<(char, char), Vec<KeypadPath>>,
    ) -> Vec<KeypadPath> {
        if let Some(path) = cache.get(&(from, to)) {
            return path.clone();
        }
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

        cache.insert((from, to), paths.clone());
        paths
    }
}

struct KeypadsSystem {
    codes: Vec<String>,
}
impl KeypadsSystem {
    fn solve(&self, times: usize) -> usize {
        let mut cache: FxHashMap<(char, char), Vec<KeypadPath>> = FxHashMap::default();
        let mut num_cache: FxHashMap<(char, char), KeypadPath> = Self::best_pairs_num(&mut cache);
        let mut dir_cache: FxHashMap<(char, char), KeypadPath> = Self::best_pairs_dir(&mut cache);
        num_cache.insert(('1', '9'), KeypadPath::from_slice("^^>>A"));
        num_cache.insert(('1', '8'), KeypadPath::from_slice("^^>A"));
        num_cache.insert(('1', '6'), KeypadPath::from_slice("^>>A"));
        num_cache.insert(('1', '5'), KeypadPath::from_slice("^>A"));
        num_cache.insert(('2', '9'), KeypadPath::from_slice("^^>A"));
        num_cache.insert(('2', '6'), KeypadPath::from_slice("^>A"));
        num_cache.insert(('0', '9'), KeypadPath::from_slice("^^^>A"));
        num_cache.insert(('0', '6'), KeypadPath::from_slice("^^>A"));
        num_cache.insert(('0', '3'), KeypadPath::from_slice("^>A"));
        num_cache.insert(('4', '9'), KeypadPath::from_slice("^>>A"));
        num_cache.insert(('4', '8'), KeypadPath::from_slice("^>A"));
        num_cache.insert(('5', '9'), KeypadPath::from_slice("^>A"));
        dir_cache.insert(('v', 'A'), KeypadPath::from_slice("^>A"));
        // for (&(from, to), v) in &num_cache {
        //     if v.len() > 2 {
        //         println!("{} -> {} : {:?}", from, to, v);
        //     }
        // }

        self.codes
            .iter()
            .map(|c| {
                let numeric_part: usize = c[0..3].parse().unwrap();
                //let shortest_sequence: usize = self.shortest_sequence_n(c, times);
                let shortest_sequence: usize =
                    self.shortest_sequence(c, times, &num_cache, &dir_cache);
                numeric_part * shortest_sequence
            })
            .sum()
    }

    fn shortest_sequence(
        &self,
        code: &str,
        times: usize,
        num_cache: &FxHashMap<(char, char), KeypadPath>,
        dir_cache: &FxHashMap<(char, char), KeypadPath>,
    ) -> usize {
        let mut modified_code: Vec<char> = vec!['A'];
        modified_code.extend(code.chars());

        let mut result_cache: FxHashMap<(char, char), usize> = FxHashMap::default();

        // Init the pairs from the code
        for pair in modified_code.windows(2) {
            *result_cache.entry((pair[0], pair[1])).or_insert(0) += 1;
        }
        //println!("{:?}", result_cache);

        // Get the shortest numercial path for each pair
        let mut new_cache: FxHashMap<(char, char), usize> = FxHashMap::default();
        for (&(from, to), &count) in result_cache.iter() {
            let path: &KeypadPath = num_cache.get(&(from, to)).unwrap();
            let mut x: Vec<KeypadMove> = vec![KeypadMove::Push];
            x.extend_from_slice(&path.moves);

            for pair in x.windows(2) {
                *new_cache
                    .entry((pair[0].to_char(), pair[1].to_char()))
                    .or_insert(0) += count;
            }
        }
        result_cache = new_cache;
        //println!("{:?}", result_cache);

        for _ in 0..times {
            let mut new_cache: FxHashMap<(char, char), usize> = FxHashMap::default();
            for (&(from, to), &count) in result_cache.iter() {
                let path: &KeypadPath = dir_cache.get(&(from, to)).unwrap();
                let mut x: Vec<KeypadMove> = vec![KeypadMove::Push];
                x.extend_from_slice(&path.moves);

                //println!("{:?}, count {}", path, count);

                for pair in x.windows(2) {
                    *new_cache
                        .entry((pair[0].to_char(), pair[1].to_char()))
                        .or_insert(0) += count;
                }
            }
            result_cache = new_cache;
            //println!("{:?}", result_cache);
        }

        let shortest: usize = result_cache.values().sum();

        //println!("{} -> {}", code, shortest);
        shortest
    }

    fn best_pairs_num(
        cache: &mut FxHashMap<(char, char), Vec<KeypadPath>>,
    ) -> FxHashMap<(char, char), KeypadPath> {
        let mut best_cache: FxHashMap<(char, char), KeypadPath> = FxHashMap::default();
        let numeric_keypad: NumericKeypad = NumericKeypad::new();
        let directional_keypad: DirectionalKeypad = DirectionalKeypad::new();
        let nums: Vec<char> = vec!['0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'A'];
        for &from in &nums {
            for &to in &nums {
                let paths: Vec<KeypadPath> = numeric_keypad.paths(from, to, cache);
                let mut min_len: usize = usize::MAX;
                //println!("{} -> {}", from, to);
                for p in paths {
                    let mut previous_paths: Vec<KeypadPath> = vec![p.clone()];
                    for _ in 0..3 {
                        previous_paths =
                            Self::next_step(previous_paths, &directional_keypad, cache);
                    }
                    let shortest: usize = previous_paths.iter().map(|p| p.len()).min().unwrap();
                    if shortest < min_len {
                        min_len = shortest;
                        //println!("{:?}", p);
                        best_cache.insert((from, to), p);
                    }
                }
            }
        }
        best_cache
    }

    fn best_pairs_dir(
        cache: &mut FxHashMap<(char, char), Vec<KeypadPath>>,
    ) -> FxHashMap<(char, char), KeypadPath> {
        let mut best_cache: FxHashMap<(char, char), KeypadPath> = FxHashMap::default();
        let directional_keypad: DirectionalKeypad = DirectionalKeypad::new();
        let nums: Vec<char> = vec!['<', '>', '^', 'v', 'A'];
        for &from in &nums {
            for &to in &nums {
                let paths: Vec<KeypadPath> = directional_keypad.paths(from, to, cache);
                let mut min_len: usize = usize::MAX;
                //println!("{} -> {}", from, to);
                for p in paths {
                    let mut previous_paths: Vec<KeypadPath> = vec![p.clone()];
                    for _ in 0..3 {
                        previous_paths =
                            Self::next_step(previous_paths, &directional_keypad, cache);
                    }
                    let shortest: usize = previous_paths.iter().map(|p| p.len()).min().unwrap();
                    if shortest < min_len {
                        min_len = shortest;
                        //println!("{:?}", p);
                        best_cache.insert((from, to), p);
                    }
                }
            }
        }
        best_cache
    }

    fn next_step(
        previous_paths: Vec<KeypadPath>,
        directional_keypad: &DirectionalKeypad,
        cache: &mut FxHashMap<(char, char), Vec<KeypadPath>>,
    ) -> Vec<KeypadPath> {
        previous_paths
            .into_iter()
            .flat_map(|p| {
                let mut from: char = 'A';
                let mut inter_paths: Vec<KeypadPath> = vec![KeypadPath { moves: Vec::new() }];
                for m in p.moves {
                    let to: char = match m {
                        KeypadMove::Up => '^',
                        KeypadMove::Down => 'v',
                        KeypadMove::Left => '<',
                        KeypadMove::Right => '>',
                        KeypadMove::Push => 'A',
                    };
                    let paths: Vec<KeypadPath> = directional_keypad.paths(from, to, cache);
                    inter_paths = inter_paths
                        .into_iter()
                        .flat_map(|p| {
                            paths
                                .iter()
                                .map(|p2| {
                                    let mut p3: KeypadPath = p.clone();
                                    p3.moves.extend_from_slice(&p2.moves);
                                    p3
                                })
                                .collect::<Vec<KeypadPath>>()
                        })
                        .collect();
                    from = to;
                }
                inter_paths
            })
            .collect()
    }

    // fn shortest_sequence_n(&self, code: &str, times: usize) -> usize {
    //     let mut cache: FxHashMap<(char, char), Vec<KeypadPath>> = FxHashMap::default();
    //     let numeric_keypad: NumericKeypad = NumericKeypad::new();
    //     let mut from: char = 'A';
    //     let mut first_paths: Vec<KeypadPath> = vec![KeypadPath { moves: Vec::new() }];
    //     for c in code.chars() {
    //         let paths: Vec<KeypadPath> = numeric_keypad.paths(from, c, &mut cache);
    //         first_paths = first_paths
    //             .into_iter()
    //             .flat_map(|p| {
    //                 paths
    //                     .iter()
    //                     .map(|p2| {
    //                         let mut p3: KeypadPath = p.clone();
    //                         p3.moves.extend_from_slice(&p2.moves);
    //                         p3
    //                     })
    //                     .collect::<Vec<KeypadPath>>()
    //             })
    //             .collect();
    //         from = c;
    //     }

    //     let directional_keypad: DirectionalKeypad = DirectionalKeypad::new();
    //     let mut previous_paths: Vec<KeypadPath> = first_paths;
    //     for _ in 0..times {
    //         let new_paths: Vec<KeypadPath> =
    //             Self::next_step(previous_paths, &directional_keypad, &mut cache);
    //         let with_len: Vec<(usize, KeypadPath)> = new_paths
    //             .into_iter()
    //             .map(|p| (p.len(), p.clone()))
    //             .collect();
    //         let shortest: usize = with_len.iter().map(|(l, _)| l).min().copied().unwrap();
    //         previous_paths = with_len
    //             .into_iter()
    //             .filter(|(l, _)| *l == shortest)
    //             .map(|(_, p)| p)
    //             .collect();
    //     }

    //     let shortest: usize = previous_paths.iter().map(|p| p.len()).min().unwrap();
    //     println!("{} -> {}", code, shortest);
    //     shortest
    // }
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
    println!("Part1: {}", keypads.solve(2));
    println!("Part2: {}", keypads.solve(25));
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
