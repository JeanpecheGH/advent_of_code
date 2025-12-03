use nom::character::complete::space0;
use nom::multi::count;
use nom::sequence::terminated;
use nom::IResult;
use nom::Parser;
use std::str::FromStr;
use util::basic_parser::parse_usize;

#[derive(Debug)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

impl Node {
    fn metadata_sum(&self) -> usize {
        let child_sum: usize = self.children.iter().map(|child| child.metadata_sum()).sum();
        let meta_sum: usize = self.metadata.iter().sum();
        child_sum + meta_sum
    }

    fn root_value(&self) -> usize {
        if self.children.is_empty() {
            self.metadata.iter().sum()
        } else {
            self.metadata
                .iter()
                .filter_map(|m| {
                    if (1..=self.children.len()).contains(m) {
                        Some(self.children[m - 1].root_value())
                    } else {
                        None
                    }
                })
                .sum()
        }
    }
}

impl FromStr for Node {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn usize_space(s: &str) -> IResult<&str, usize> {
            terminated(parse_usize, space0).parse(s)
        }
        fn parse_node(s: &str) -> IResult<&str, Node> {
            let (s, nb_child) = usize_space(s)?;
            let (s, nb_meta) = usize_space(s)?;

            let (s, children) = count(parse_node, nb_child).parse(s)?;
            let (s, metadata) = count(usize_space, nb_meta).parse(s)?;

            Ok((s, Node { children, metadata }))
        }

        Ok(parse_node(s).unwrap().1)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2018/input/day_08.txt").expect("Cannot open input file");
    let node: Node = s.parse().unwrap();

    println!("Part1: The sum of all metadata is {}", node.metadata_sum());
    println!("Part2: The value of the root node is {}", node.root_value());
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2";

    #[test]
    fn part_1() {
        let node: Node = EXAMPLE_1.parse().unwrap();
        assert_eq!(node.metadata_sum(), 138);
    }
    #[test]
    fn part_2() {
        let node: Node = EXAMPLE_1.parse().unwrap();
        assert_eq!(node.root_value(), 66);
    }
}
