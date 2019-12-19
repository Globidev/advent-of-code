use super::Int;
use super::memory::Memory;
use num::Integer;
use typenum::{Unsigned, U0, U1, U2, U3};
use arraytools::ArrayTools;

#[derive(Debug)]
pub enum Instruction {
    Add(Params<U3>),
    Mul(Params<U3>),
    Input(Params<U1>),
    Output(Params<U1>),
    JmpTrue(Params<U2>),
    JmpFalse(Params<U2>),
    CmpLt(Params<U3>),
    CmpEq(Params<U3>),
    RelBase(Params<U1>),
    Halt(Params<U0>),
}

use Instruction::*;

const MAX_OP_SIZE: usize = 4;

impl Instruction {
    pub fn decode(&[op_descriptor, params @ ..]: &[Int; MAX_OP_SIZE]) -> (Self, usize) {
        let (mut param_codes, op_code) = (op_descriptor as u16).div_rem(&100);

        let next_param = |idx| {
            let (next_param_codes, param_code) = param_codes.div_rem(&10);
            param_codes = next_param_codes;
            Param::from_code_and_value(param_code, params[idx])
        };

        macro_rules! decode_ops {
            ($($code:literal => $instr:ident),*) => {
                match op_code {
                    $($code => {
                        const PARAM_COUNT: usize = instruction_param_count(&$instr);
                        let params = <[_; PARAM_COUNT]>::indices().map(next_param);
                        ($instr(params), PARAM_COUNT + 1)
                    }),*,
                    unknown => unimplemented!("Unknown op code: {}", unknown),
                }
            }
        }

        decode_ops!(
            1  => Add,
            2  => Mul,
            3  => Input,
            4  => Output,
            5  => JmpTrue,
            6  => JmpFalse,
            7  => CmpLt,
            8  => CmpEq,
            9  => RelBase,
            99 => Halt
        )
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Param {
    Position { addr: Int },
    Immediate { value: Int },
    Relative { base_addr: Int },
}

impl Param {
    fn from_code_and_value(code: u16, value: Int) -> Self {
        match code {
            0 => Param::Position { addr: value },
            1 => Param::Immediate { value },
            2 => Param::Relative { base_addr: value },
            unknown => unreachable!("Unknown param code: {}", unknown)
        }
    }

    pub fn get(self, mem: &mut Memory) -> Int {
        match self {
            Param::Position { addr } => *mem.get(addr),
            Param::Immediate { value } => value,
            Param::Relative { base_addr } => *mem.get_relative(base_addr),
        }
    }

    pub fn get_mut(self, mem: &mut Memory) -> &mut Int {
        match self {
            Param::Position { addr } => mem.get(addr),
            Param::Immediate { .. } => unreachable!("Immediate as destination"),
            Param::Relative { base_addr } => mem.get_relative(base_addr),
        }
    }
}

type Params<N: Unsigned> = [Param; N::USIZE];

const fn instruction_param_count<T>(_: &impl Fn(T) -> Instruction) -> usize {
    std::mem::size_of::<T>() / std::mem::size_of::<Param>()
}

#[cfg(test)]
mod tests {
    use assert_matches::assert_matches;
    use super::*;

    #[test]
    fn ops() {
        assert_matches!(
            Instruction::decode(&[1, 2, 3, 4]).0,
            Instruction::Add([
                Param::Position { addr: 2 },
                Param::Position { addr: 3 },
                Param::Position { addr: 4 },
            ])
        )
    }
}
