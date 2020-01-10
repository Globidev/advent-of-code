use std::fmt::Debug;
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day05.txt");

pub fn day05() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    const SHIP_AIR_CONDITIONER_UNIT_ID: Int = 1;

    run_program(program, SHIP_AIR_CONDITIONER_UNIT_ID)
}

pub fn part2(program: &[Int]) -> Int {
    const SHIP_THERMAL_RADIATOR_CONTROLLER_ID: Int = 5;

    run_program(program, SHIP_THERMAL_RADIATOR_CONTROLLER_ID)
}

fn run_program(program: &[Int], seed: Int) -> Int {
    VirtualMachine::load(program)
        .input_once(seed)
        .single_output()
        .run()
        .output()
        .expect("No output!")
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Int> + '_ {
    input.split(',')
        .map(|raw_number| raw_number.parse().expect("Invalid integer code"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let program = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part1(&program), 9_006_673);
    }

    #[test]
    fn p2() {
        let program = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&program), 3_629_692);
    }
}
