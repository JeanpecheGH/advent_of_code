use itertools::Itertools;
use std::collections::HashSet;

const NB_CHIP_PART1: usize = 10;
const NB_CHIP_PART2: usize = 14;

#[derive(Eq, PartialEq, Hash, Copy, Clone, Debug)]
struct Building<const N: usize> {
    elevator: u8,
    items: [u8; N],
}

macro_rules! building {
    ($p:expr) => {
        impl Building<$p> {
            fn is_valid(&self) -> bool {
                let half = self.items.len() / 2;
                self.items[..half].iter().enumerate().all(|(i, &chip)| {
                    chip == self.items[half + i]
                        || self.items[half..].iter().all(|&gen| gen != chip)
                })
            }

            fn moves(&self) -> Vec<Self> {
                self.up_and_down()
                    .into_iter()
                    .flat_map(|i| {
                        let items_to_move = self.same_floor();
                        items_to_move
                            .iter()
                            .map(|&item_index| {
                                let mut items = self.items;
                                items[item_index] = i;
                                Self { elevator: i, items }
                            })
                            .filter(|b| b.is_valid())
                            .chain(
                                items_to_move
                                    .clone()
                                    .into_iter()
                                    .combinations(2)
                                    .map(|combi| {
                                        let mut items = self.items;
                                        items[combi[0]] = i;
                                        items[combi[1]] = i;
                                        Self { elevator: i, items }
                                    })
                                    .filter(|b| b.is_valid()),
                            )
                            .collect::<Vec<Self>>()
                    })
                    .collect()
            }

            fn up_and_down(&self) -> Vec<u8> {
                match self.elevator {
                    0 => vec![1],
                    1 | 2 => vec![self.elevator - 1, self.elevator + 1],
                    3 => vec![2],
                    _ => Vec::new(),
                }
            }

            fn same_floor(&self) -> Vec<usize> {
                self.items
                    .iter()
                    .enumerate()
                    .filter_map(|(i, &floor)| {
                        if self.elevator == floor {
                            Some(i)
                        } else {
                            None
                        }
                    })
                    .collect()
            }
        }
    };
}

building!(NB_CHIP_PART1);
building!(NB_CHIP_PART2);

fn main() {
    // The first floor contains a thulium generator, a thulium-compatible microchip, a plutonium generator, and a strontium generator.
    // The second floor contains a plutonium-compatible microchip and a strontium-compatible microchip.
    // The third floor contains a promethium generator, a promethium-compatible microchip, a ruthenium generator, and a ruthenium-compatible microchip.
    // The fourth floor contains nothing relevant.

    //Part1: Chips will be : [thulium, plutonium, strontium, promethium, ruthenium]
    let target_building = Building {
        elevator: 0,
        items: [0, 1, 1, 2, 2, 0, 0, 0, 2, 2],
    };

    let end_building = Building {
        elevator: 3,
        items: [3; NB_CHIP_PART1],
    };

    let mut building_set: HashSet<Building<NB_CHIP_PART1>> = HashSet::new();
    building_set.insert(end_building);

    let now = std::time::Instant::now();

    let mut to_compute = vec![end_building];
    let mut found = false;
    let mut step = 0;
    loop {
        println!(
            "Step: {}, Number of total positions computed: {}, Number of positions for this step: {}",
            step,
            building_set.len(),
            to_compute.len(),
        );
        to_compute = to_compute
            .into_iter()
            .flat_map(|b| b.moves())
            .filter(|&b| {
                if building_set.insert(b) {
                    if b == target_building {
                        println!("Part1: Target building found at distance {}", step + 1);
                        found = true;
                    }
                    true
                } else {
                    false
                }
            })
            .collect();
        if found {
            break;
        }
        step += 1;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);

    //Part2: Added at first floor : An elerium generator, an elerium-compatible microchip, a dilithium generator, a dilithium-compatible microchip.
    //Chips will be : [elerium, dilithium, thulium, plutonium, strontium, promethium, ruthenium]
    let target_building = Building {
        elevator: 0,
        items: [0, 0, 0, 1, 1, 2, 2, 0, 0, 0, 0, 0, 2, 2],
    };

    let end_building = Building {
        elevator: 3,
        items: [3; NB_CHIP_PART2],
    };

    let mut building_set: HashSet<Building<NB_CHIP_PART2>> = HashSet::new();
    building_set.insert(end_building);

    let now = std::time::Instant::now();

    let mut to_compute = vec![end_building];
    let mut found = false;
    let mut step = 0;
    loop {
        println!(
            "Step: {}, Number of total positions computed: {}, Number of positions for this step: {}",
            step,
            building_set.len(),
            to_compute.len(),
        );
        to_compute = to_compute
            .into_iter()
            .flat_map(|b| b.moves())
            .filter(|&b| {
                if building_set.insert(b) {
                    if b == target_building {
                        println!("Part2: Target building found at distance {}", step + 1);
                        found = true;
                    }
                    true
                } else {
                    false
                }
            })
            .collect();
        if found {
            break;
        }
        step += 1;
    }
    let elapsed = now.elapsed();
    println!("Elapsed: {:.2?}", elapsed);
}
