use std::str::FromStr;

#[derive(Debug, Clone)]
struct BitsPacket {
    version: usize,
    id: usize,
    sub_packets: Vec<BitsPacket>,
    value: Option<usize>,
}

impl BitsPacket {
    fn version_sum(&self) -> usize {
        self.sub_packets
            .iter()
            .map(|p| p.version_sum())
            .sum::<usize>()
            + self.version
    }

    fn evaluate(&self) -> usize {
        match self.id {
            0 => self.sub_packets.iter().map(|p| p.evaluate()).sum(),
            1 => self.sub_packets.iter().map(|p| p.evaluate()).product(),
            2 => self.sub_packets.iter().map(|p| p.evaluate()).min().unwrap(),
            3 => self.sub_packets.iter().map(|p| p.evaluate()).max().unwrap(),
            4 => self.value.unwrap(),
            5 => (self.sub_packets[0].evaluate() > self.sub_packets[1].evaluate()) as usize,
            6 => (self.sub_packets[0].evaluate() < self.sub_packets[1].evaluate()) as usize,
            7 => (self.sub_packets[0].evaluate() == self.sub_packets[1].evaluate()) as usize,
            _ => 0,
        }
    }
}

impl FromStr for BitsPacket {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn to_binary(c: char) -> &'static str {
            match c {
                '0' => "0000",
                '1' => "0001",
                '2' => "0010",
                '3' => "0011",
                '4' => "0100",
                '5' => "0101",
                '6' => "0110",
                '7' => "0111",
                '8' => "1000",
                '9' => "1001",
                'A' => "1010",
                'B' => "1011",
                'C' => "1100",
                'D' => "1101",
                'E' => "1110",
                'F' => "1111",
                _ => "",
            }
        }

        fn to_usize(s: &str) -> usize {
            s.chars()
                .fold(0, |acc, c| if c == '1' { acc * 2 + 1 } else { acc * 2 })
        }

        fn parse_packet(s: &str) -> (&str, BitsPacket) {
            let version: usize = to_usize(&s[0..3]);
            let id: usize = to_usize(&s[3..6]);

            let mut rest: &str = &s[6..];
            let (sub_packets, value): (Vec<BitsPacket>, Option<usize>) = match id {
                4 => {
                    let mut v: usize = 0;
                    loop {
                        let stop_parsing: bool = &rest[0..1] == "0";
                        let chunk: usize = to_usize(&rest[1..5]);
                        rest = &rest[5..];
                        v = v * 16 + chunk;
                        if stop_parsing {
                            break;
                        }
                    }
                    (Vec::new(), Some(v))
                }
                _ => {
                    let length_id: usize = to_usize(&rest[0..1]);
                    let mut vec: Vec<BitsPacket> = Vec::new();
                    match length_id {
                        0 => {
                            let length: usize = to_usize(&rest[1..16]);
                            rest = &rest[16..];
                            let target: &str = &rest[length..];
                            while rest != target {
                                let (sub_rest, sub_packet) = parse_packet(rest);
                                rest = sub_rest;
                                vec.push(sub_packet)
                            }
                        }
                        _ => {
                            let nb_packets: usize = to_usize(&rest[1..12]);
                            rest = &rest[12..];
                            for _ in 0..nb_packets {
                                let (sub_rest, sub_packet) = parse_packet(rest);
                                rest = sub_rest;
                                vec.push(sub_packet)
                            }
                        }
                    }

                    (vec, None)
                }
            };

            (
                rest,
                BitsPacket {
                    version,
                    id,
                    sub_packets,
                    value,
                },
            )
        }

        let bits: String = s.chars().map(to_binary).collect();
        Ok(parse_packet(&bits).1)
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_16.txt").expect("Cannot open input file");
    let packet: BitsPacket = s.parse().unwrap();
    println!(
        "Part1: The sum of all version number is {}",
        packet.version_sum()
    );
    println!(
        "Part2: The BITS transmission evaluates to {}",
        packet.evaluate()
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_test_1() {
        let packet: BitsPacket = "D2FE28".parse().unwrap();
        assert_eq!(6, packet.version_sum());
        assert_eq!(2021, packet.evaluate());
    }

    #[test]
    fn part_1_test_2() {
        let packet: BitsPacket = "8A004A801A8002F478".parse().unwrap();
        assert_eq!(16, packet.version_sum());
    }

    #[test]
    fn part_1_test_3() {
        let packet: BitsPacket = "620080001611562C8802118E34".parse().unwrap();
        assert_eq!(12, packet.version_sum());
    }

    #[test]
    fn part_1_test_4() {
        let packet: BitsPacket = "C0015000016115A2E0802F182340".parse().unwrap();
        assert_eq!(23, packet.version_sum());
    }

    #[test]
    fn part_1_test_5() {
        let packet: BitsPacket = "A0016C880162017C3686B18A3D4780".parse().unwrap();
        assert_eq!(31, packet.version_sum());
    }

    #[test]
    fn part_2_test_1() {
        let packet: BitsPacket = "C200B40A82".parse().unwrap();
        assert_eq!(3, packet.evaluate());
    }
    #[test]
    fn part_2_test_2() {
        let packet: BitsPacket = "04005AC33890".parse().unwrap();
        assert_eq!(54, packet.evaluate());
    }
    #[test]
    fn part_2_test_3() {
        let packet: BitsPacket = "880086C3E88112".parse().unwrap();
        assert_eq!(7, packet.evaluate());
    }
    #[test]
    fn part_2_test_4() {
        let packet: BitsPacket = "CE00C43D881120".parse().unwrap();
        assert_eq!(9, packet.evaluate());
    }
    #[test]
    fn part_2_test_5() {
        let packet: BitsPacket = "D8005AC2A8F0".parse().unwrap();
        assert_eq!(1, packet.evaluate());
    }
    #[test]
    fn part_2_test_6() {
        let packet: BitsPacket = "F600BC2D8F".parse().unwrap();
        assert_eq!(0, packet.evaluate());
    }
    #[test]
    fn part_2_test_7() {
        let packet: BitsPacket = "9C005AC2F8F0".parse().unwrap();
        assert_eq!(0, packet.evaluate());
    }
    #[test]
    fn part_2_test_8() {
        let packet: BitsPacket = "9C0141080250320F1802104A08".parse().unwrap();
        assert_eq!(1, packet.evaluate());
    }
}
