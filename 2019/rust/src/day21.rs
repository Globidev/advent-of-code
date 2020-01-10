use std::fmt::{self, Display, Debug};
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day21.txt");

pub fn day21() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    use {Instruction::*, Register::*};

    run_springscript(program, &[
        Not(C, J),
        And(D, J),
        Not(A, T),
        Or(T, J),
        Walk
    ])
}

pub fn part2(program: &[Int]) -> Int {
    use {Instruction::*, Register::*};

    run_springscript(program, &[
        Not(C, J),
        And(D, J),
        And(H, J),
        Not(B, T),
        And(D, T),
        Or(T, J),
        Not(A, T),
        Or(T, J),
        Run,
    ])
}

fn run_springscript(program: &[Int], script: &[Instruction]) -> Int {
    let ascii_script = script.iter().join("\n");
    let script_bytes = ascii_script.bytes().map(Int::from);

    VirtualMachine::load(program)
        .input_iter(script_bytes)
        .single_output()
        .run()
        .output()
        .expect("Failed to reach the hull!")
}

#[derive(Debug)]
#[allow(unused)]
enum Register { A, B, C, D, E, F, G, H, I, T, J }

#[derive(Debug)]
enum Instruction {
    And(Register, Register),
    Not(Register, Register),
    Or(Register, Register),
    Walk,
    Run
}

impl Display for Register {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Register::A => write!(f, "A"),
            Register::B => write!(f, "B"),
            Register::C => write!(f, "C"),
            Register::D => write!(f, "D"),
            Register::E => write!(f, "E"),
            Register::F => write!(f, "F"),
            Register::G => write!(f, "G"),
            Register::H => write!(f, "H"),
            Register::I => write!(f, "I"),
            Register::T => write!(f, "T"),
            Register::J => write!(f, "J"),
        }
    }
}

impl Display for Instruction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Instruction::And(r1, r2) => write!(f, "AND {} {}", r1, r2),
            Instruction::Not(r1, r2) => write!(f, "NOT {} {}", r1, r2),
            Instruction::Or(r1, r2) => write!(f, "OR {} {}", r1, r2),
            Instruction::Walk => write!(f, "WALK\n"),
            Instruction::Run => write!(f, "RUN\n"),
        }
    }
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

        assert_eq!(part1(&code), 19_353_692);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 1_142_048_514);
    }
}
