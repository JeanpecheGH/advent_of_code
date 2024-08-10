use std::collections::VecDeque;

const INPUT: usize = 324;

#[derive(Debug, Clone)]
struct Spinlock {
    speed: usize,
}

impl Spinlock {
    fn after(&self, len: usize) -> usize {
        let mut v: VecDeque<usize> = VecDeque::new();
        v.push_front(0);

        for i in 1..=len {
            //Vec rotation is the fastest way to do this
            v.rotate_left(self.speed % v.len());
            v.push_back(i);
        }
        v.front().copied().unwrap()
    }

    fn after_zero(&self, len: usize) -> usize {
        //No need to compute the vector, we just want the last value placed after 0
        let mut insert_position: usize = 0;
        let mut after_zero: usize = 0;
        for i in 1..=len {
            insert_position = (insert_position + self.speed + 1) % i;
            if insert_position == 0 {
                after_zero = i;
            }
        }
        after_zero
    }
}

fn main() {
    let now = std::time::Instant::now();
    let spinlock: Spinlock = Spinlock { speed: INPUT };
    println!(
        "Part1: The value right after 2017 is {}",
        spinlock.after(2017)
    );
    println!(
        "Part2: The value right after 0 is {}",
        spinlock.after_zero(50_000_000)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn part_1() {
        let spinlock: Spinlock = Spinlock { speed: 3 };
        assert_eq!(638, spinlock.after(2017));
    }
    #[test]
    fn part_2() {
        let spinlock: Spinlock = Spinlock { speed: 3 };
        assert_eq!(1222153, spinlock.after_zero(50_000_000));
    }
}
