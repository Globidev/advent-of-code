use std::fmt::Debug;
use itertools::Itertools;
use crate::intcode::{Int, vm::VirtualMachine, io::ext::{Split, Iter, SingleOutput}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day09.txt");

pub fn day09() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    run_program(program, 1)
}

pub fn part2(program: &[Int]) -> Int {
    run_program(program, 2)
}

fn run_program(program: &[Int], seed: Int) -> Int {
    let mut out = SingleOutput::new();
    let io = Split(
        Iter(std::iter::once(seed)),
        &mut out,
    );

    let vm = VirtualMachine::new(program, io);
    vm.run();

    out.get().expect("Didn't get any output")
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
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part1(&code), 3_638_931_938);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 86_025);
    }
}
