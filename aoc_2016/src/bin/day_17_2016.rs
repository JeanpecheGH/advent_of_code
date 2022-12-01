use std::cmp::Ordering;

const INPUT: &str = "mmsxrhfx";

#[derive(Debug, Clone)]
struct Node {
    x: i16,
    y: i16,
    doors: [bool; 4],
    path: Vec<char>,
}

impl Node {
    fn new(x: i16, y: i16, path: Vec<char>) -> Self {
        let word = format!("{}{}", INPUT, path.iter().collect::<String>());
        let digest = md5::compute(word);
        let doors: [bool; 4] = Node::bytes_to_bool(digest.0);
        Node { x, y, doors, path }
    }

    fn path_string(&self) -> String {
        self.path.iter().collect()
    }

    fn neighbours(&self) -> Vec<Node> {
        self.neighbours_pos()
            .into_iter()
            .map(|(c, x, y)| {
                let mut new_path = self.path.clone();
                new_path.push(c);
                Node::new(x, y, new_path)
            })
            .collect()
    }

    fn neighbours_pos(&self) -> Vec<(char, i16, i16)> {
        let candidates = vec![
            ('U', self.x - 1, self.y),
            ('D', self.x + 1, self.y),
            ('L', self.x, self.y - 1),
            ('R', self.x, self.y + 1),
        ];
        candidates
            .into_iter()
            .enumerate()
            .filter(|&(i, (_, x, y))| (0..4).contains(&x) && (0..4).contains(&y) && self.doors[i])
            .map(|(_, t)| t)
            .collect()
    }

    fn score(&self) -> usize {
        self.path.len() + self.dist()
    }

    fn dist(&self) -> usize {
        6 - self.x as usize - self.y as usize
    }

    fn is_end(&self) -> bool {
        self.x == 3 && self.y == 3
    }

    fn bytes_to_bool(bytes: [u8; 16]) -> [bool; 4] {
        let mut bools: [bool; 4] = [false; 4];
        for i in 0..2 {
            let v: u8 = bytes[i];
            bools[i * 2] = (v >> 4) > 10;
            bools[i * 2 + 1] = (v & 0x0F) > 10;
        }
        bools
    }
}

fn main() {
    let node: Node = Node::new(0, 0, Vec::new());

    let now = std::time::Instant::now();
    let mut candidates: Vec<Node> = vec![node.clone()];
    loop {
        let best_node = candidates.pop().unwrap();
        if best_node.is_end() {
            println!(
                "Part1: The shortest path is {}, found in {:?}",
                best_node.path_string(),
                now.elapsed()
            );
            break;
        }
        best_node.neighbours().into_iter().for_each(|n| {
            let idx: usize = candidates.partition_point(|x| match x.score().cmp(&n.score()) {
                Ordering::Less => false,
                Ordering::Greater => true,
                Ordering::Equal => x.dist() > n.dist(),
            });
            candidates.insert(idx, n)
        })
    }

    let now = std::time::Instant::now();
    candidates = vec![node.clone()];
    let mut worst_node: Node = node;
    loop {
        let first_node = candidates.pop();
        match first_node {
            None => break,
            Some(n) if n.is_end() => {
                if n.path_string().len() > worst_node.path_string().len() {
                    worst_node = n;
                }
            }
            Some(n) => candidates.append(&mut n.neighbours()),
        }
    }
    println!(
        "Part2: The longest path is {} step long, found in {:?}",
        worst_node.path_string().len(),
        now.elapsed()
    );
}
