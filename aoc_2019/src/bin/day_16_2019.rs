use std::str::FromStr;

#[derive(Clone)]
struct Transmission {
    elements: Vec<u8>,
    offset: usize,
}

impl Transmission {
    fn phase(&mut self) {
        let size = self.elements.len();
        let mut new_elements: Vec<i32> = vec![0; size];
        for i in 0..size {
            let n: i32 = self.elements[i] as i32;
            //Distribute this element in every output elements
            for (j, elem) in new_elements.iter_mut().enumerate().take(i + 1) {
                let i_plus: i32 = i as i32 + 1;
                let j_plus: i32 = j as i32 + 1;
                let pattern_pos: i32 = (i_plus % (j_plus * 4)) / j_plus;

                let mul = match pattern_pos {
                    1 => 1,
                    3 => -1,
                    _ => 0,
                };
                *elem += mul * n;
            }
        }
        self.elements = new_elements
            .into_iter()
            .map(|n| (n % 10).unsigned_abs() as u8)
            .collect();
    }

    fn simple_phase(&mut self) {
        let size = self.elements.len();
        let mut new_elements: Vec<i32> = vec![0; size];

        let mut n: i32 = 0;
        for i in (0..size).rev() {
            n += self.elements[i] as i32;
            new_elements[i] = n;
        }
        self.elements = new_elements
            .into_iter()
            .map(|n| (n % 10).unsigned_abs() as u8)
            .collect();
    }

    fn n_phase(&mut self, n: usize, simple: bool) {
        for _ in 0..n {
            if simple {
                self.simple_phase();
            } else {
                self.phase();
            }
        }
    }

    fn multiply_and_offset(&mut self, n: usize) {
        let clone: Vec<u8> = self.elements.clone();
        let size: usize = clone.len();
        let d: usize = self.offset / size;
        let r: usize = self.offset % size;

        self.elements = self.elements[r..].to_vec();

        for _ in 0..(n - d - 1) {
            self.elements.extend(&clone);
        }
    }

    fn first_eight(&self) -> usize {
        self.elements[0..8].iter().fold(0, |mut acc, &n| {
            acc *= 10;
            acc += n as usize;
            acc
        })
    }
}

impl FromStr for Transmission {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let elements: Vec<u8> = s
            .lines()
            .next()
            .unwrap()
            .chars()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        let offset: usize = elements[0..7].iter().fold(0, |mut acc, &n| {
            acc *= 10;
            acc += n as usize;
            acc
        });
        Ok(Transmission { elements, offset })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_16.txt").expect("Cannot open input file");
    let mut transmission: Transmission = s.parse().unwrap();
    let mut transmission_2: Transmission = transmission.clone();
    transmission.n_phase(100, false);
    println!(
        "Part1: The first digits of the output are {:#8}",
        transmission.first_eight()
    );
    transmission_2.multiply_and_offset(10_000);
    transmission_2.n_phase(100, true);
    println!(
        "Part2: The 8 digits message is {:#8}",
        transmission_2.first_eight()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_1_part_1() {
        let mut transmission: Transmission = "12345678".parse().unwrap();
        transmission.phase();
        assert_eq!(transmission.first_eight(), 48226158);
        transmission.phase();
        assert_eq!(transmission.first_eight(), 34040438);
        transmission.phase();
        assert_eq!(transmission.first_eight(), 03415518);
        transmission.phase();
        assert_eq!(transmission.first_eight(), 01029498);
    }

    #[test]
    fn test_input_2_part_1() {
        let mut transmission: Transmission = "80871224585914546619083218645595".parse().unwrap();
        transmission.n_phase(100, false);
        assert_eq!(transmission.first_eight(), 24176176);
    }

    #[test]
    fn test_input_3_part_1() {
        let mut transmission: Transmission = "19617804207202209144916044189917".parse().unwrap();
        transmission.n_phase(100, false);
        assert_eq!(transmission.first_eight(), 73745418);
    }

    #[test]
    fn test_input_4_part_1() {
        let mut transmission: Transmission = "69317163492948606335995924319873".parse().unwrap();
        transmission.n_phase(100, false);
        assert_eq!(transmission.first_eight(), 52432133);
    }

    #[test]
    fn test_multiply_and_offset() {
        let mut transmission: Transmission = "0000038789".parse().unwrap();
        assert_eq!(transmission.offset, 38);
        transmission.multiply_and_offset(5);
        assert_eq!(
            transmission.elements,
            vec![8, 9, 0, 0, 0, 0, 0, 3, 8, 7, 8, 9]
        )
    }

    #[test]
    fn test_input_1_part_2() {
        let mut transmission: Transmission = "03036732577212944063491565474664".parse().unwrap();
        transmission.multiply_and_offset(10_000);
        transmission.n_phase(100, true);
        assert_eq!(transmission.offset, 0303673);
        assert_eq!(transmission.first_eight(), 84462026);
    }
    #[test]
    fn test_input_2_part_2() {
        let mut transmission: Transmission = "02935109699940807407585447034323".parse().unwrap();
        transmission.multiply_and_offset(10_000);
        transmission.n_phase(100, true);
        assert_eq!(transmission.offset, 0293510);
        assert_eq!(transmission.first_eight(), 78725270);
    }
    #[test]
    fn test_input_3_part_2() {
        let mut transmission: Transmission = "03081770884921959731165446850517".parse().unwrap();
        transmission.multiply_and_offset(10_000);
        transmission.n_phase(100, true);
        assert_eq!(transmission.offset, 0308177);
        assert_eq!(transmission.first_eight(), 53553731);
    }
}
