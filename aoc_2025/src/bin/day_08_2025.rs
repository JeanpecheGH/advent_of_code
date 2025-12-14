use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::str::FromStr;
use util::basic_parser::parse_pos3;
use util::coord::Pos3;

#[derive(Debug, Eq, PartialEq)]
struct Junction {
    a: usize,
    b: usize,
    dist: usize,
}

impl Junction {
    fn from_pos(pos: &[(usize, &Pos3)]) -> Junction {
        let dist: usize = pos[0].1.distance_squared(*pos[1].1);
        Junction {
            a: pos[0].0,
            b: pos[1].0,
            dist,
        }
    }
}

impl Ord for Junction {
    fn cmp(&self, other: &Self) -> Ordering {
        other.dist.cmp(&self.dist)
    }
}

impl PartialOrd for Junction {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
struct DisjointSet {
    parents: Vec<usize>,
    sizes: Vec<usize>,
    nb_sets: usize,
}

impl DisjointSet {
    fn new(nb_sets: usize) -> DisjointSet {
        let parents: Vec<usize> = (0..nb_sets).collect();
        let sizes: Vec<usize> = vec![1; nb_sets];
        DisjointSet {
            parents,
            sizes,
            nb_sets,
        }
    }

    fn join(&mut self, a: usize, b: usize) {
        let prt_a: usize = self.parents[a];
        let prt_b: usize = self.parents[b];

        let (tgt_grp, src_grp) = match prt_a.cmp(&prt_b) {
            Ordering::Less => (prt_a, prt_b),
            Ordering::Greater => (prt_b, prt_a),
            Ordering::Equal => return, // Nothing to do
        };

        for idx in self.parents.iter_mut() {
            if *idx == src_grp {
                *idx = tgt_grp
            }
        }

        self.sizes[tgt_grp] += self.sizes[src_grp];
        self.sizes[src_grp] = 0;
        self.nb_sets -= 1;
    }

    fn top_3(&self) -> usize {
        self.sizes.iter().sorted_unstable().rev().take(3).product()
    }

    fn is_complete(&self) -> bool {
        self.nb_sets == 1
    }
}

struct Playground {
    boxes: Vec<Pos3>,
}

impl Playground {
    fn build_circuit(&self, nb_junctions: usize) -> (usize, usize) {
        //Compute and sort all distances
        let mut queue: BinaryHeap<Junction> = self
            .boxes
            .iter()
            .enumerate()
            .combinations(2)
            .map(|pair| Junction::from_pos(&pair))
            .collect();

        let mut set: DisjointSet = DisjointSet::new(self.boxes.len());
        // Apply the first nb_junctions junctions
        for _ in 0..nb_junctions {
            let junction = queue.pop().unwrap();
            set.join(junction.a, junction.b);
        }

        let largest_circuit_product: usize = set.top_3();

        // Apply the remaining junctions until we link all boxes
        while let Some(junction) = queue.pop() {
            set.join(junction.a, junction.b);
            if set.is_complete() {
                return (
                    largest_circuit_product,
                    self.boxes[junction.a].0 * self.boxes[junction.b].0,
                );
            }
        }
        (0, 0)
    }
}

impl FromStr for Playground {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let boxes: Vec<Pos3> = s.lines().map(|l| parse_pos3(l).unwrap().1).collect();

        Ok(Playground { boxes })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2025/input/day_08.txt").expect("Cannot open input file");
    let playground: Playground = s.parse().unwrap();
    let (largest_circuit_product, one_circuit) = playground.build_circuit(1000);
    println!(
        "Part1: The product of the size of the 3 largest circuits is {}",
        largest_circuit_product
    );
    println!(
        "Part2: The last boxes connected give a product of {}",
        one_circuit
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689
";
    #[test]
    fn test_1() {
        let playground: Playground = EXAMPLE_1.parse().unwrap();
        assert_eq!(playground.build_circuit(10), (40, 25272));
    }
}
