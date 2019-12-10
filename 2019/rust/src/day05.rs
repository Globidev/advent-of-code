use std::fmt::Debug;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day05.txt");

pub fn day05() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR);

    (part1(program.clone()), part2(program))
}

pub fn part1(mut program: Program) -> Integer {
    const SHIP_AIR_CONDITIONER_UNIT_ID: Integer = 1;

    program.seed_with(vec![SHIP_AIR_CONDITIONER_UNIT_ID]);
    // program.seed_with(vec![245182, 790572]);
    program.run();

    *program.output.last()
        .expect("No output!")
}

pub fn part2(mut program: Program) -> Integer {
    // 42
    const SHIP_THERMAL_RADIATOR_CONTROLLER_ID: Integer = 5;

    program.seed_with(vec![SHIP_THERMAL_RADIATOR_CONTROLLER_ID]);
    program.run();

    *program.output.last()
        .expect("No output!")
}

#[derive(Clone)]
pub struct Program {
    memory: Memory,
    input: Vec<Integer>,
    output: Vec<Integer>
}

impl Program {
    fn load_from_code(code: Vec<Integer>) -> Self {
        Self {
            memory: Memory::load(code.into_boxed_slice()),
            input: Vec::new(),
            output: Vec::new(),
        }
    }

    fn seed_with(&mut self, input: Vec<Integer>) {
        self.input = input;
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

        // for instr in pc.decode_instr(mem) {
        //     // dbg!(&instr);
        //     if !instr.execute(mem, &mut self.input, &mut self.output, &mut pc) {
        //         break
        //     }
        // }
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
    // #[inline]
    fn execute(self, mem: &mut Memory, inputs: &mut Vec<Integer>, output: &mut Vec<Integer>, pc: &mut Pc) -> bool {
        match self {
            Instruction::Add { lhs, rhs, dest } => {
                mem.set(dest, lhs.value(mem) + rhs.value(mem));
            },
            Instruction::Mul { lhs, rhs, dest } => {
                mem.set(dest, lhs.value(mem) * rhs.value(mem));
            },
            Instruction::Input { dest } => {
                mem.set(dest, inputs.remove(0));
            },
            Instruction::Output { param } => {
                output.push(param.value(mem))
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

pub fn parse_input(input: &str) -> Program {
    let code = input.split(',')
        .map(|raw_number| raw_number.parse().expect("Invalid integer code"))
        .collect();

    Program::load_from_code(code)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let program = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(program), 9_006_673);
    }

    #[test]
    fn p2() {
        let program = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(program), 3_629_692);
    }
}
