struct Marker<'a> {
    sequence: &'a str,
    len: usize,
    times: usize,
}

impl Marker<'_> {
    fn size(&self) -> usize {
        self.len * self.times
    }

    fn parse_marker(s: &str) -> (usize, usize) {
        let numbers: Vec<&str> = s.split('x').collect();
        let len = numbers[0].parse().unwrap();
        let times = numbers[1].parse().unwrap();
        (len, times)
    }

    fn rec_size(&self) -> usize {
        let bytes: &[u8] = self.sequence.as_bytes();

        let mut seq_size: usize = 0;
        let mut i: usize = 0;
        while i < bytes.len() {
            match bytes[i] {
                OPEN => {
                    let end = self.sequence[i..].find(')').unwrap() + i;
                    let (len, times) = Marker::parse_marker(&self.sequence[i + 1..end]);
                    let marker = Marker {
                        sequence: &self.sequence[end + 1..=end + len],
                        len,
                        times,
                    };
                    seq_size += marker.rec_size();
                    i = end + marker.len + 1;
                }
                _ => {
                    seq_size += 1;
                    i += 1;
                }
            }
        }
        seq_size * self.times
    }
}

const OPEN: u8 = b'(';

fn main() {
    let line: String =
        util::file_as_string("aoc_2016/input/day_09.txt").expect("Cannot open input file");
    let coded = line.strip_suffix("\r\n").unwrap();

    let line_bytes: &[u8] = coded.as_bytes();

    let mut part1_size: usize = 0;
    let mut i: usize = 0;
    while i < line_bytes.len() {
        match line_bytes[i] {
            OPEN => {
                let end = coded[i..].find(')').unwrap() + i;
                let (len, times) = Marker::parse_marker(&coded[i + 1..end]);
                let marker = Marker {
                    sequence: &coded[end + 1..end + len],
                    len,
                    times,
                };
                part1_size += marker.size();
                i = end + marker.len + 1;
            }
            _ => {
                part1_size += 1;
                i += 1;
            }
        }
    }

    println!("Part1: Uncompressed text has size {part1_size}");

    let root_marker = Marker {
        sequence: coded,
        len: coded.len(),
        times: 1,
    };

    println!(
        "Part2: Recursively uncompressed text has size {}",
        root_marker.rec_size()
    );
}
