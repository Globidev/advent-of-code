use std::fmt::Debug;
use std::thread;
use std::sync::mpsc::{Sender, channel};
use itertools::Itertools;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day07.txt");

pub fn day07() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Integer]) -> Integer {
    (0..=4)
        .permutations(5)
        .map(|settings| run_amplifiers(program, &settings))
        .max()
        .expect("No settings")
}

pub fn part2(program: &[Integer]) -> Integer {
    (5..=9)
        .permutations(5)
        .map(|settings| run_amplifiers_feedback_loop(program, &settings))
        .max()
        .expect("No settings")
}

fn run_amplifiers(program: &[Integer], settings: &[Integer]) -> Integer {
    let (mut tx, mut rx) = channel();
    let init_tx = tx.clone();

    let amplifiers = settings.iter()
        .map(|&setting| {
            let (next_tx, next_rx) = channel();
            tx.send(setting).expect("Failed to send phase setting");
            tx = next_tx.clone();
            let rx = std::mem::replace(&mut rx, next_rx);
            let program = Program::load(program.to_vec(), rx.into_iter(), next_tx);
            program
        });

    let handles = amplifiers
        .map(|mut amp| thread::spawn(move || amp.run()))
        .collect_vec();

    init_tx.send(0).expect("Failed to send initial input value");

    let result = rx.recv().expect("Failed to get output");

    for handle in handles {
        handle.join().expect("Failed to join amplifiers")
    }

    result
}

fn run_amplifiers_feedback_loop(code: &[Integer], settings: &[Integer]) -> Integer {
    let (mut tx, mut rx) = channel();
    let init_tx = tx.clone();

    let amplifiers = settings[1..].iter()
        .map(|&setting| {
            let (next_tx, next_rx) = channel();
            tx.send(setting).expect("Failed to send phase setting");
            tx = next_tx.clone();
            let rx = std::mem::replace(&mut rx, next_rx);
            let program = Program::load(code.to_vec(), rx.into_iter(), next_tx);
            program
        })
        .collect_vec();

    let first_amp = Program::load(code.to_vec(), rx.into_iter(), init_tx);
    tx.send(settings[0]).expect("Failed to send phase setting");
    tx.send(0).expect("Failed to initial input value");

    let handles = std::iter::once(first_amp).chain(amplifiers.into_iter())
        .map(|mut amp| thread::spawn(move || { amp.run(); amp }))
        .collect_vec();

    let mut amplifiers = handles.into_iter()
        .map(|handle| handle.join().expect("Failed to join amplifiers"))
        .collect_vec();

    amplifiers.first_mut()
        .expect("No amplifiers")
        .input.next()
        .expect("Failed to get output")
}

#[derive(Clone)]
pub struct Program<I, O> {
    memory: Memory,
    input: I,
    output: O
}

impl<I, O> Program<I, O>
where
    I: InputStream,
    O: OutputStream,
{
    fn load(code: Vec<Integer>, input: I, output: O) -> Self {
        Self {
            memory: Memory::load(code.into_boxed_slice()),
            input,
            output,
        }
    }

    fn run(&mut self) {
        let mem = &mut self.memory;

        let mut pc = Pc::new();

        loop {
            let instr = pc.decode_instr(mem);

            if !instr.execute(mem, &mut self.input, &mut self.output, &mut pc) {
                break
            }
        }
    }
}

pub trait InputStream: Iterator<Item = Integer> { }
impl<I: Iterator<Item = Integer>> InputStream for I { }

pub trait OutputStream {
    fn output(&mut self, value: Integer);
}

impl OutputStream for Sender<Integer> {
    fn output(&mut self, value: Integer) {
        // println!("{:?} ==> {}", thread::current().id(), value);
        self.send(value)
            .expect("Failed to send value over channel!")
    }
}

#[derive(Debug, Clone)]
struct Memory {
    cells: Box<[Integer]>
}

impl Memory {
    fn load(cells: Box<[Integer]>) -> Self { Self { cells } }

