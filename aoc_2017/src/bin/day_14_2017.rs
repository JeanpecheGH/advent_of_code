use util::coord::Pos;
use util::hashers::KnotHash;

const INPUT: &str = "xlqgujun";

#[derive(Debug, Clone)]
struct DiskDefragmenter {
    grid: Vec<Vec<bool>>,
}

impl DiskDefragmenter {
    fn nb_used(&self) -> usize {
        self.grid
            .iter()
            .map(|row| row.iter().filter(|&&b| b).count())
            .sum()
    }

    fn nb_regions(&self) -> usize {
        let mut groups: Vec<Vec<Pos>> = Vec::new();

        for (y, row) in self.grid.iter().enumerate() {
            for (x, &b) in row.iter().enumerate() {
                if b {
                    let pos: Pos = Pos(x, y);
                    //Partition in groups that are in range of our star and those that are not
                    let (in_range, mut out_of_range): (Vec<Vec<Pos>>, Vec<Vec<Pos>>) = groups
                        .into_iter()
                        .partition(|c| c.iter().any(|s| s.distance(pos) <= 1));
                    //Merge the group in range and add our new pos to it
                    let mut merge: Vec<Pos> = in_range.into_iter().flatten().collect();
                    merge.push(pos);
                    out_of_range.push(merge);
                    groups = out_of_range;
                }
            }
        }

        groups.len()
    }

    fn from_str(s: &str) -> DiskDefragmenter {
        let grid: Vec<Vec<bool>> = (0..128)
            .map(|i| {
                let source: String = format!("{s}-{i}");
                let khash: KnotHash = KnotHash::new(&source);
                khash
                    .hash_vec()
                    .into_iter()
                    .flat_map(|mut n| {
                        let mut v: Vec<bool> = Vec::new();
                        (0..8).for_each(|_| {
                            v.push(n % 2 != 0);
                            n /= 2;
                        });
                        v.reverse();
                        v
                    })
                    .collect()
            })
            .collect();

        DiskDefragmenter { grid }
    }
}

fn main() {
    let now = std::time::Instant::now();
    let defrag: DiskDefragmenter = DiskDefragmenter::from_str(INPUT);
    println!("Part1: {} bits are used", defrag.nb_used());
    println!(
        "Part2: The used bits are forming {} groups",
        defrag.nb_regions()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "flqrgnkx";

    #[test]
    fn part_1() {
        let defrag: DiskDefragmenter = DiskDefragmenter::from_str(EXAMPLE_1);
        assert_eq!(8108, defrag.nb_used());
    }
    #[test]
    fn part_2() {
        let defrag: DiskDefragmenter = DiskDefragmenter::from_str(EXAMPLE_1);
        assert_eq!(1242, defrag.nb_regions());
    }
}
