use std::collections::VecDeque;
use util::coord::PosI;
use util::intcode::IntCode;

#[derive(Copy, Clone)]
struct Packet {
    target: usize,
    data: PosI,
}
#[derive(Clone)]
struct Computer {
    intcode: IntCode,
}

impl Computer {
    fn new(intcode: IntCode) -> Computer {
        Computer { intcode }
    }

    fn start(&mut self, address: isize) {
        self.intcode.set_inifinite();
        self.intcode.add_input(vec![address]);
    }

    fn receive_data(&mut self, data: PosI) {
        self.intcode.add_input(vec![data.0, data.1]);
    }

    fn read_packet(&mut self) -> Option<Packet> {
        if self.intcode.output.len() == 3 {
            let y: isize = self.intcode.output.pop().unwrap();
            let x: isize = self.intcode.output.pop().unwrap();
            let target: usize = self.intcode.output.pop().unwrap() as usize;
            Some(Packet {
                target,
                data: PosI(x, y),
            })
        } else {
            None
        }
    }

    fn compute(&mut self) {
        self.intcode.compute_one();
    }
}

struct Nat {
    first: Option<isize>,
    packet: Option<Packet>,
    last_y: Option<isize>,
    finished: bool,
}

impl Nat {
    fn new() -> Nat {
        Nat {
            first: None,
            packet: None,
            last_y: None,
            finished: false,
        }
    }

    fn store_packet(&mut self, p: Packet) {
        if self.first.is_none() {
            self.first = Some(p.data.1);
        }
        self.packet = Some(p);
    }

    fn get_packet(&mut self) -> Packet {
        match (self.packet, self.last_y) {
            (Some(p), Some(i)) if p.data.1 == i => {
                self.finished = true;
                p
            }
            (Some(p), _) => {
                self.last_y = Some(p.data.1);
                p
            }
            _ => panic!("Cannot send non existent packet"),
        }
    }
}

struct Network {
    computers: Vec<Computer>,
    packets: Vec<VecDeque<Packet>>,
    nat: Nat,
}

impl Network {
    fn new(intcode: IntCode, n: usize) -> Network {
        let mut computers: Vec<Computer> = Vec::new();
        let mut packets: Vec<VecDeque<Packet>> = Vec::new();
        let c: Computer = Computer::new(intcode);
        for i in 0..n {
            let mut this_computer = c.clone();
            this_computer.start(i as isize);
            computers.push(this_computer);
            packets.push(VecDeque::new());
        }

        Network {
            computers,
            packets,
            nat: Nat::new(),
        }
    }

    fn compute_loop(&mut self) {
        let mut nb_idle = 0;
        while !self.nat.finished {
            //Arbitrary large number of idle cycles
            if nb_idle == 700 {
                let p: Packet = self.nat.get_packet();
                self.packets[0].push_front(p);
                nb_idle = 0;
            }
            let idle: bool = self.compute_all();
            if idle {
                nb_idle += 1;
            } else {
                nb_idle = 0;
            }
        }
    }

    fn compute_all(&mut self) -> bool {
        let mut idle: bool = true;
        for (n, c) in self.computers.iter_mut().enumerate() {
            while let Some(p) = self.packets[n].pop_back() {
                idle = false;
                c.receive_data(p.data)
            }
            c.compute();
            if let Some(p) = c.read_packet() {
                idle = false;
                if p.target == 255 {
                    self.nat.store_packet(p);
                } else {
                    self.packets[p.target].push_front(p);
                }
            }
        }
        idle
    }

    fn answers(&self) -> (isize, isize) {
        (self.nat.first.unwrap(), self.nat.last_y.unwrap())
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2019/input/day_23.txt").expect("Cannot open input file");
    let intcode: IntCode = s.lines().next().unwrap().parse().unwrap();

    let mut network: Network = Network::new(intcode, 50);
    network.compute_loop();
    let (first_y, last_y) = network.answers();

    println!("Part1: The first Y value sent to the NAT is {first_y}");
    println!("Part2: The first Y value to be sent twice in a row from NAT to IP 0 is {last_y}");
    println!("Computing time: {:?}", now.elapsed());
}
