use std::cmp::min;
use std::collections::HashMap;

const MANA_MAX: u16 = 1500;
const SPELLS: [Spell; 5] = [
    Spell {
        name: SpellType::Missile,
        cost: 53,
        duration: 0,
    },
    Spell {
        name: SpellType::Drain,
        cost: 73,
        duration: 0,
    },
    Spell {
        name: SpellType::Shield,
        cost: 113,
        duration: 6,
    },
    Spell {
        name: SpellType::Poison,
        cost: 173,
        duration: 6,
    },
    Spell {
        name: SpellType::Recharge,
        cost: 229,
        duration: 5,
    },
];

#[derive(Debug, Copy, Clone)]
struct Boss {
    hp: u16,
    damage: u16,
}

impl Boss {
    fn lose(&mut self, damage: u16) {
        let loss = min(self.hp, damage);
        self.hp -= loss;
    }

    fn dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug, Copy, Clone)]
struct Player {
    hp: u16,
    armor: u16,
    mana: u16,
}

impl Player {
    fn lose_life(&mut self, loss: u16) {
        let actual_loss = min(self.hp, loss);
        self.hp -= actual_loss;
    }
    fn attacked(&mut self, damage: u16) {
        let reduced_damage = if self.armor >= damage {
            1
        } else {
            damage - self.armor
        };
        self.lose_life(reduced_damage);
    }

    fn gain_life(&mut self, gain: u16) {
        self.hp += gain
    }

    fn gain_armor(&mut self, armor: u16) {
        self.armor += armor
    }

    fn lose_armor(&mut self, armor: u16) {
        self.armor -= armor
    }

    fn gain_mana(&mut self, mana: u16) {
        self.mana += mana
    }

    fn lose_mana(&mut self, mana: u16) {
        self.mana -= mana
    }

    fn dead(&self) -> bool {
        self.hp == 0
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum SpellType {
    Missile,
    Drain,
    Shield,
    Poison,
    Recharge,
}

#[derive(Debug, Copy, Clone)]
struct Spell {
    name: SpellType,
    cost: u16,
    duration: u16,
}

#[derive(Debug, Clone)]
struct Battlefield {
    player: Player,
    boss: Boss,
    effects: HashMap<SpellType, u16>,
}

impl Battlefield {
    fn apply_effects(&mut self) {
        let mut to_remove: Vec<SpellType> = Vec::new();
        for (&spell, dura) in &mut self.effects {
            match (spell, &dura) {
                (SpellType::Shield, 6) => {
                    self.player.gain_armor(7);
                    *dura -= 1;
                }
                (SpellType::Shield, 1) => {
                    self.player.lose_armor(7);
                    to_remove.push(SpellType::Shield);
                    *dura -= 1;
                }
                (SpellType::Shield, _) => {
                    *dura -= 1;
                }
                (SpellType::Poison, _) => {
                    self.boss.lose(3);
                    if *dura == 1 {
                        to_remove.push(SpellType::Poison);
                    }
                    *dura -= 1;
                }
                (SpellType::Recharge, _) => {
                    self.player.gain_mana(101);
                    if *dura == 1 {
                        to_remove.push(SpellType::Recharge);
                    }
                    *dura -= 1;
                }
                _ => (),
            }
        }

        for key in to_remove {
            self.effects.remove(&key);
        }
    }

    fn player_turn(&mut self, spell: &Spell) {
        self.player.lose_mana(spell.cost);
        match spell.name {
            SpellType::Missile => {
                self.boss.lose(4);
            }
            SpellType::Drain => {
                self.boss.lose(2);
                self.player.gain_life(2);
            }
            SpellType::Shield => {
                self.effects.insert(spell.name, spell.duration);
            }
            SpellType::Poison => {
                self.effects.insert(spell.name, spell.duration);
            }
            SpellType::Recharge => {
                self.effects.insert(spell.name, spell.duration);
            }
        }
    }

    fn boss_turn(&mut self) {
        self.player.attacked(self.boss.damage);
    }

    fn a_turn(&mut self, spell: &Spell, hard: bool) -> Option<u16> {
        let cost = spell.cost;
        if self.player.mana < cost {
            return None;
        }
        self.player_turn(spell);
        if self.boss.dead() {
            return Some(spell.cost);
        }
        if hard {
            self.player.lose_life(1);
            if self.player.dead() {
                return None;
            }
        }
        self.apply_effects();
        if self.boss.dead() {
            return Some(spell.cost);
        }
        self.boss_turn();
        if self.player.dead() {
            return None;
        }
        if hard {
            self.player.lose_life(1);
            if self.player.dead() {
                return None;
            }
        }
        self.apply_effects();
        Some(spell.cost)
    }
}

fn main() {
    let boss = Boss { hp: 58, damage: 9 };
    let player = Player {
        hp: 50,
        armor: 0,
        mana: 500,
    };
    let battlefield = Battlefield {
        player,
        boss,
        effects: HashMap::new(),
    };

    if let Some(res) = game_loop(battlefield.clone(), false, Vec::new(), 0) {
        println!(
            "Part1: We need to use at least {} mana to cast our spells and win: {:?}",
            res.1, res.0
        );
    } else {
        println!("Part1: No solution found!")
    }

    if let Some(res) = game_loop(battlefield, true, Vec::new(), 0) {
        println!(
            "Part2: We now need to use at least {} mana to cast our spells and win: {:?}",
            res.1, res.0
        );
    } else {
        println!("Part2: No solution found!")
    }
}

fn game_loop(
    battlefield: Battlefield,
    hard: bool,
    spells_cast: Vec<SpellType>,
    mana_used: u16,
) -> Option<(Vec<SpellType>, u16)> {
    if battlefield.boss.dead() {
        return Some((spells_cast, mana_used));
    }
    if mana_used > MANA_MAX {
        return None;
    }
    SPELLS
        .iter()
        .filter(|&sp| !battlefield.effects.contains_key(&sp.name))
        .flat_map(|&sp| {
            let mut bf = battlefield.clone();
            bf.a_turn(&sp, hard).and_then(|mana| {
                let mut sps = spells_cast.clone();
                sps.push(sp.name);
                game_loop(bf, hard, sps, mana_used + mana)
            })
        })
        .min_by(|a, b| a.1.cmp(&b.1))
}
