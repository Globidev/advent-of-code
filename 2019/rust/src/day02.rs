use std::fmt::Debug;
use itertools::Itertools;
use rayon::prelude::*;
use crate::intcode::{Int, vm::VirtualMachine, io::ext::Pure};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day02.txt");

pub fn day02() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    run_program(program, 12, 02)
}

pub fn part2(program: &[Int]) -> Int {
    const TARGET_OUTPUT: Int = 19_690_720;

    let combinations = (0..=99_i64).into_par_iter()
        .flat_map(|noun| (0..=99_i64).into_par_iter().map(move |verb| (noun, verb)));

    let (noun, verb) = combinations
        .find_any(|&(noun, verb)| run_program(program, noun, verb) == TARGET_OUTPUT)
        .expect("No noun/verb combination produced the desired output!");

    100 * noun + verb
}

fn run_program(program: &[Int], noun: Int, verb: Int) -> Int {
    let mut program = program.to_vec();
    program[1] = noun;
    program[2] = verb;

    let vm = VirtualMachine::new(program, Pure);

    let mem_snapshot = vm.run();
    mem_snapshot[0]
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

        assert_eq!(part1(&program), 4_462_686);
    }

    #[test]
    fn p2() {
        let program = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&program), 5_936);
    }
}
