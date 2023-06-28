struct Triangle {
    sides: [u16; 3],
}

impl Triangle {
    fn from_slice(slice: &[u16]) -> Self {
        let sides: [u16; 3] = [slice[0], slice[1], slice[2]];
        Triangle { sides }
    }

    fn from_sides(a: u16, b: u16, c: u16) -> Self {
        let sides: [u16; 3] = [a, b, c];
        Triangle { sides }
    }

    fn valid(&mut self) -> bool {
        self.sides.sort_unstable();
        self.sides[0] + self.sides[1] > self.sides[2]
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_03.txt").expect("Cannot open input file");

    let numbers: Vec<Vec<u16>> = s
        .lines()
        .map(|s| {
            let chars: Vec<char> = s.chars().collect();
            chars
                .chunks(5)
                .map(|cs| {
                    cs.iter()
                        .filter(|&&c| c != ' ')
                        .collect::<String>()
                        .parse::<u16>()
                        .unwrap()
                })
                .collect()
        })
        .collect();

    let nb_triangles = numbers
        .iter()
        .filter_map(|sl| {
            let mut t = Triangle::from_slice(sl);
            if t.valid() {
                Some(t)
            } else {
                None
            }
        })
        .count();

    println!("Part1: There are {nb_triangles} valid triangles");

    let new_nb_triangles = numbers
        .chunks(3)
        .flat_map(|chunk| {
            let mut first: Triangle = Triangle::from_sides(chunk[0][0], chunk[1][0], chunk[2][0]);
            let mut mid: Triangle = Triangle::from_sides(chunk[0][1], chunk[1][1], chunk[2][1]);
            let mut last: Triangle = Triangle::from_sides(chunk[0][2], chunk[1][2], chunk[2][2]);
            let mut v: Vec<Triangle> = Vec::new();
            if first.valid() {
                v.push(first)
            }
            if mid.valid() {
                v.push(mid)
            }
            if last.valid() {
                v.push(last)
            }
            v
        })
        .count();

    println!("Part2: There are now {new_nb_triangles} valid triangles");
}
