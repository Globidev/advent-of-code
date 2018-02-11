extern crate nom;

use std::collections::{BinaryHeap, VecDeque};

#[derive(Clone)]
struct Boss {
    hp: u32,
    damage: u32,
}

struct Player {
    hp: u32,
    armor: u32,
    mana: u32,
}

struct Spell {
    id: &'static str,
    cost: u32,
    effect: Effect
}

impl Eq for Spell { }
impl PartialEq for Spell {
    fn eq(&self, other: &Spell) -> bool {
        self.id == other.id
    }
}

type Spells = Vec<&'static Spell>;

type Duration = u16;
type Resolve = fn(&mut Player, &mut Boss, Duration);

type Effects = VecDeque<(Resolve, Duration, &'static str)>;

enum Effect {
    Immediate(Resolve),
    OverTime(Duration, Resolve),
}

enum FightResult { Win, LossByDeath, LossByOOM, }
type FightLogic = fn(&mut Fight, &Spells) -> FightResult;
type PreTurn = fn(&mut Fight);

struct Fight {
    player: Player,
    boss: Boss,
    effects: Effects,
}

const SPELLS: [Spell; 5] = [
    Spell   { id: "Magic Missile"
            , cost: 53
            , effect: Effect::Immediate(|_, b, _|
                b.hp = b.hp.checked_sub(4).unwrap_or(0)
            )
            },
    Spell   { id: "Drain"
            , cost: 73
            , effect: Effect::Immediate(|p, b, _| {
                b.hp = b.hp.checked_sub(2).unwrap_or(0);
                p.hp += 2;
            })
            },
    Spell   { id: "Shield"
            , cost: 113
            , effect: Effect::OverTime(6, |p, _, t|
                match t {
                    6 => p.armor += 7,
                    1 => p.armor -= 7,
                    _ => ()
                }
            )
            },
    Spell   { id: "Poison"
            , cost: 173
            , effect: Effect::OverTime(6, |_, b, _|
                b.hp = b.hp.checked_sub(3).unwrap_or(0)
            )
            },
    Spell   { id: "Recharge"
            , cost: 229
            , effect: Effect::OverTime(5, |p, _, _|
                p.mana += 101
            )
            }
];

const PLAYER: Player = Player {
    hp: 50,
    armor: 0,
    mana: 500,
};

fn parse_boss(raw_boss: &str) -> Boss {
    use self::nom::*;

    use std::str::from_utf8;

    named!(number<u32>, map!(
        digit,
        |s| from_utf8(s).unwrap().parse::<u32>().unwrap()
    ));

    named!(boss<Boss>, do_parse!(
        tag_s!("Hit Points: ") >>
        hp: number             >>
        tag_s!("\nDamage: ")   >>
        dmg: number            >>
        (Boss { hp: hp, damage: dmg })
    ));

    match boss(raw_boss.as_bytes()) {
        IResult::Done(_, boss) => boss,
        _                      => panic!("Wrong character format")
    }
}

fn damage_amount(damage: u32, armor: u32) -> u32 {
    use std::cmp::Ordering::Greater;

    match damage.cmp(&armor) {
        Greater => damage - armor,
        _       => 1
    }
}

fn can_cast(fight: &Fight, spell: &Spell) -> bool {
    let enough_mana = fight.player.mana >= spell.cost;
    let active_spell_idx = fight.effects.iter()
                                .find(|&&(_, d, id)| d > 1 && id == spell.id);

    enough_mana && active_spell_idx.is_none()
}

fn cast(fight: &mut Fight, spell: &Spell) {
    use self::Effect::*;

    match spell.effect {
        Immediate(resolve) => resolve(&mut fight.player, &mut fight.boss, 0),
        OverTime(duration, resolve) => {
            fight.effects.push_front((resolve, duration, spell.id))
        },
    }

    fight.player.mana -= spell.cost;
}

fn resolve_effects(fight: &mut Fight) {
    let mut truncate_idx = 0;

    for effect in fight.effects.iter_mut() {
        effect.0(&mut fight.player, &mut fight.boss, effect.1);
        effect.1 -= 1;

        if effect.1 > 0 {
            truncate_idx += 1;
        }
    }

    fight.effects.truncate(truncate_idx);
}

fn fight_logic(spells: &Spells, fight: &mut Fight, pre_turn: PreTurn) -> FightResult {
    use self::FightResult::*;

    for spell in spells {
        // Player turn
        pre_turn(fight);
        if fight.player.hp == 0 { return LossByDeath }

        resolve_effects(fight);
        if fight.boss.hp == 0 { return Win }

        cast(fight, spell);
        if fight.boss.hp == 0 { return Win }

        // Boss turn
        resolve_effects(fight);
        if fight.boss.hp == 0 { return Win }

        let boss_damage = damage_amount(fight.boss.damage, fight.player.armor);
        fight.player.hp = fight.player.hp.checked_sub(boss_damage).unwrap_or(0);
        if fight.player.hp == 0 { return LossByDeath }

    }

    LossByOOM
}

#[derive(Eq, PartialEq)]
struct Node {
    cost: u32,
    spells: Spells
}

use std::cmp::Ordering;

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn minmax_fight(boss: &Boss, fight_logic: FightLogic) -> Option<u32> {
    let mut open_set = BinaryHeap::new();

    for spell in &SPELLS {
        open_set.push(Node { cost: spell.cost, spells: vec![spell] })
    }

    loop {
        if let Some(node) = open_set.pop() {
            let mut fight = Fight {
                boss: boss.clone(),
                player: PLAYER,
                effects: Effects::new()
            };

            let result = fight_logic(&mut fight, &node.spells);

            match result {
                FightResult::Win         => return Some(node.cost),
                FightResult::LossByDeath => (),
                FightResult::LossByOOM   => {
                    let castable_spells = SPELLS.iter()
                                                .filter(|s| can_cast(&fight, s));

                    for spell in castable_spells {
                        let mut new_spells = node.spells.clone();
                        new_spells.push(spell);
                        open_set.push(Node {
                            cost: node.cost + spell.cost,
                            spells: new_spells
                        });
                    }
                },
            }
        }
        else {
            break
        }
    }

    None
}

fn fight_p1(fight: &mut Fight, spells: &Spells) -> FightResult {
    fight_logic(spells, fight, |_| { })
}

fn fight_p2(fight: &mut Fight, spells: &Spells) -> FightResult {
    fight_logic(spells, fight, |f| f.player.hp -= 1)
}

pub fn p1(input: &str) -> u32 {
    let boss = parse_boss(input.trim());

    minmax_fight(&boss, fight_p1).unwrap_or(0)
}

pub fn p2(input: &str) -> u32 {
    let boss = parse_boss(input.trim());

    minmax_fight(&boss, fight_p2).unwrap_or(0)
}
