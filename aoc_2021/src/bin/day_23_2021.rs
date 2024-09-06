extern crate core;

use nom::bytes::complete::take;
use nom::character::complete::{anychar, char};
use nom::combinator::map;
use nom::multi::separated_list1;
use nom::sequence::preceded;
use nom::IResult;
use std::cell::OnceCell;
use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::ops::Range;
use std::str::FromStr;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Room {
    id: usize,
    max_size: usize,
    pods: Vec<Amphipod>,
}

impl Room {
    fn from_id_slice(id: usize, s: &[Amphipod]) -> Self {
        Room {
            id,
            max_size: s.len(),
            pods: s.to_vec(),
        }
    }

    fn is_full(&self) -> bool {
        self.pods.len() == self.max_size && self.pods.iter().all(|pod| *pod as usize == self.id)
    }

    fn can_pop(&self) -> bool {
        !self.pods.iter().all(|pod| *pod as usize == self.id)
    }

    fn pop(&mut self) -> (Amphipod, usize) {
        let pod: Amphipod = self.pods.pop().unwrap();
        (pod, self.max_size - self.pods.len())
    }

    fn push(&mut self, pod: Amphipod) -> usize {
        self.pods.push(pod);
        self.max_size - self.pods.len() + 1
    }

    fn heuristic(&self) -> usize {
        //Sum the heuristic energy of the amphipods inside
        //We're starting from the back of the room, to see if the pods present need to be moved or not
        let (_, nb_stored, pods_heuristic): (bool, usize, usize) = self
            .pods
            .iter()
            .enumerate()
            .fold((true, 0, 0), |(is_clean, nb_stored, mut h), (i, p)| {
                if *p as usize == self.id && is_clean {
                    //All pods until here belong in this room, no need to move them
                    (true, nb_stored + 1, h)
                } else {
                    //A pod further back needs to be moving out even if we're in the right room
                    let add_steps = self.max_size - i //Get out of the room
                            + 3 //Reach nearest rooms, including this one
                            + self.id.abs_diff(*p as usize).saturating_sub(1) * 2; //Reach further rooms
                    h += add_steps * p.step_cost();
                    (false, nb_stored, h)
                }
            });
        //Add the energy that amphipods will have to spend to fill the room to the back
        let to_reenter: usize = self.max_size - nb_stored;
        let reenter_heuristic: usize =
            (to_reenter) * (to_reenter.saturating_sub(1)) / 2 * 10usize.pow(self.id as u32);
        pods_heuristic + reenter_heuristic
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
struct AmphiNode {
    center_hall: [Option<Amphipod>; 11],
    rooms: [Room; 4],
    energy: usize,
    heuristic: OnceCell<usize>,
}

impl AmphiNode {
    fn hall_to_room(&self, hall_id: usize, room_id: usize) -> Self {
        let mut center_hall: [Option<Amphipod>; 11] = self.center_hall;
        let mut rooms: [Room; 4] = self.rooms.clone();
        let pod = center_hall[hall_id].unwrap();
        center_hall[hall_id] = None;
        let steps: usize = rooms[room_id].push(pod) + hall_id.abs_diff((room_id + 1) * 2);
        AmphiNode {
            center_hall,
            rooms,
            energy: self.energy + steps * pod.step_cost(),
            heuristic: OnceCell::new(),
        }
    }
    fn room_to_hall(&self, hall_id: usize, room_id: usize) -> Self {
        let mut center_hall: [Option<Amphipod>; 11] = self.center_hall;
        let mut rooms: [Room; 4] = self.rooms.clone();
        let (pod, steps): (Amphipod, usize) = rooms[room_id].pop();
        center_hall[hall_id] = Some(pod);
        let steps = steps + hall_id.abs_diff((room_id + 1) * 2);
        AmphiNode {
            center_hall,
            rooms,
            energy: self.energy + steps * pod.step_cost(),
            heuristic: OnceCell::new(),
        }
    }

    fn is_end(&self) -> bool {
        self.rooms.iter().all(|r| r.is_full())
    }

    fn hall_is_free(&self, hall_id: usize, room_id: usize, inclusive: bool) -> bool {
        fn range(hall_id: usize, room_id: usize, inclusive: bool) -> Range<usize> {
            let room_in_hall: usize = (room_id + 1) * 2;
            if hall_id < room_in_hall {
                hall_id + !inclusive as usize..room_in_hall
            } else {
                room_in_hall + 1..hall_id + inclusive as usize
            }
        }
        self.center_hall[range(hall_id, room_id, inclusive)]
            .iter()
            .all(|p| p.is_none())
    }

    fn neighbours(&self) -> Vec<Self> {
        //If any pod can move from the hall in any room, it's the only neighbour we need

        //First we get the pods that can get out of the room, any None is a room that can be Entered!
        let can_pop: Vec<bool> = self.rooms.iter().map(|r| r.can_pop()).collect();

        for (hall_id, pod_opt) in self.center_hall.iter().enumerate() {
            if let Some(pod) = pod_opt {
                let room_id: usize = *pod as usize;
                //If the pod room is accessible (can enter + way is free) we return that Node as a neighbour
                if !can_pop[room_id] && self.hall_is_free(hall_id, room_id, false) {
                    return vec![self.hall_to_room(hall_id, room_id)];
                }
            }
        }

        //Else for each room, reach all hall spot possible
        let hall_spaces: Vec<usize> = vec![0, 1, 3, 5, 7, 9, 10];
        can_pop
            .into_iter()
            .enumerate()
            .filter(|&(_, can_pop)| can_pop)
            .flat_map(|(room_id, _)| {
                //let (pod, steps) = opt.unwrap();
                hall_spaces
                    .iter()
                    .filter(move |&&hall_id| self.hall_is_free(hall_id, room_id, true))
                    .map(move |&hall_id| self.room_to_hall(hall_id, room_id))
            })
            .collect()
    }

    fn heuristic(&self) -> usize {
        *self.heuristic.get_or_init(|| self.compute_heuristic())
    }

    fn compute_heuristic(&self) -> usize {
        //Heuristic for all pods in the hall
        let hall_h: usize = self
            .center_hall
            .iter()
            .enumerate()
            .map(|(i, o)| {
                o.map(|pod| {
                    let steps: usize = i.abs_diff((pod as usize + 1) * 2) + 1;
                    steps * pod.step_cost()
                })
                .unwrap_or(0)
            })
            .sum();
        //Sum heuristics of each room
        let room_h: usize = self.rooms.iter().map(|r| r.heuristic()).sum();

        hall_h + room_h
    }
}

impl Ord for AmphiNode {
    fn cmp(&self, other: &Self) -> Ordering {
        let other_sum = other.energy + other.heuristic();
        let self_sum = self.energy + self.heuristic();

        other_sum
            .cmp(&self_sum)
            .then(other.energy.cmp(&self.energy))
    }
}

impl PartialOrd for AmphiNode {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Amphipod {
    Amber = 0,
    Bronze = 1,
    Copper = 2,
    Desert = 3,
}

impl Amphipod {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Amphipod::Amber,
            'B' => Amphipod::Bronze,
            'C' => Amphipod::Copper,
            _ => Amphipod::Desert,
        }
    }

    fn step_cost(&self) -> usize {
        10usize.pow(*self as u32)
    }
}

#[derive(Debug, Clone)]
struct AmphipodsBurrow {
    starting_rooms: Vec<Vec<Amphipod>>,
}

impl AmphipodsBurrow {
    fn starting_rooms(&self, extended: bool) -> Vec<Vec<Amphipod>> {
        let mut starting_rooms: Vec<Vec<Amphipod>> = self.starting_rooms.clone();
        if extended {
            //Adding the following pods between the 2 already present by room
            //#D#C#B#A#
            //#D#B#A#C#
            const FOLD: [[Amphipod; 2]; 4] = [
                [Amphipod::Desert, Amphipod::Desert],
                [Amphipod::Copper, Amphipod::Bronze],
                [Amphipod::Bronze, Amphipod::Amber],
                [Amphipod::Amber, Amphipod::Copper],
            ];
            for i in 0..4 {
                starting_rooms[i].rotate_right(1);
                starting_rooms[i].push(FOLD[i][1]);
                starting_rooms[i].push(FOLD[i][0]);
                starting_rooms[i].rotate_left(1)
            }
        }
        starting_rooms
    }

