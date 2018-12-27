use std::cmp::Reverse;
use rayon::prelude::*;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day24.txt");

pub fn day24() -> (u32, u32) {
    let groups: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (part1(&groups), part2(&groups))
}

pub fn part1(groups: &[Group]) -> u32 {
    match fight(groups, 0) {
        Winner(_, units_left) => units_left,
        _ => panic!("The fight resulted in a draw")
    }
}

pub fn part2(groups: &[Group]) -> u32 {
    (1..u32::max_value())
        .into_par_iter()
        .filter_map(|boost| match fight(groups, boost) {
            Winner(ImmuneSystem, units_left) => Some(units_left),
            _ => None
        })
        .find_first(|_| true)
        .expect("No reasonnable immune boost value was enough")
}

use self::{ArmyKind::*, FightResult::*};

fn fight(groups: &[Group], boost: u32) -> FightResult {
    let mut groups = groups.to_vec();

    groups.iter_mut()
        .filter(|group| group.army == ImmuneSystem)
        .for_each(|group| group.attack.damage += boost);

    loop {
        // Targetting
        groups.sort_by_key(|group| (
            Reverse(group.effective_power()),
            Reverse(group.initiative),
        ));

        let mut locked = hashbrown::HashSet::with_capacity(groups.len());
        let mut fights = Vec::with_capacity(groups.len());

        for (attacking_idx, attacking_group) in groups.iter().enumerate() {
            let available_targets = groups.iter()
                .enumerate()
                .filter(|(defending_idx, defending_group)|
                    !locked.contains(defending_idx) &&
                    attacking_group.army != defending_group.army &&
                    attacking_group.damage(defending_group) > 0
                );

            let best_target = available_targets
                .max_by_key(|(_, defending_group)| (
                    attacking_group.damage(defending_group),
                    defending_group.effective_power(),
                    defending_group.initiative,
                ));

            if let Some((defending_idx, _)) = best_target {
                fights.push((attacking_idx, defending_idx));
                locked.insert(defending_idx);
            }
        }

        // Attacking
        fights.sort_by_key(|&(attacking_group_idx, _)|
            Reverse(groups[attacking_group_idx].initiative)
        );

        let mut total_casualties = 0;

        for (attacking_group_idx, defending_group_idx) in fights {
            let casualties = {
                let attacking_group = &groups[attacking_group_idx];
                let defending_group = &groups[defending_group_idx];
                let damage = attacking_group.damage(defending_group);
                damage / defending_group.hp
            };

            let defending_group = &mut groups[defending_group_idx];
            defending_group.units = defending_group.units.saturating_sub(casualties);
            total_casualties += casualties
        }

        // Assessment
        if total_casualties == 0 { break Draw }

        groups.retain(|group| group.units > 0);

        let soldiers_left = groups.iter()
            .fold((0, 0), |(immunes, infectious), group| match group.army {
                ImmuneSystem => (immunes + group.units, infectious),
                Infection => (immunes, infectious + group.units),
            });

        match soldiers_left {
            (0, infectious_left) => break Winner(Infection, infectious_left),
            (immunes_left, 0) => break Winner(ImmuneSystem, immunes_left),
            _ => ()
        }
    }
}

enum FightResult {
    Winner(ArmyKind, u32),
    Draw,
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Group> + '_ {
    let mut groups = input.split("\n\n");

    let immune_system = groups.next().unwrap().split('\n').skip(1);
    let infection = groups.next().unwrap().split('\n').skip(1);

    immune_system.map(|raw| parse_group(raw, ArmyKind::ImmuneSystem))
        .chain(infection.map(|raw| parse_group(raw, ArmyKind::Infection)))
}

fn parse_group(input: &str, army: ArmyKind) -> Group {
    let mut split = input.split(" units each with ");
    let units = split.next().unwrap().parse().unwrap();

    let mut split = split.next().unwrap().split(" hit points ");
    let hp = split.next().unwrap().parse().unwrap();

    let mut split = split.next().unwrap().split("with an attack that does ");
    let raw_resistances = split.next().unwrap();

    let mut weaknesses = Vec::new();
    let mut immunities = Vec::new();

    if raw_resistances.starts_with("(") {
        let mut resistances = raw_resistances[1..raw_resistances.len()-2].split("; ");

        while let Some(resistance) = resistances.next() {
            if resistance.starts_with("weak") {
                weaknesses.extend(resistance[8..].split(", ")
                    .map(DamageKind::from))
            } else {
                immunities.extend(resistance[10..].split(", ")
                    .map(DamageKind::from))
            }
        }
    }

    let mut split = split.next().unwrap().split(" damage at initiative ");
    let mut raw_damage = split.next().unwrap().split(' ');

    let damage_value = raw_damage.next().unwrap().parse().unwrap();
    let damage_kind = raw_damage.next().unwrap().into();

    let initiative = split.next().unwrap().parse().unwrap();

    Group {
        army,
        units,
        hp,
        attack: Attack { damage: damage_value, kind: damage_kind },
        initiative,
        weaknesses,
        immunities
    }
}

#[derive(Debug, Clone, PartialEq)]
enum ArmyKind {
    ImmuneSystem,
    Infection
}

#[derive(Debug, Clone)]
pub struct Group {
    army: ArmyKind,
    units: u32,
    hp: u32,
    attack: Attack,
    initiative: u32,
    weaknesses: Vec<DamageKind>,
    immunities: Vec<DamageKind>,
}

#[derive(Debug, Clone)]
struct Attack {
    damage: u32,
    kind: DamageKind,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum DamageKind {
    Bludgeoning,
    Cold,
    Fire,
    Radiation,
    Slashing,
}

impl Group {
    fn effective_power(&self) -> u32 {
        self.units * self.attack.damage
    }

    fn damage(&self, other: &Group) -> u32 {
        let is_immune = other.immunities.contains(&self.attack.kind);
        let is_weak = other.weaknesses.contains(&self.attack.kind);

        let modifier = match (is_immune, is_weak) {
            (true, _) => 0,
            (_, true) => 2,
            _ => 1
        };

        self.effective_power() * modifier
    }
}

impl From<&str> for DamageKind {
    fn from(input: &str) -> Self {
        match input {
            "bludgeoning" => DamageKind::Bludgeoning,
            "cold" => DamageKind::Cold,
            "fire" => DamageKind::Fire,
            "radiation" => DamageKind::Radiation,
            "slashing" => DamageKind::Slashing,
            unknown => panic!("unknown damage kind: {}", unknown)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let groups: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&groups), 22676);
    }

    #[test]
    fn p2() {
        let groups: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&groups), 4510);
    }
}
