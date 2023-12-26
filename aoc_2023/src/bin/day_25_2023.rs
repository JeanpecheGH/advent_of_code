use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::multi::separated_list1;
use nom::sequence::terminated;
use nom::IResult;
use rand::{thread_rng, Rng};
use std::collections::HashSet;
use std::str::FromStr;

#[derive(Clone, Debug)]
struct Node {
    nodes: HashSet<usize>,
}

impl Node {
    fn new() -> Node {
        Node {
            nodes: HashSet::new(),
        }
    }

    fn from(n: usize) -> Node {
        let nodes: HashSet<usize> = HashSet::from([n]);
        Node { nodes }
    }

    fn is_empty(&self) -> bool {
        self.nodes.is_empty()
    }

    fn extends(&mut self, other: Node) {
        self.nodes.extend(other.nodes.iter())
    }

    fn contains(&self, n: &usize) -> bool {
        self.nodes.contains(n)
    }
}
#[derive(Clone, Debug)]
struct Graph {
    nodes: Vec<Node>,
    edges: Vec<(usize, usize)>,
}

impl Graph {
    fn min_cut(&mut self) {
        let nb: usize = self.nodes.len();
        if nb <= 6 {
            self.contract_until(2);
        } else {
            let lim: usize = 1 + (nb as f64 / f64::sqrt(2f64)).ceil() as usize;
            self.contract_until(lim);
            self.min_cut();

            let mut other: Graph = self.clone();
            other.contract_until(lim);
            other.min_cut();
            if other.nodes.len() < self.nodes.len() {
                *self = other;
            }
        }
    }
    fn contract_until(&mut self, limit: usize) {
        while self.nodes.len() > limit {
            self.contract();
        }
    }
    fn contract(&mut self) {
        //Get random edge
        let rand_idx: usize = thread_rng().gen_range(0..self.edges.len());
        let (a, b) = self.edges[rand_idx];

        //Merge the two nodes linked by this edge
        let mut store: Node = Node::new();
        let mut i: usize = 0;
        while i < self.nodes.len() {
            let n: &mut Node = self.nodes.get_mut(i).unwrap();
            if n.contains(&a) || n.contains(&b) {
                if store.is_empty() {
                    store = n.clone();
                    self.nodes.remove(i);
                } else {
                    n.extends(store);
                    store = n.clone();
                    break;
                }
            } else {
                i += 1;
            }
        }
        //Delete all edges that connected the two nodes before merging
        self.edges
            .retain(|(i, j)| !store.contains(i) || !store.contains(j));
    }

    fn from_nodes_edges(nodes: Vec<usize>, edges: Vec<(usize, usize)>) -> Graph {
        let nodes: Vec<Node> = nodes.into_iter().map(Node::from).collect();
        Graph { nodes, edges }
    }

    fn score(&self) -> usize {
        self.nodes.iter().map(|n| n.nodes.len()).product()
    }
}

struct Component {
    id: usize,
    wired_to: Vec<usize>,
}

impl FromStr for Component {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_component(s: &str) -> IResult<&str, Component> {
            fn to_usize(s: &str) -> usize {
                s.chars()
                    .map(|c| c as usize - 'a' as usize + 1)
                    .fold(0, |acc, c| acc * 30 + c)
            }
            let (s, id) = terminated(alpha1, tag(": "))(s)?;
            let (s, wired_to) = separated_list1(char(' '), alpha1)(s)?;
            let id = to_usize(id);
            let wired_to = wired_to.into_iter().map(to_usize).collect();

            Ok((s, Component { id, wired_to }))
        }

        Ok(parse_component(s).unwrap().1)
    }
}

struct Snowverload {
    components: Vec<Component>,
}

impl Snowverload {
    fn min_cut_product(&self) -> usize {
        let graph: Graph = self.build_graph();

        loop {
            let mut g: Graph = graph.clone();
            g.min_cut();
            if g.edges.len() == 3 {
                return g.score();
            }
        }
    }

    fn build_graph(&self) -> Graph {
        //Get a set of all component id
        let id_set: HashSet<usize> = self
            .components
            .iter()
            .flat_map(|c| {
                let mut v: Vec<usize> = c.wired_to.clone();
                v.push(c.id);
                v
            })
            .collect();
        let nodes: Vec<usize> = id_set.into_iter().collect();

        //Get a vec of all edges
        let edges: Vec<(usize, usize)> = self
            .components
            .iter()
            .flat_map(|c| c.wired_to.iter().map(|&c_2| (c.id, c_2)))
            .collect();

        Graph::from_nodes_edges(nodes, edges)
    }
}

impl FromStr for Snowverload {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let components: Vec<Component> = s.lines().map(|l| l.parse().unwrap()).collect();
        Ok(Snowverload { components })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2023/input/day_25.txt").expect("Cannot open input file");
    let snowverload: Snowverload = s.parse().unwrap();
    println!(
        "After cutting the three wires, the product of the size of the two groups is {}",
        snowverload.min_cut_product()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr
";

    #[test]
    fn part_1() {
        let snowverload: Snowverload = EXAMPLE_1.parse().unwrap();
        assert_eq!(snowverload.min_cut_product(), 54);
    }
}
