use std::fmt::Debug;
use std::ops::{Add, Mul};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day02.txt");

pub fn day02() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR);

    (part1(program.clone()), part2(program))
}

pub fn part1(mut program: Program) -> IntegerCode {
    program.restore_gravity_assist();
    program.run()
}

pub fn part2(program: Program) -> IntegerCode {
    const TARGET_OUTPUT: IntegerCode = 19_690_720;

    for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = program.clone();
            program.seed_with(noun, verb);

            if program.run() == TARGET_OUTPUT {
                return 100 * noun + verb
            }
        }
    }

    panic!("No noun/verb combination produced the desired output!")
}

#[derive(Clone)]
pub struct Program {
    code: Vec<IntegerCode>
}

impl Program {
    fn restore_gravity_assist(&mut self) {
        // 1202 program alarm
        self.seed_with(12, 02)
    }

    fn seed_with(&mut self, noun: IntegerCode, verb: IntegerCode) {
        self.code[1] = noun;
        self.code[2] = verb;
    }

    fn run(&mut self) -> IntegerCode {
        let code = self.code.as_mut_slice();

        let mut pc = Pc::new();

        while let Some(instr) = pc.next_instr(code) {
            // dbg!(&instr);
            if !instr.execute(code) {
                break
            }
        }

        self.code[0]
    }
}

struct Pc {
    position: usize,
}

impl Pc {
    fn new() -> Self {
        Self { position: 0 }
    }

    fn next_instr(&mut self, code: &mut [IntegerCode]) -> Option<Instruction> {
        use std::convert::TryInto;

        let instr_range = self.position..self.position + 4;
        let raw_instr: [IntegerCode; 4] = code.get(instr_range)?.try_into()
            .expect("Got an invalid range somehow");

        let [op_code, lhs, rhs, output] = raw_instr;
        let instr = Instruction {
            op: Op::from_code(op_code),
            inputs: (lhs, rhs),
            output,
        };

        self.position += 4;
        Some(instr)
    }
}

#[derive(Debug)]
struct Instruction {
    op: Op,
    inputs: (IntegerCode, IntegerCode),
    output: IntegerCode,
}

impl Instruction {
    fn execute(self, code: &mut [IntegerCode]) -> bool {
        let bin_op = match self.op {
            Op::Add => Add::add,
            Op::Mul => Mul::mul,
            Op::Halt => return false,
        };

        let (lhs, rhs) = self.inputs;
        code[self.output] = bin_op(code[lhs], code[rhs]);

        true
    }
}

#[derive(Debug)]
enum Op {
    Add,
    Mul,
    Halt,
}

impl Op {
    fn from_code(op_code: IntegerCode) -> Self {
        match op_code {
            1  => Op::Add,
            2  => Op::Mul,
            99 => Op::Halt,
            invalid => panic!("Got an invalid op code: {}", invalid)
        }
    }
}

pub type IntegerCode = usize;

pub fn parse_input(input: &str) -> Program {
    let code = input.split(',')
        .map(|raw_number| raw_number.parse().expect("Invalid integer code"))
        .collect();

    Program { code }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let program = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(program), 4_462_686);
    }

    #[test]
    fn p2() {
        let program = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(program), 5_936);
    }
}
