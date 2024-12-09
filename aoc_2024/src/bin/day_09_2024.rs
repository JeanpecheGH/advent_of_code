use std::collections::VecDeque;
use std::str::FromStr;

struct Block {
    length: usize,
    id: Option<usize>,
}

impl Block {
    fn checksum(&self, idx: usize) -> (usize, usize) {
        let new_idx: usize = idx + self.length;
        (
            new_idx,
            self.id.unwrap_or(0) * self.length * (idx + new_idx - 1) / 2,
        )
    }
}

struct FileSystem {
    disk_map: Vec<usize>,
}

impl FileSystem {
    fn checksum(&self) -> usize {
        let mut fragmented: Vec<Option<usize>> = Vec::new();
        let mut nb_empty: usize = 0;
        for (idx, &n) in self.disk_map.iter().enumerate() {
            if idx % 2 == 0 {
                // Data
                (0..n).for_each(|_| fragmented.push(Some(idx / 2)));
            } else {
                //Empty
                nb_empty += n;
                (0..n).for_each(|_| fragmented.push(None));
            }
        }

        let mut idx_refill: usize = 0;
        //Densify
        while nb_empty > 0 {
            match fragmented.pop().unwrap() {
                None => (),
                Some(n) => {
                    //Insert from the start
                    while fragmented[idx_refill].is_some() {
                        idx_refill += 1;
                    }
                    fragmented[idx_refill] = Some(n);
                }
            }
            nb_empty -= 1;
        }

        //Compute checksum
        fragmented
            .iter()
            .enumerate()
            .map(|(idx, &opt)| idx * opt.unwrap_or(0))
            .sum()
    }

    fn checksum_by_block(&self) -> usize {
        let mut fragmented: Vec<Block> = Vec::new();
        for (idx, &n) in self.disk_map.iter().enumerate() {
            if idx % 2 == 0 {
                fragmented.push(Block {
                    length: n,
                    id: Some(idx / 2),
                })
            } else {
                fragmented.push(Block {
                    length: n,
                    id: None,
                })
            }
        }

        //Densify
        let mut tail: VecDeque<Block> = VecDeque::new();

        while let Some(b) = fragmented.pop() {
            if b.id.is_none() {
                tail.push_front(b);
            } else {
                //Check for a big enough space
                if let Some(pos) = fragmented
                    .iter()
                    .position(|other_bl| other_bl.id.is_none() && other_bl.length >= b.length)
                {
                    //Split the space block if needed
                    let old_len = fragmented[pos].length;
                    if old_len > b.length {
                        fragmented.insert(
                            pos + 1,
                            Block {
                                length: old_len - b.length,
                                id: None,
                            },
                        )
                    }
                    //Empty the current position
                    tail.push_front(Block {
                        length: b.length,
                        id: None,
                    });
                    //Insert the new block
                    fragmented[pos] = b;
                } else {
                    tail.push_front(b);
                }
            }
        }

        //Compute checksum
        tail.iter()
            .fold((0, 0), |(idx, sum), block| {
                let (new_idx, part_sum) = block.checksum(idx);
                (new_idx, sum + part_sum)
            })
            .1
    }
}

impl FromStr for FileSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let disk_map: Vec<usize> = s
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect();
        Ok(FileSystem { disk_map })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2024/input/day_09.txt").expect("Cannot open input file");
    let fs: FileSystem = s.parse().unwrap();
    println!("Part1: The filesystem checksum is {}", fs.checksum());
    println!(
        "Part2: Avoiding file fragmentation, the filesystem checksum is {}",
        fs.checksum_by_block()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_1: &str = "2333133121414131402";

    #[test]
    fn part_1() {
        let fs: FileSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!(fs.checksum(), 1928);
    }
    #[test]
    fn part_2() {
        let fs: FileSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!(fs.checksum_by_block(), 2858);
    }
}
