use fxhash::{FxHashMap, FxHashSet};
use itertools::Itertools;
use std::str::FromStr;
use util::basic_parser::parse_pos3;
use util::coord::Pos3;

struct Playground {
    boxes: Vec<Pos3>,
}

impl Playground {
    fn apply_junctions(
        &self,
        junctions: &[(Pos3, Pos3)],
        existing_circuits: &mut Vec<FxHashSet<Pos3>>,
    ) -> usize {
        let mut circuits: Vec<FxHashSet<Pos3>> = existing_circuits.clone();
        for &(a, b) in junctions {
            let mut new_circuit: FxHashSet<Pos3> = FxHashSet::from_iter([a, b]);
            let mut new_circuits: Vec<FxHashSet<Pos3>> = Vec::new();

            for circuit in circuits {
                if circuit.contains(&a) || circuit.contains(&b) {
                    new_circuit.extend(circuit.iter())
                } else {
                    new_circuits.push(circuit);
                }
            }
            if new_circuit.len() == self.boxes.len() {
                return a.0 * b.0;
            }
            new_circuits.push(new_circuit);
            circuits = new_circuits
        }
        *existing_circuits = circuits;
        0
    }

    fn build_circuit(&self, nb_junctions: usize) -> (usize, usize) {
        //Compute all distances
        let mut dist_dict: FxHashMap<(Pos3, Pos3), usize> = FxHashMap::default();

        for (i, &pos_a) in self.boxes.iter().enumerate() {
            for j in (i + 1)..self.boxes.len() {
                let pos_b = self.boxes[j];
                let dist: usize = pos_a.distance_squared(pos_b);
                dist_dict.insert((pos_a, pos_b), dist);
            }
        }

        let junctions: Vec<(Pos3, Pos3)> = dist_dict
            .iter()
            .sorted_unstable_by(|(_, a), (_, b)| a.cmp(b))
            .map(|((a, b), _)| (*a, *b))
            .collect();

        let mut circuits: Vec<FxHashSet<Pos3>> = Vec::new();
        // Apply the first nb_junctions junctions
        self.apply_junctions(&junctions[0..nb_junctions], &mut circuits);

        let largest_circuit_product: usize = circuits
            .iter()
            .map(|c| c.len())
            .sorted_unstable()
            .rev()
            .take(3)
            .product();

        // Apply the remaining junctions until we link all boxes
        let one_circuit: usize = self.apply_junctions(&junctions[nb_junctions..], &mut circuits);
        (largest_circuit_product, one_circuit)
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
    println!("Part1: {}", largest_circuit_product);
    println!("Part2: {}", one_circuit);
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
