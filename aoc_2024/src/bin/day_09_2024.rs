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
    fn block_checksum(id: usize, pos: usize, size: usize) -> usize {
        id / 2 * size * (size + 2 * pos - 1) / 2
    }

    fn checksum(&self) -> usize {
        let mut disk: Vec<usize> = self.disk_map.clone();
        let mut sum: usize = 0;
        let mut curr_pos: usize = 0;
        let mut front_id: usize = 0;
        let mut back_id: usize = disk.len() - 1;

        while front_id <= back_id {
            let mut block_size: usize = disk[front_id];
            if front_id.is_multiple_of(2) {
                //Used space, compute partial checksum
                sum += FileSystem::block_checksum(front_id, curr_pos, block_size);
                curr_pos += block_size;
            } else {
                //Empty space, fill from the back block
                while block_size > 0 {
                    let nb_back: usize = disk[back_id];
                    if nb_back > block_size {
                        //Fill all this block
                        sum += FileSystem::block_checksum(back_id, curr_pos, block_size);
                        //Consume partially the back block
                        disk[back_id] -= block_size;
                        curr_pos += block_size;
                        block_size = 0;
                    } else {
                        //Fill block partially
                        sum += FileSystem::block_checksum(back_id, curr_pos, nb_back);
                        block_size -= nb_back;
                        //Back block consumed. Go to the next non-empty block in the back
                        back_id -= 2;
                        curr_pos += nb_back;
                    }
                }
            }
            //Go to the next front block
            front_id += 1;
        }

        sum
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

    #[test]
    fn test_block() {
        let id: usize = 44;
        let a: usize = 17;
        let n: usize = 15;
        let sum_1: usize = (a..a + n).sum();
        let sum_2: usize = FileSystem::block_checksum(id, a, n);
        assert_eq!((id / 2) * sum_1, sum_2);
    }
    #[test]
    fn test_empty_block() {
        let id: usize = 44;
        let a: usize = 17;
        let n: usize = 0;
        let sum_1: usize = (a..a + n).sum();
        let sum_2: usize = FileSystem::block_checksum(id, a, n);
        assert_eq!((id / 2) * sum_1, sum_2);
    }
    #[test]
    fn test_block_null_id() {
        let id: usize = 0;
        let a: usize = 17;
        let n: usize = 15;
        let sum_1: usize = (a..a + n).sum();
        let sum_2: usize = FileSystem::block_checksum(id, a, n);
        assert_eq!((id / 2) * sum_1, sum_2);
    }
}
