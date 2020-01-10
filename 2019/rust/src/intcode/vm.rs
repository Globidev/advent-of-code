use std::borrow::Cow;
use super::{Int, io::{IO, Input, Output, ext::{Pure, Split, Iter, SingleOutput}}, memory::Memory, cpu::{CPU, ExecResult}};
use std::iter::{once, Once};

pub struct VirtualMachine<D> {
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
}

pub struct UnboundedDriver;

impl VirtualMachine<UnboundedDriver> {
    pub fn load<'a>(program: impl Into<Cow<'a, [Int]>>) -> VMBuilderProgram<'a> {
        VMBuilderProgram(program.into())
    }
}

pub struct VMBuilderProgram<'a>(Cow<'a, [Int]>);
pub struct VMBuilderProgramInput<'a, I: Input>(Cow<'a, [Int]>, I);
pub struct VMBuilderProgramOutput<'a, O: Output>(Cow<'a, [Int]>, O);
pub struct VMBuilderProgramIO<'a, D: IO>(Cow<'a, [Int]>, D);

impl<'a> VMBuilderProgram<'a> {
    pub fn driver<D: IO>(self, driver: D) -> VMBuilderProgramIO<'a, D> {
        VMBuilderProgramIO(self.0, driver)
    }

    pub fn input_driver<I: Input>(self, driver: I) -> VMBuilderProgramInput<'a, I> {
        VMBuilderProgramInput(self.0, driver)
    }

    pub fn output_driver<O: Output>(self, driver: O) -> VMBuilderProgramOutput<'a, O> {
        VMBuilderProgramOutput(self.0, driver)
    }

    pub fn with_driver<D: IO + Default>(self) -> VMBuilderProgramIO<'a, D> {
        self.driver(D::default())
    }

    pub fn with_input_driver<I: Input + Default>(self) -> VMBuilderProgramInput<'a, I> {
        self.input_driver(I::default())
    }

    pub fn with_output_driver<O: Output + Default>(self) -> VMBuilderProgramOutput<'a, O> {
        self.output_driver(O::default())
    }

    pub fn input_iter<I: Iterator<Item = Int>>(self, iter: I) -> VMBuilderProgramInput<'a, Iter<I>> {
        self.input_driver(Iter(iter))
    }

    pub fn input_once(self, value: Int) -> VMBuilderProgramInput<'a, Iter<Once<Int>>> {
        self.input_iter(once(value))
    }
}

impl<'a, I: Input> VMBuilderProgramInput<'a, I> {
    pub fn output_driver<O: Output>(self, driver: O) -> VMBuilderProgramIO<'a, Split<I, O>> {
        VMBuilderProgramIO(self.0, Split(self.1, driver))
    }

    pub fn single_output(self) -> VMBuilderProgramIO<'a, Split<I, SingleOutput>> {
        self.output_driver(SingleOutput::new())
    }
}

impl<'a, O: Output> VMBuilderProgramOutput<'a, O> {
    pub fn input_driver<I: Input>(self, driver: I) -> VMBuilderProgramIO<'a, Split<I, O>> {
        VMBuilderProgramIO(self.0, Split(driver, self.1))
    }
}

impl VMBuilder<Pure> for VMBuilderProgram<'_> {
    fn build(self) -> VirtualMachine<Pure> {
        VirtualMachine::new(self.0, Pure)
    }
}

impl<D: IO> VMBuilder<D> for VMBuilderProgramIO<'_, D> {
    fn build(self) -> VirtualMachine<D> {
        VirtualMachine::new(self.0, self.1)
    }
}

impl<I: Input> VMBuilder<Split<I, Pure>> for VMBuilderProgramInput<'_, I> {
    fn build(self) -> VirtualMachine<Split<I, Pure>> {
        self.output_driver(Pure).build()
    }
}

impl<O: Output> VMBuilder<Split<Pure, O>> for VMBuilderProgramOutput<'_, O> {
    fn build(self) -> VirtualMachine<Split<Pure, O>> {
        self.input_driver(Pure).build()
    }
}

pub trait VMBuilder<D: IO>: Sized {
    fn build(self) -> VirtualMachine<D>;

    fn run(self) -> EndRunState<D> {
        self.build()
            .run()
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
