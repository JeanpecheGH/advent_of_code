struct IPv7 {
    supernets: Vec<String>,
    hypernets: Vec<String>,
}

impl IPv7 {
    fn support_tls(&self) -> bool {
        self.supernets.iter().any(|spn| IPv7::has_abba(spn))
            && self.hypernets.iter().all(|hpn| !IPv7::has_abba(hpn))
    }

    fn support_ssl(&self) -> bool {
        let supernet_abas = IPv7::get_abas(&self.supernets);
        let hypernet_abas = IPv7::get_abas(&self.hypernets);

        supernet_abas
            .iter()
            .any(|&spn| hypernet_abas.iter().any(|&hpn| spn[..2] == hpn[1..]))
    }

    fn has_abba(net: &str) -> bool {
        let chars: Vec<char> = net.chars().collect();
        chars
            .windows(4)
            .any(|win| win[0] == win[3] && win[1] == win[2] && win[0] != win[1])
    }

    fn get_abas(sequences: &[String]) -> Vec<&str> {
        sequences
            .iter()
            .flat_map(|sqn| {
                sqn.as_bytes()
                    .windows(3)
                    .enumerate()
                    .filter_map(|(index, win)| {
                        //We compare u8, but we return &str
                        if win[0] == win[2] && win[0] != win[1] {
                            Some(&sqn[index..=index + 2])
                        } else {
                            None
                        }
                    })
            })
            .collect()
    }
}

fn main() {
    let s = util::file_as_string("aoc_2016/input/day_07.txt").expect("Cannot open input file");

    let ips: Vec<IPv7> = s
        .lines()
        .map(|s| {
            let words: Vec<&str> = s.split(['[', ']']).collect();
            let mut supernets: Vec<String> = Vec::new();
            let mut hypernets: Vec<String> = Vec::new();
            for (index, &sequence) in words.iter().enumerate() {
                if index % 2 == 0 {
                    supernets.push(sequence.to_string());
                } else {
                    hypernets.push(sequence.to_string());
                }
            }
            IPv7 {
                supernets,
                hypernets,
            }
        })
        .collect();

    let nb_tls: usize = ips.iter().filter(|&ip| ip.support_tls()).count();

    println!("Part1: {nb_tls} ips support TLS");

    let nb_ssl: usize = ips.iter().filter(|&ip| ip.support_ssl()).count();

    println!("Part1: {nb_ssl} ips support SSL");
}
