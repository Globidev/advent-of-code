use super::{Int, io::IO, memory::Memory, ops::Instruction};

#[derive(Default)]
pub struct CPU {
    pc: usize,
}

impl CPU {
    pub fn exec_next(&mut self, mem: &mut Memory, world: &mut impl IO) -> ExecResult {
        let instr_data = mem.read_4(self.pc);

        let (instr, size) = Instruction::decode(instr_data);
        self.pc += size;

        match instr {
            Instruction::Add([lhs, rhs, dest]) => {
                let result = lhs.get(mem) + rhs.get(mem);
                *dest.get_mut(mem) = result;
            },
            Instruction::Mul([lhs, rhs, dest]) => {
                let result = lhs.get(mem) * rhs.get(mem);
                *dest.get_mut(mem) = result;
            },
            Instruction::Input([param]) => {
                *param.get_mut(mem) = world.input();
            },
            Instruction::Output([param]) => {
                world.output(param.get(mem));
            },
            Instruction::JmpTrue([cond, dest]) => {
                if cond.get(mem) != 0 {
                    self.pc = dest.get(mem) as usize;
                }
            },
            Instruction::JmpFalse([cond, dest]) => {
                if cond.get(mem) == 0 {
                    self.pc = dest.get(mem) as usize;
                }
            },
            Instruction::CmpLt([lhs, rhs, dest]) => {
                let result = lhs.get(mem) < rhs.get(mem);
                *dest.get_mut(mem) = Int::from(result);
            },
            Instruction::CmpEq([lhs, rhs, dest]) => {
                let result = lhs.get(mem) == rhs.get(mem);
                *dest.get_mut(mem) = Int::from(result);
            },
            Instruction::RelBase(_) => unimplemented!(),
            Instruction::Halt(_) => return ExecResult::Halt,
        }

        ExecResult::Ok
    }
}

pub enum ExecResult {
    Ok,
    Halt,
}
