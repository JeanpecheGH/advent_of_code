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
        let hash: String = format!("{:x}", digest);
        let doors_vec: Vec<bool> = hash
            .chars()
            .take(4)
            .map(|c| ('b'..='f').contains(&c))
            .collect();
        let doors: [bool; 4] = [doors_vec[0], doors_vec[1], doors_vec[2], doors_vec[3]];
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
            let idx: usize =
                candidates.partition_point(|x| x.score() >= n.score() && x.dist() > n.dist());
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
            Some(n) => n.neighbours().into_iter().for_each(|n| {
                let idx: usize =
                    candidates.partition_point(|x| x.score() >= n.score() && x.dist() > n.dist());
                candidates.insert(idx, n)
            }),
        }
    }
    println!(
        "Part2: The longest path is {} step long, found in {:?}",
        worst_node.path_string().len(),
        now.elapsed()
    );
}