    fn get(&self, idx: usize) -> Integer {
        unsafe { *self.cells.get_unchecked(idx) }
    }

    fn set(&mut self, idx: Integer, value: Integer) {
        unsafe { *self.cells.get_unchecked_mut(idx as usize) = value; }
    }
}

struct Pc {
    position: usize,
}

impl Pc {
    fn new() -> Self {
        Self { position: 0 }
    }

    fn next_position(&mut self) -> usize {
        let pos = self.position;
        self.position += 1;
        pos
    }

    fn jump(&mut self, new_position: Integer) {
        self.position = new_position as usize;
    }

    #[inline(always)]
    fn decode_params<const N: usize>(&mut self, mem: &Memory, param_codes: Integer)
        -> [Param; N]
    {
        use std::mem::MaybeUninit;

        let mut params: [MaybeUninit<Param>; N] = unsafe {
            MaybeUninit::uninit().assume_init()
        };

        #[inline(always)]
        fn lookup(val: Integer) -> [Integer; 3] {
            match val {
                000 => [0, 0, 0],
                001 => [1, 0, 0],
                010 => [0, 1, 0],
                011 => [1, 1, 0],
                100 => [0, 0, 1],
                // 101 => [1, 0, 1],
                110 => [0, 1, 1],
                // 111 => [1, 1, 1],
                _ => unsafe { std::hint::unreachable_unchecked() },
            }
        }

        let codes = lookup(param_codes);

        for (param_slot, &param_code) in params.iter_mut().zip(codes.iter()) {
            let param = Param::from_code_and_value(param_code, mem.get(self.next_position()));
            *param_slot = MaybeUninit::new(param);
        }

        unsafe { *(&params as *const _ as *const _) }
    }

    fn decode_instr(&mut self, mem: &Memory) -> Instruction {
        use Instruction::*;

        let op_descriptor = mem.get(self.next_position());
        let op_code = op_descriptor % 100;
        let param_codes = op_descriptor / 100;

        let instr = match op_code {
            ADD_OP_CODE => {
                let params: [Param; 3] = self.decode_params(mem, param_codes);
                let [lhs, rhs, dest_param] = params;
                let dest = match dest_param {
                    Param::Immediate(_) => unsafe { std::hint::unreachable_unchecked() }//panic!("Output parameters can't be immediates"),
                    Param::Position(value) => value,
                };
                Add { lhs, rhs, dest }
            },
            MUL_OP_CODE => {
                let params: [Param; 3] = self.decode_params(mem, param_codes);
                let [lhs, rhs, dest_param] = params;
                let dest = match dest_param {
                    Param::Immediate(_) => unsafe { std::hint::unreachable_unchecked() }//panic!("Output parameters can't be immediates"),
                    Param::Position(value) => value,
                };
                Mul { lhs, rhs, dest }
            },
            INPUT_OP_CODE => {
                let params: [Param; 1] = self.decode_params(mem, param_codes);
                let [dest_param] = params;
                let dest = match dest_param {
                    Param::Immediate(_) => unsafe { std::hint::unreachable_unchecked() }//panic!("Output parameters can't be immediates"),
                    Param::Position(value) => value,
                };
                Input { dest }
            },
            OUTPUT_OP_CODE => {
                let params: [Param; 1] = self.decode_params(mem, param_codes);
                let [param] = params;
                Output { param }
            },
            JMP_TRUE_OP_CODE => {
                let params: [Param; 2] = self.decode_params(mem, param_codes);
                let [cond, dest] = params;
                JmpTrue { cond, dest }
            },
            JMP_FALSE_OP_CODE => {
                let params: [Param; 2] = self.decode_params(mem, param_codes);
                let [cond, dest] = params;
                JmpFalse { cond, dest }
            },
            LT_OP_CODE => {
                let params: [Param; 3] = self.decode_params(mem, param_codes);
                let [lhs, rhs, dest_param] = params;
                let dest = match dest_param {
                    Param::Immediate(_) => unsafe { std::hint::unreachable_unchecked() }//panic!("Output parameters can't be immediates"),
                    Param::Position(value) => value,
                };
                Lt { lhs, rhs, dest }
            },
            EQ_OP_CODE => {
                let params: [Param; 3] = self.decode_params(mem, param_codes);
                let [lhs, rhs, dest_param] = params;
                let dest = match dest_param {
                    Param::Immediate(_) => unsafe { std::hint::unreachable_unchecked() }//panic!("Output parameters can't be immediates"),
                    Param::Position(value) => value,
                };
                Eq { lhs, rhs, dest }
            },
            HALT_OP_CODE => Halt,
            invalid => panic!("Unknown Op code: {}", invalid)
        };

        instr
    }
}

