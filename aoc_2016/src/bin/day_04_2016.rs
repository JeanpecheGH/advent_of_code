use std::cmp::Ordering;
use std::collections::HashMap;

#[derive(Debug)]
struct Room {
    letters: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn valid(&self) -> bool {
        let map: HashMap<char, u32> = self.letters.chars().fold(HashMap::new(), |mut acc, c| {
            *acc.entry(c).or_insert(0) += 1;
            acc
        });

        let mut pairs: Vec<(char, u32)> = map.iter().map(|(&c, &i)| (c, i)).collect();
        pairs.sort_by(|&(ca, ia), &(cb, ib)| match ia.cmp(&ib) {
            Ordering::Equal => ca.cmp(&cb),
            Ordering::Greater => Ordering::Less,
            Ordering::Less => Ordering::Greater,
        });

        let slice: &[(char, u32)] = &pairs[..5];
        let check: String = slice.iter().map(|pair| pair.0).collect();
        check.eq(&self.checksum)
    }

    fn decrypt(&self) -> String {
        let a = 'a' as u32;
        let bytes: Vec<u8> = self.letters.bytes().collect();
        let rotated: String = bytes
            .iter()
            .map(|&b| char::from_u32((b as u32 - a + self.id) % 26 + a).unwrap())
            .collect();
        format!("{}-{}", rotated, self.id)
    }
}

fn main() {
    let lines = util::file_as_lines("aoc_2016/input/day_04.txt").expect("Cannot open input file");

    let rooms: Vec<Room> = lines
        .map(|l| {
            let s = l.unwrap();
            let words: Vec<&str> = s.split('-').collect();
            let mut letters: String = "".to_string();
            for word in words.iter().take(words.len() - 1) {
                letters.push_str(word)
            }
            let split_end: Vec<&str> = words[words.len() - 1].split('[').collect();
            let id: u32 = split_end[0].parse().unwrap();
            let checksum = split_end[1].strip_suffix(']').unwrap().to_string();
            Room {
                letters,
                id,
                checksum,
            }
        })
        .collect();

    let id_sum: u32 = rooms
        .iter()
        .filter_map(|room| if room.valid() { Some(room.id) } else { None })
        .sum();

    println!("Part1: The sum of the IDs of valid Rooms is {id_sum}");

    let north_pole_rooms: Vec<String> = rooms
        .iter()
        .filter_map(|room| {
            let room_name = room.decrypt();
            if room_name.contains("northpole") {
                Some(room_name)
            } else {
                None
            }
        })
        .collect();

    println!(
        "Part2: Rooms containing North Pole items {:?}",
        north_pole_rooms
    );
}
