use std::collections::BTreeSet;
use std::str::FromStr;

struct Block {
    id: usize,
    pos: usize,
    length: usize,
}

impl Block {
    fn checksum(&self, new_pos: Option<usize>) -> usize {
        let final_pos: usize = new_pos.unwrap_or(self.pos);
        self.id * self.length * (2 * final_pos + self.length - 1) / 2
    }
}

#[derive(Debug)]
struct EmptyCache {
    cache: [BTreeSet<usize>; 9],
}

impl EmptyCache {
    fn new() -> EmptyCache {
        EmptyCache {
            cache: Default::default(),
        }
    }

    fn insert_empty_block(&mut self, pos: usize, length: usize) {
        self.cache[length - 1].insert(pos);
    }

    fn remove_block(&mut self, length: usize) {
        self.cache[length - 1].pop_first();
    }

    fn first_available(&self, length: usize) -> Option<(usize, usize)> {
        (length - 1..9)
            .filter_map(|i| self.cache[i].first().map(|&pos| (pos, i + 1)))
            .min_by(|&(pos_a, _), &(pos_b, _)| pos_a.cmp(&pos_b))
    }

    fn move_block(&mut self, block: &Block) -> Option<usize> {
        //Find the leftmost empty block big enough
        let (pos, length) = self.first_available(block.length)?;
        if pos > block.pos {
            //Don't move the block, it's already well-placed
            return None;
        }
        //Split empty block if it was bigger
        if length > block.length {
            let new_length: usize = length - block.length;
            let new_pos: usize = pos + block.length;
            self.insert_empty_block(new_pos, new_length);
        }
        self.remove_block(length);

        Some(pos)
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
        //First pass
        //Store blocks in a new array with their id, length and actual position
        //Store empty spaces in a cache
        let (_, mut cache, blocks): (usize, EmptyCache, Vec<Block>) =
            self.disk_map.iter().enumerate().fold(
                (0, EmptyCache::new(), Vec::default()),
                |(mut curr_pos, mut cache, mut blocks), (idx, &len)| {
                    if len > 0 {
                        if idx.is_multiple_of(2) {
                            let b: Block = Block {
                                id: idx / 2,
                                pos: curr_pos,
                                length: len,
                            };
                            blocks.push(b);
                        } else {
                            cache.insert_empty_block(curr_pos, len);
                        }
                        curr_pos += len;
                    }
                    (curr_pos, cache, blocks)
                },
            );

        //Second pass (from the right)
        // Try to move each block to the leftmost open space
        // Split the empty space used if need be.
        // Compute checksum in the new place
        blocks
            .into_iter()
            .rev()
            .map(|b| {
                let new_pos: Option<usize> = cache.move_block(&b);
                b.checksum(new_pos)
            })
            .sum()
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
