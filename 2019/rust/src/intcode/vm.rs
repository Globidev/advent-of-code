use std::borrow::Cow;
use super::{Int, io::{IO, Input, Output, ext::{Pure, Split, Iter, SingleOutput}}, memory::Memory, cpu::{CPU, ExecResult}};
use std::marker::PhantomData;
use std::iter::{once, Once};

pub struct VirtualMachine<D: IO> {
    driver: D,
    memory: Memory,
    cpu: CPU,
}

impl<D: IO> VirtualMachine<D> {
    pub fn new<'a>(program: impl Into<Cow<'a, [Int]>>, driver: D) -> Self {
        Self {
            driver,
            memory: Memory::load(program.into()),
            cpu: CPU::default(),
        }
    }

    pub fn run(mut self) -> EndRunState<D> {
        let driver = &mut self.driver;
        let mem = &mut self.memory;

        loop {
            let exec_result = self.cpu.exec_next(mem, driver);

            if let ExecResult::Halt = exec_result {
                break
            }
        }

        EndRunState {
            memory: self.memory.cells,
            driver: self.driver,
        }
    }

    pub fn builder() -> VMBuilder<D> {
        VMBuilder(PhantomData)
    }
}

pub struct VMBuilder<D: IO>(PhantomData<D>);
pub struct VMBuilderProgram<'a, D: IO>(Cow<'a, [Int]>, PhantomData<D>);
pub struct VMBuilderProgramInput<'a, I: Input, O: Output>(Cow<'a, [Int]>, I, PhantomData<O>);
pub struct VMBuilderProgramOutput<'a, I: Input, O: Output>(Cow<'a, [Int]>, O, PhantomData<I>);
pub struct VMBuilderProgramIO<'a, D: IO>(Cow<'a, [Int]>, D);

impl<D: IO> VMBuilder<D> {
    pub fn load<'a>(self, program: impl Into<Cow<'a, [Int]>>) -> VMBuilderProgram<'a, D> {
        VMBuilderProgram(program.into(), PhantomData)
    }
}

impl VMBuilderProgram<'_, Pure> {
    pub fn build(self) -> VirtualMachine<Pure> {
        VirtualMachine::new(self.0, Pure)
    }
}

impl<'a, I: Input, O: Output> VMBuilderProgram<'a, Split<I, O>> {
    pub fn input_driver(self, driver: I) -> VMBuilderProgramInput<'a, I, O> {
        VMBuilderProgramInput(self.0, driver, PhantomData)
    }

    pub fn output_driver(self, driver: O) -> VMBuilderProgramOutput<'a, I, O> {
        VMBuilderProgramOutput(self.0, driver, PhantomData)
    }
}

impl<'a, D: IO> VMBuilderProgram<'a, D> {
    pub fn driver(self, driver: D) -> VMBuilderProgramIO<'a, D> {
        VMBuilderProgramIO(self.0, driver)
    }
}

impl<'a, I: Iterator<Item = Int>, O: Output> VMBuilderProgram<'a, Split<Iter<I>, O>> {
    pub fn input_iter(self, iter: I) -> VMBuilderProgramInput<'a, Iter<I>, O> {
        self.input_driver(Iter(iter))
    }
}

impl<'a, O: Output> VMBuilderProgram<'a, Split<Iter<Once<Int>>, O>> {
    pub fn input_once(self, value: Int) -> VMBuilderProgramInput<'a, Iter<Once<Int>>, O> {
        self.input_iter(once(value))
    }
}

impl<'a, I: Input, O: Output> VMBuilderProgramInput<'a, I, O> {
    pub fn output_driver(self, driver: O) -> VMBuilderProgramIO<'a, Split<I, O>> {
        VMBuilderProgramIO(self.0, Split(self.1, driver))
    }
}

impl<'a, I: Input> VMBuilderProgramInput<'a, I, SingleOutput> {
    pub fn single_output(self) -> VMBuilderProgramIO<'a, Split<I, SingleOutput>> {
        self.output_driver(SingleOutput::new())
    }
}

impl<'a, I: Input, O: Output> VMBuilderProgramOutput<'a, I, O> {
    pub fn input_driver(self, driver: I) -> VMBuilderProgramIO<'a, Split<I, O>> {
        VMBuilderProgramIO(self.0, Split(driver, self.1))
    }
}

impl<'a, O: Output> VMBuilderProgramOutput<'a, Pure, O> {
    pub fn build(self) -> VirtualMachine<Split<Pure, O>> {
        self.input_driver(Pure).build()
    }
}

impl<D: IO> VMBuilderProgramIO<'_, D> {
    pub fn build(self) -> VirtualMachine<D> {
        VirtualMachine::new(self.0, self.1)
    }
}

pub struct EndRunState<D: IO> {
    pub memory: Vec<Int>,
    pub driver: D
}

impl<I: Input, O: Output> EndRunState<Split<I, O>> {
    pub fn input(&mut self) -> &mut I {
        &mut self.driver.0
    }

    pub fn output(&mut self) -> &mut O {
        &mut self.driver.1
    }

    pub fn into_input(self) -> I {
        self.driver.0
    }

    pub fn into_output(self) -> O {
        self.driver.1
    }
}