    fn starting_node(&self, extended: bool) -> AmphiNode {
        let center_hall: [Option<Amphipod>; 11] = [None; 11];
        let starting_rooms = self.starting_rooms(extended);
        let rooms: [Room; 4] = [
            Room::from_id_slice(0, &starting_rooms[0]),
            Room::from_id_slice(1, &starting_rooms[1]),
            Room::from_id_slice(2, &starting_rooms[2]),
            Room::from_id_slice(3, &starting_rooms[3]),
        ];
        let energy: usize = 0;
        let heuristic: OnceCell<usize> = OnceCell::new();

        AmphiNode {
            center_hall,
            rooms,
            energy,
            heuristic,
        }
    }
    fn organize(&self, extended: bool) -> usize {
        let mut queue: BinaryHeap<AmphiNode> = BinaryHeap::default();
        queue.push(self.starting_node(extended));

        while let Some(node) = queue.pop() {
            if node.is_end() {
                return node.energy;
            }
            queue.extend(node.neighbours());
        }
        0
    }
}

impl FromStr for AmphipodsBurrow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_row(s: &str) -> IResult<&str, Vec<Amphipod>> {
            let (s, pods) = preceded(
                take(3usize),
                separated_list1(char('#'), map(anychar, Amphipod::from_char)),
            )(s)?;

            Ok((s, pods))
        }

        let rows: Vec<Vec<Amphipod>> = s
            .lines()
            .skip(2)
            .take(2)
            .map(|l| parse_row(l).unwrap().1)
            .collect();
        let mut starting_rooms: Vec<Vec<Amphipod>> = Vec::new();
        for i in 0..4 {
            starting_rooms.push(vec![rows[1][i], rows[0][i]]);
        }
        Ok(AmphipodsBurrow { starting_rooms })
    }
}

fn main() {
    let now = std::time::Instant::now();
    let s = util::file_as_string("aoc_2021/input/day_23.txt").expect("Cannot open input file");
    let burrow: AmphipodsBurrow = s.parse().unwrap();
    println!(
        "Part1: The amphipods need {} energy to rejoin their rooms",
        burrow.organize(false)
    );
    println!(
        "Part2: With the additional amphipods, they need {} energy to rejoin their rooms",
        burrow.organize(true)
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########
";

    #[test]
    fn part_1() {
        let burrow: AmphipodsBurrow = EXAMPLE_1.parse().unwrap();
        assert_eq!(12521, burrow.organize(false));
    }

    #[test]
    fn part_2() {
        let burrow: AmphipodsBurrow = EXAMPLE_1.parse().unwrap();
        assert_eq!(44169, burrow.organize(true));
    }
}