const ADD_OP_CODE: Integer = 1;
const MUL_OP_CODE: Integer = 2;
const INPUT_OP_CODE: Integer = 3;
const OUTPUT_OP_CODE: Integer = 4;
const JMP_TRUE_OP_CODE: Integer = 5;
const JMP_FALSE_OP_CODE: Integer = 6;
const LT_OP_CODE: Integer = 7;
const EQ_OP_CODE: Integer = 8;
const HALT_OP_CODE: Integer = 99;

#[derive(Debug)]
enum Instruction {
    Add { lhs: Param, rhs: Param, dest: Integer },
    Mul { lhs: Param, rhs: Param, dest: Integer },
    Input { dest: Integer },
    Output { param: Param },
    JmpTrue { cond: Param, dest: Param },
    JmpFalse { cond: Param, dest: Param },
    Lt { lhs: Param, rhs: Param, dest: Integer },
    Eq { lhs: Param, rhs: Param, dest: Integer },
    Halt,
}

#[derive(Debug, Clone, Copy)]
enum Param {
    Position(Integer),
    Immediate(Integer),
}

impl Param {
    fn from_code_and_value(code: Integer, value: Integer) -> Self {
        match code {
            0 => Param::Position(value),
            1 => Param::Immediate(value),
            other => panic!("Invalid param code: {}", other)
        }
    }

    fn value(self, mem: &Memory) -> Integer {
        match self {
            Param::Position(addr) => mem.get(addr as usize), // ⚠
            Param::Immediate(val) => val,
        }
    }
}

impl Instruction {
    #[inline]
    fn execute<I, O>(self, mem: &mut Memory, input: &mut I, output: &mut O, pc: &mut Pc) -> bool
    where
        I: InputStream,
        O: OutputStream,
    {
        match self {
            Instruction::Add { lhs, rhs, dest } => {
                mem.set(dest, lhs.value(mem) + rhs.value(mem));
            },
            Instruction::Mul { lhs, rhs, dest } => {
                mem.set(dest, lhs.value(mem) * rhs.value(mem));
            },
            Instruction::Input { dest } => {
                // println!("{:?} WAITING input", thread::current().id());
                let v = input.next().expect("No input available!");
                mem.set(dest, v);
                // println!("{:?} GOT input: {}", thread::current().id(), v);
            },
            Instruction::Output { param } => {
                output.output(param.value(mem))
            },
            Instruction::JmpTrue { cond, dest } =>  {
                if cond.value(mem) != 0 {
                    pc.jump(dest.value(mem))
                }
            },
            Instruction::JmpFalse { cond, dest } => {
                if cond.value(mem) == 0 {
                    pc.jump(dest.value(mem))
                }
            },
            Instruction::Lt { lhs, rhs, dest } => {
                let result = lhs.value(mem) < rhs.value(mem);
                mem.set(dest, Integer::from(result));
            },
            Instruction::Eq { lhs, rhs, dest } => {
                let result = lhs.value(mem) == rhs.value(mem);
                mem.set(dest, Integer::from(result));
            },

            Instruction::Halt => return false,
        }

        true
    }
}

pub type Integer = i32;

pub fn parse_input(input: &str) -> impl Iterator<Item = Integer> + '_ {
    input.split(',')
        .map(|raw_number| raw_number.parse().expect("Invalid integer code"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part1(&code), 38_834);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 69_113_332);
    }
}
