struct Fighter {
    hp: u16,
    damage: u16,
    armor: u16,
}

#[derive(Clone, Copy, Debug)]
struct Equipment {
    weapon: Item,
    armor: Item,
    left_ring: Item,
    right_ring: Item,
}

impl Equipment {
    fn price(&self) -> u16 {
        self.weapon.price + self.armor.price + self.left_ring.price + self.right_ring.price
    }
    fn damage(&self) -> u16 {
        self.weapon.damage + self.armor.damage + self.left_ring.damage + self.right_ring.damage
    }
    fn armor(&self) -> u16 {
        self.weapon.armor + self.armor.armor + self.left_ring.armor + self.right_ring.armor
    }
}

#[derive(Clone, Copy, Debug)]
struct Item {
    price: u16,
    damage: u16,
    armor: u16,
}

fn main() {
    let boss = Fighter {
        hp: 104,
        damage: 8,
        armor: 1,
    };
    let player_hp: u16 = 100;

    let weapons: [Item; 5] = [
        Item {
            price: 8,
            damage: 4,
            armor: 0,
        },
        Item {
            price: 10,
            damage: 5,
            armor: 0,
        },
        Item {
            price: 25,
            damage: 6,
            armor: 0,
        },
        Item {
            price: 40,
            damage: 7,
            armor: 0,
        },
        Item {
            price: 74,
            damage: 8,
            armor: 0,
        },
    ];

    let armors: [Item; 6] = [
        Item {
            price: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            price: 13,
            damage: 0,
            armor: 1,
        },
        Item {
            price: 31,
            damage: 0,
            armor: 2,
        },
        Item {
            price: 53,
            damage: 0,
            armor: 3,
        },
        Item {
            price: 75,
            damage: 0,
            armor: 4,
        },
        Item {
            price: 102,
            damage: 0,
            armor: 5,
        },
    ];

    let rings: [Item; 7] = [
        Item {
            price: 0,
            damage: 0,
            armor: 0,
        },
        Item {
            price: 25,
            damage: 1,
            armor: 0,
        },
        Item {
            price: 50,
            damage: 2,
            armor: 0,
        },
        Item {
            price: 100,
            damage: 3,
            armor: 0,
        },
        Item {
            price: 20,
            damage: 0,
            armor: 1,
        },
        Item {
            price: 40,
            damage: 0,
            armor: 2,
        },
        Item {
            price: 80,
            damage: 0,
            armor: 3,
        },
    ];

    let mut cheapest: Option<Equipment> = None;
    let mut expensive: Option<Equipment> = None;

    for damage in 4..=13_u16 {
        let armor_range = match damage {
            11..=13 => 0..6_u16,
            _ => 0..boss.damage,
        };
        for armor in armor_range {
            let player = Fighter {
                hp: player_hp,
                damage,
                armor,
            };

            let equipments: Vec<Equipment> =
                valid_equipments(&weapons, &armors, &rings, damage, armor);
            if win_fight(&player, &boss) {
                cheapest = match (cheapest, pick_equipment(equipments, true)) {
                    (None, Some(e)) => Some(e),
                    (Some(a), Some(b)) if a.price() > b.price() => {
                        println!("Replace cheapest equipment with {b:?}");
                        Some(b)
                    }
                    _ => cheapest,
                };
            } else {
                expensive = match (expensive, pick_equipment(equipments, false)) {
                    (None, Some(e)) => Some(e),
                    (Some(a), Some(b)) if a.price() < b.price() => {
                        println!("Replace most expensive equipment with {b:?}");
                        Some(b)
                    }
                    _ => expensive,
                };
            }
        }
    }

    println!(
        "\nPart1: The cheapest winning equipment is costing {}: {:?}",
        cheapest.unwrap().price(),
        cheapest.unwrap()
    );
    println!(
        "\nPart2: The most expensive losing equipment is costing {}: {expensive:?}",
        expensive.unwrap().price(),
    );
}

fn win_fight(player: &Fighter, boss: &Fighter) -> bool {
    let damage_dealt = player.damage - boss.armor;
    let damage_received = boss.damage - player.armor;

    let turns_to_kill = if boss.hp.is_multiple_of(damage_dealt) {
        boss.hp / damage_dealt
    } else {
        boss.hp / damage_dealt + 1
    };
    let turns_to_die = if player.hp.is_multiple_of(damage_received) {
        player.hp / damage_received
    } else {
        player.hp / damage_received + 1
    };

    turns_to_kill <= turns_to_die
}

fn valid_equipments(
    weapons: &[Item],
    armors: &[Item],
    rings: &[Item],
    target_damage: u16,
    target_armor: u16,
) -> Vec<Equipment> {
    let mut valid_equipment: Vec<Equipment> = Vec::new();
    for &weapon in weapons {
        for &armor in armors {
            for &left_ring in rings {
                for &right_ring in rings {
                    if left_ring.price != right_ring.price || left_ring.price == 0 {
                        let equip = Equipment {
                            weapon,
                            armor,
                            left_ring,
                            right_ring,
                        };
                        if equip.damage() == target_damage && equip.armor() == target_armor {
                            valid_equipment.push(equip);
                        }
                    }
                }
            }
        }
    }
    valid_equipment
}

fn pick_equipment(mut equipments: Vec<Equipment>, cheapest: bool) -> Option<Equipment> {
    equipments.sort_by(|&a, &b| a.price().cmp(&b.price()));
    if cheapest {
        equipments.first().copied()
    } else {
        equipments.last().copied()
    }
}
