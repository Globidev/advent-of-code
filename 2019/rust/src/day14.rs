use std::fmt::Debug;
use std::collections::HashMap;
use std::cmp::Ordering;
use itertools::Itertools;
use num::Integer;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day14.txt");

pub fn day14() -> impl Debug {
    let reactions = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&reactions), part2(&reactions))
}

pub fn part1(reactions: &[Reaction]) -> usize {
    let goal = Reagent { chemical: Chemical::Fuel, amount: 1 };
    ore_requirements(reactions, goal)
}

pub fn part2(reactions: &[Reaction]) -> usize {
    let goal = Reagent { chemical: Chemical::Fuel, amount: 1 };
    let min_ore_needed = ore_requirements(reactions, goal);

    let directly_produced_fuel = 1_000_000_000_000 / min_ore_needed;
    dichotomic_search((directly_produced_fuel, directly_produced_fuel * 2), reactions)
}

fn ore_requirements(reactions: &[Reaction], target: Reagent) -> usize {
    let producers: HashMap<_, _> = reactions.iter()
        .map(|(inputs, output)| (&output.chemical, (inputs, &output.amount)))
        .collect();

    let mut reqs = HashMap::new();
    let mut leftovers = HashMap::new();
    reqs.insert(target.chemical, target.amount);

    let mut ore_amount = 0;

    while !reqs.is_empty() {
        let target_chem = reqs.keys().next().unwrap().clone();
        let target_amount = reqs.remove(&target_chem).unwrap();

        if target_chem == Chemical::Ore {
            ore_amount += target_amount
        } else {
            let &(inputs, produced_amount) = producers.get(&target_chem)
                .expect("Missing reaction");

            let (reactions_needed, remainder) = target_amount.div_rem(&produced_amount);
            let reactions_needed = reactions_needed + remainder.min(1);
            if remainder > 0 {
                *leftovers.entry(target_chem).or_insert(0) += produced_amount - remainder;
            }

            for input in inputs {
                let raw_amount = reactions_needed * input.amount;
                let adjust = leftovers.remove(&input.chemical).unwrap_or(0);
                let amount = if adjust > raw_amount {
                    leftovers.insert(input.chemical.clone(), adjust - raw_amount);
                    0
                } else {
                    raw_amount - adjust
                };
                *reqs.entry(input.chemical.clone()).or_insert(0) += amount;
            }
        }
    }

    ore_amount
}

fn dichotomic_search(bound: (usize, usize), reactions: &[Reaction]) -> usize {
    const THRESHOLD: usize = 1_000_000_000_000;

    let (mut lo, mut hi) = bound;

    while hi - lo > 1 {
        let mid = (lo + hi) / 2;
        let goal = Reagent { chemical: Chemical::Fuel, amount: mid };
        let ore_amount = ore_requirements(reactions, goal);

        match ore_amount.cmp(&THRESHOLD) {
            Ordering::Less => lo = mid,
            Ordering::Equal => return mid,
            Ordering::Greater => hi = mid,
        }
    }

    lo
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum Chemical {
    Ore,
    Fuel,
    Other(String),
}

impl From<&str> for Chemical {
    fn from(s: &str) -> Self {
        match s {
            "ORE" => Self::Ore,
            "FUEL" => Self::Fuel,
            other => Self::Other(other.to_owned()),
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Reagent {
    chemical: Chemical,
    amount: usize,
}

type Reaction = (Vec<Reagent>, Reagent);

pub fn parse_input(input: &str) -> impl Iterator<Item = Reaction> + '_ {
    input.lines()
        .map(|raw_reaction| {
            let (input_reagents, mut output_reagents) = raw_reaction
                .split(" => ")
                .map(parse_reagent)
                .collect_tuple()
                .expect("Missing reagents");

            assert!(output_reagents.len() == 1, "More than 1 output reagent in reaction");
            (input_reagents, output_reagents.swap_remove(0))
        })
}

fn parse_reagent(raw_reagents: &str) -> Vec<Reagent> {
    raw_reagents.split(", ")
        .map(|raw_reagent| {
            let (raw_amount, raw_chemical) = raw_reagent.split(' ')
                .collect_tuple()
                .expect("Malformed reagent");

            let amount = raw_amount.parse().expect("Malformed reagent amount");
            let chemical = raw_chemical.into();

            Reagent {
                chemical,
                amount,
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 202_617);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 7_863_863);
    }
}
