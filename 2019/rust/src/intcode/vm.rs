use std::borrow::Cow;
use super::{Int, io::IO, memory::Memory, cpu::{CPU, ExecResult}};

pub struct VirtualMachine<W: IO> {
    world: W,
    pub memory: Memory,
    cpu: CPU,
}

impl<W: IO> VirtualMachine<W> {
    pub fn new<'a>(program: impl Into<Cow<'a, [Int]>>, world: W) -> Self {
        Self {
            world,
            memory: Memory::load(program.into()),
            cpu: CPU::default(),
        }
    }

    pub fn run(mut self) -> Vec<Int> {
        let world = &mut self.world;
        let mem = &mut self.memory;

        loop {
            let exec_result = self.cpu.exec_next(mem, world);

            if let ExecResult::Halt = exec_result {
                break
            }
        }

        self.memory.cells
    }
}
