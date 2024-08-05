use fxhash::FxHashMap;
use std::str::FromStr;
use util::basic_parser::usize_list;

#[derive(Debug, Clone)]
struct Memory {
    blocks: Vec<usize>,
}

impl Memory {
    fn redistribution_cycles(&self) -> (usize, usize) {
        let mut mem: Vec<usize> = self.blocks.clone();
        let mut cache: FxHashMap<Vec<usize>, usize> = FxHashMap::default();

        let mut nb_cycles: usize = 0;
        let mut return_opt: Option<usize> = cache.insert(mem.clone(), nb_cycles);

        while return_opt.is_none() {
            let (mut max, mut pos_max): (usize, usize) =
                mem.iter()
                    .enumerate()
                    .fold(
                        (0, 0),
                        |(max, pos_max), (pos, &v)| {
                            if v > max {
                                (v, pos)
                            } else {
                                (max, pos_max)
                            }
                        },
                    );

            mem[pos_max] = 0;
            while max > 0 {
                pos_max = (pos_max + 1) % mem.len();
                mem[pos_max] += 1;
                max -= 1;
            }
            nb_cycles += 1;
            return_opt = cache.insert(mem.clone(), nb_cycles);
        }

        (nb_cycles, nb_cycles - return_opt.unwrap())
    }
}

impl FromStr for Memory {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let blocks: Vec<usize> = s.lines().next().map(|l| usize_list(l).unwrap().1).unwrap();
        Ok(Memory { blocks })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2017/input/day_06.txt").expect("Cannot open input file");
    let memory: Memory = s.parse().unwrap();
    let (nb_cycle, loop_cycle): (usize, usize) = memory.redistribution_cycles();

    println!("Part1: The memory returns to a know state after {nb_cycle} cycles");
    println!("Part2: The loop between two identical state is {loop_cycle} cycles long");
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "0  2   7   0";

    #[test]
    fn part_1() {
        let memory: Memory = EXAMPLE_1.parse().unwrap();
        assert_eq!((5, 4), memory.redistribution_cycles());
    }
}
