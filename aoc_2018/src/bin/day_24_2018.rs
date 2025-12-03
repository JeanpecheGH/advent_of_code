use fxhash::FxHashMap;
use itertools::Itertools;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, char};
use nom::combinator::{map, opt};
use nom::multi::separated_list1;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::Parser;
use nom_permutation::permutation_opt;
use std::cmp::Ordering;
use std::str::FromStr;
use util::basic_parser::parse_usize;
use util::split_blocks;

#[derive(Debug, Clone, Eq, PartialEq)]
struct UnitGroup {
    nb_units: usize,
    hit_points: usize,
    immunities: Vec<String>,
    weaknesses: Vec<String>,
    attack: usize,
    boost_attack: usize,
    attack_type: String,
    initiative: usize,
}

impl UnitGroup {
    fn set_boost(&mut self, boost: usize) {
        self.boost_attack = boost;
    }
    fn effective_power(&self) -> usize {
        self.nb_units * (self.attack + self.boost_attack)
    }

    fn attack_dmg(&self, attacker: &UnitGroup) -> usize {
        if !self.immunities.contains(&attacker.attack_type) {
            let base_dmg: usize = attacker.effective_power();
            if self.weaknesses.contains(&attacker.attack_type) {
                base_dmg * 2
            } else {
                base_dmg
            }
        } else {
            0
        }
    }

    fn get_attacked(&mut self, attacker: &UnitGroup) {
        let dmg: usize = self.attack_dmg(attacker);
        let kills: usize = dmg / self.hit_points;
        if kills > self.nb_units {
            self.nb_units = 0;
        } else {
            self.nb_units -= kills;
        }
    }
}

impl Ord for UnitGroup {
    fn cmp(&self, other: &Self) -> Ordering {
        self.effective_power()
            .cmp(&other.effective_power())
            .then(self.initiative.cmp(&other.initiative))
    }
}

impl PartialOrd for UnitGroup {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for UnitGroup {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_weaknesses(s: &str) -> IResult<&str, Vec<String>> {
            preceded(
                tag("weak to "),
                separated_list1(tag(", "), map(alpha1, |str: &str| str.to_string())),
            )
            .parse(s)
        }

        fn parse_immunities(s: &str) -> IResult<&str, Vec<String>> {
            preceded(
                tag("immune to "),
                separated_list1(tag(", "), map(alpha1, |str: &str| str.to_string())),
            )
            .parse(s)
        }

        fn parse_group(s: &str) -> IResult<&str, UnitGroup> {
            let (s, nb_units) = parse_usize(s)?;
            let (s, _) = tag(" units each with ")(s)?;
            let (s, hit_points) = parse_usize(s)?;
            let (s, _) = tag(" hit points ")(s)?;
            let (s, opt_tuple) = opt(delimited(
                char('('),
                permutation_opt((parse_immunities, tag("; "), parse_weaknesses)),
                tag(") "),
            ))
            .parse(s)?;
            let (immunities, weaknesses): (Vec<String>, Vec<String>) =
                if let Some((i, _, w)) = opt_tuple {
                    (i.unwrap_or(Vec::new()), w.unwrap_or(Vec::new()))
                } else {
                    (Vec::new(), Vec::new())
                };
            let (s, _) = tag("with an attack that does ")(s)?;
            let (s, attack) = parse_usize(s)?;
            let (s, attack_type) =
                map(preceded(char(' '), alpha1), |str: &str| str.to_string()).parse(s)?;
            let (s, _) = tag(" damage at initiative ")(s)?;
            let (s, initiative) = parse_usize(s)?;

            let group = UnitGroup {
                nb_units,
                hit_points,
                immunities,
                weaknesses,
                attack,
                boost_attack: 0,
                attack_type,
                initiative,
            };
            Ok((s, group))
        }

        Ok(parse_group(s).unwrap().1)
    }
}

#[derive(Debug, Clone)]
struct ImmuneSystem {
    defense: Vec<UnitGroup>,
    infection: Vec<UnitGroup>,
}

impl ImmuneSystem {
    fn target_selection(&self) -> (FxHashMap<usize, usize>, FxHashMap<usize, usize>) {
        fn inner_target_selection(
            source: &[UnitGroup],
            target: &[UnitGroup],
        ) -> FxHashMap<usize, usize> {
            // In decreasing order of effective power, groups choose their targets
            // In a tie, the group with the higher initiative chooses first
            let ord_source = source
                .iter()
                .enumerate()
                .sorted_by(|(_, a), (_, b)| b.cmp(a));
            let mut ranked_targets: Vec<(usize, &UnitGroup)> = target.iter().enumerate().collect();

            let mut attack_map: FxHashMap<usize, usize> = FxHashMap::default();
            for (rank, attacker) in ord_source {
                if !ranked_targets.is_empty() {
                    ranked_targets.sort_by(|(_, a), (_, b)| {
                        a.attack_dmg(attacker)
                            .cmp(&b.attack_dmg(attacker))
                            .then(a.cmp(b))
                    });
                    let (target_rank, possible_target) = ranked_targets.pop().unwrap();
                    // If it cannot deal any defending groups damage, it does not choose a target
                    if possible_target.attack_dmg(attacker) == 0 {
                        ranked_targets.push((target_rank, possible_target));
                    } else {
                        attack_map.insert(rank, target_rank);
                    }
                }
            }
            attack_map
        }

        (
            inner_target_selection(&self.defense, &self.infection),
            inner_target_selection(&self.infection, &self.defense),
        )
    }

