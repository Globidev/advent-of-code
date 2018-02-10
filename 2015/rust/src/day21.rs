extern crate nom;

#[derive(Debug)]
struct Character {
    hp: u32,
    damage: u32,
    armor: u32,
}

const INITIAL_PLAYER: Character = Character { hp: 100, damage: 0, armor: 0 };

#[derive(Debug, PartialEq)]
enum FightResult { Win, Loss }

#[derive(Debug, PartialEq)]
struct Item {
    cost: u32,
    damage: u32,
    armor: u32
}

type Items = Vec<&'static Item>;

const WEAPONS: [Item; 5] = [
    Item { cost: 8,  damage: 4, armor: 0 },
    Item { cost: 10, damage: 5, armor: 0 },
    Item { cost: 25, damage: 6, armor: 0 },
    Item { cost: 40, damage: 7, armor: 0 },
    Item { cost: 74, damage: 8, armor: 0 },
];

const ARMORS: [Item; 5] = [
    Item { cost: 13,  damage: 0, armor: 1 },
    Item { cost: 31,  damage: 0, armor: 2 },
    Item { cost: 53,  damage: 0, armor: 3 },
    Item { cost: 75,  damage: 0, armor: 4 },
    Item { cost: 102, damage: 0, armor: 5 },
];

const RINGS: [Item; 6] = [
    Item { cost: 25,  damage: 1, armor: 0 },
    Item { cost: 50,  damage: 2, armor: 0 },
    Item { cost: 100, damage: 3, armor: 0 },
    Item { cost: 20,  damage: 0, armor: 1 },
    Item { cost: 40,  damage: 0, armor: 2 },
    Item { cost: 80,  damage: 0, armor: 3 },
];

fn parse_character(raw_character: &str) -> Character {
    use self::nom::*;

    use std::str::from_utf8;

    named!(number<u32>, map!(
        digit,
        |s| from_utf8(s).unwrap().parse::<u32>().unwrap()
    ));

    named!(character<Character>, do_parse!(
        tag_s!("Hit Points: ") >>
        hp: number             >>
        tag_s!("\nDamage: ")   >>
        dmg: number            >>
        tag_s!("\nArmor: ")    >>
        armor: number          >>
        (Character { hp: hp, damage: dmg, armor: armor })
    ));

    match character(raw_character.as_bytes()) {
        IResult::Done(_, character) => character,
        _                           => panic!("Wrong character format")
    }
}

fn damage_amount(damage: u32, armor: u32) -> u32 {
    use std::cmp::Ordering::Greater;

    match damage.cmp(&armor) {
        Greater => damage - armor,
        _       => 1
    }
}

fn turns_until_death(hp: u32, damage: u32) -> u32 {
    hp / damage + if hp % damage == 0 { 0 } else { 1 }
}

fn fight_result(player: &Character, boss: &Character) -> FightResult {
    use std::cmp::Ordering::Greater;

    let player_damage_per_turn = damage_amount(player.damage, boss.armor);
    let boss_damage_per_turn = damage_amount(boss.damage, player.armor);

    let turns_until_player_death = turns_until_death(player.hp, boss_damage_per_turn);
    let turns_until_boss_death = turns_until_death(boss.hp, player_damage_per_turn);

    match turns_until_boss_death.cmp(&turns_until_player_death) {
        Greater => FightResult::Loss,
        _       => FightResult::Win
    }
}

fn equip(items: &Items) -> Character {
    items.iter().fold(INITIAL_PLAYER, |stats, item| {
        Character {
            hp: stats.hp,
            damage: stats.damage + item.damage,
            armor: stats.armor + item.armor
        }
    })
}

fn fight_result_with_items(items: &Items, boss: &Character) -> FightResult {
    fight_result(&equip(items), boss)
}

fn item_combinations() -> Vec<Items> {
    let mut items = Vec::new();

    for weapon in &WEAPONS {
        items.push(vec![weapon]);
        for armor in &ARMORS {
            items.push(vec![weapon, armor]);
            for ring1 in &RINGS {
                items.push(vec![weapon, ring1]);
                items.push(vec![weapon, armor, ring1]);
                for ring2 in &RINGS {
                    if ring2 != ring1 {
                        items.push(vec![weapon, ring1, ring2]);
                        items.push(vec![weapon, armor, ring1, ring2]);
                    }
                }
            }
        }
    }

    items
}

fn winning_item_combinations(boss: &Character) -> Vec<Items> {
    item_combinations().into_iter().filter(|items|
        fight_result_with_items(items, boss) == FightResult::Win
    ).collect()
}

fn losing_item_combinations(boss: &Character) -> Vec<Items> {
    item_combinations().into_iter().filter(|items|
        fight_result_with_items(items, boss) == FightResult::Loss
    ).collect()
}

pub fn p1(input: &str) -> u32 {
    let boss = parse_character(input.trim());

    winning_item_combinations(&boss).iter()
                                    .map(|items| items.iter().map(|i| i.cost).sum())
                                    .min().unwrap_or(0)
}

pub fn p2(input: &str) -> u32 {
    let boss = parse_character(input.trim());
    losing_item_combinations(&boss).iter()
                                   .map(|items| items.iter().map(|i| i.cost).sum())
                                   .max().unwrap_or(0)
}
