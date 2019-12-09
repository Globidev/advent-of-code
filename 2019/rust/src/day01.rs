use std::fmt::Debug;
use itertools::Itertools;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day01.txt");

pub fn day01() -> impl Debug {
    let modules = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&modules), part2(&modules))
}

pub fn part1(modules: &[Module]) -> Fuel {
    modules.iter()
        .flat_map(|module| module.fuel_required_for_launch())
        .sum()
}

pub fn part2(modules: &[Module]) -> Fuel {
    modules.iter()
        .flat_map(|module| {
            let init_fuel = module.fuel_required_for_launch();
            std::iter::successors(init_fuel, |&fuel| {
                let fuel_module = Module { mass: fuel };
                fuel_module.fuel_required_for_launch()
            })
        })
        .sum()
}

pub struct Module {
    mass: Mass,
}

impl Module {
    pub fn fuel_required_for_launch(&self) -> Option<Fuel> {
        (self.mass / 3).checked_sub(2)
    }
}

pub type Mass = u32;
pub type Fuel = u32;

pub fn parse_input(input: &str) -> impl Iterator<Item = Module> + '_ {
    input.lines()
        .map(|raw_mass| raw_mass.parse().expect("Invalid mass number"))
        .map(|mass| Module { mass })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 3_152_375);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 4_725_720);
    }
}