    fn attacking(
        &mut self,
        defense_attacks: &FxHashMap<usize, usize>,
        infection_attacks: &FxHashMap<usize, usize>,
    ) {
        let mut all_groups: Vec<(bool, usize, usize)> = self
            .defense
            .iter()
            .enumerate()
            .map(|(rank, group)| (true, rank, group.initiative))
            .collect::<Vec<(bool, usize, usize)>>();
        all_groups.extend(
            self.infection
                .iter()
                .enumerate()
                .map(|(rank, group)| (false, rank, group.initiative)),
        );

        all_groups.sort_by(|(_, _, a), (_, _, b)| b.cmp(a));

        for (is_defense, rank, _) in all_groups.into_iter() {
            if is_defense {
                if let Some(&target_rank) = defense_attacks.get(&rank) {
                    let attacking_group: &UnitGroup = &self.defense[rank];
                    self.infection[target_rank].get_attacked(attacking_group);
                }
            } else if let Some(&target_rank) = infection_attacks.get(&rank) {
                let attacking_group: &UnitGroup = &self.infection[rank];
                self.defense[target_rank].get_attacked(attacking_group);
            }
        }
    }

    fn round_fight(&mut self) {
        //Target selection
        let (defense_attacks, infection_attacks) = self.target_selection();

        //Attack
        self.attacking(&defense_attacks, &infection_attacks);

        //Cleanup empty groups
        self.defense.retain(|g| g.nb_units > 0);
        self.infection.retain(|g| g.nb_units > 0);
    }

    fn fight(&mut self) -> (usize, usize) {
        let mut nb_def: usize = self.defense.iter().map(|g| g.nb_units).sum();
        let mut nb_inf: usize = self.infection.iter().map(|g| g.nb_units).sum();
        while !self.defense.is_empty() && !self.infection.is_empty() {
            self.round_fight();
            let new_nb_def: usize = self.defense.iter().map(|g| g.nb_units).sum();
            let new_nb_inf: usize = self.infection.iter().map(|g| g.nb_units).sum();

            if new_nb_def == nb_def && new_nb_inf == nb_inf {
                //This fight is stuck, abort
                return (0, 0);
            }
            nb_def = new_nb_def;
            nb_inf = new_nb_inf;
        }
        (nb_def, nb_inf)
    }

    fn set_boost(&mut self, boost: usize) {
        for g in self.defense.iter_mut() {
            g.set_boost(boost);
        }
    }

    fn smallest_boost(&mut self) -> (usize, usize) {
        let backup_defense: Vec<UnitGroup> = self.defense.clone();
        let backup_infection: Vec<UnitGroup> = self.infection.clone();

        let mut boost: usize = 0;
        let (mut def, inf) = self.fight();
        while def == 0 {
            boost += 1;
            self.defense = backup_defense.clone();
            self.infection = backup_infection.clone();
            self.set_boost(boost);
            (def, _) = self.fight();
        }

        (inf, def)
    }
}

impl FromStr for ImmuneSystem {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_system(s: &str) -> Vec<UnitGroup> {
            s.lines().skip(1).map(|l| l.parse().unwrap()).collect()
        }

        let blocks: Vec<&str> = split_blocks(s);
        let defense: Vec<UnitGroup> = parse_system(blocks[0]);
        let infection: Vec<UnitGroup> = parse_system(blocks[1]);

        Ok(ImmuneSystem { defense, infection })
    }
}

fn main() {
    let now = std::time::Instant::now();

    let s = util::file_as_string("aoc_2018/input/day_24.txt").expect("Cannot open input file");
    let mut system: ImmuneSystem = s.parse().unwrap();
    let (part_1, part_2) = system.smallest_boost();

    println!(
        "Part1: The infection stand victorious with {} units",
        part_1
    );
    println!(
        "Part2: After getting a boost, the immune system wins with {} remaining units",
        part_2
    );
    println!("Computing time: {:?}", now.elapsed());
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE_1: &str = "Immune System:
17 units each with 5390 hit points (weak to radiation, bludgeoning) with an attack that does 4507 fire damage at initiative 2
989 units each with 1274 hit points (immune to fire; weak to bludgeoning, slashing) with an attack that does 25 slashing damage at initiative 3

Infection:
801 units each with 4706 hit points (weak to radiation) with an attack that does 116 bludgeoning damage at initiative 1
4485 units each with 2961 hit points (immune to radiation; weak to fire, cold) with an attack that does 12 slashing damage at initiative 4
";

    #[test]
    fn part_1() {
        let mut system: ImmuneSystem = EXAMPLE_1.parse().unwrap();
        assert_eq!((5216, 51), system.smallest_boost());
    }
}
