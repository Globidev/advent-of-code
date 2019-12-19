use super::Int;

pub trait IO {
    fn input(&mut self) -> Int;
    fn output(&mut self, value: Int);
}

pub trait Input {
    fn input(&mut self) -> Int;
}

impl<T: Input> Input for &mut T {
    fn input(&mut self) -> Int { Input::input(*self) }
}

pub trait Output {
    fn output(&mut self, value: Int);
}

impl<T: Output> Output for &mut T {
    fn output(&mut self, value: Int) { Output::output(*self, value) }
}

impl<T: Input + Output> IO for T {
    fn input(&mut self) -> Int { Input::input(self) }
    fn output(&mut self, value: Int) { Output::output(self, value) }
}

pub mod ext {
    pub struct Split<I, O>(pub I, pub O);

    impl<I: Input, O: Output> IO for Split<I, O> {
        fn input(&mut self) -> Int {
            self.0.input()
        }
        fn output(&mut self, value: Int) {
            self.1.output(value)
        }
    }
    pub struct Pure;

    impl Input for Pure {
        fn input(&mut self) -> Int { panic!("No value available") }
    }

    impl Output for Pure {
        fn output(&mut self, _value: Int) { }
    }

    pub struct Iter<I>(pub I);

    impl<I: Iterator<Item = Int>> Input for Iter<I> {
        fn input(&mut self) -> Int { self.0.next().expect("No more values available") }
    }

    use super::*;
    #[derive(Default, Debug)]
    pub struct SingleOutput(Option<Int>);

    impl SingleOutput {
        pub fn new() -> Self {
            Self::default()
        }

        pub fn get(&self) -> Option<Int> {
            self.0
        }
    }

    impl Output for SingleOutput {
        fn output(&mut self, value: Int) { self.0 = Some(value) }
    }

    use std::sync::mpsc;

    impl Input for mpsc::Receiver<Int> {
        fn input(&mut self) -> Int {
            self.recv().expect("Failed to recv value")
        }
    }


    impl Output for mpsc::Sender<Int> {
        fn output(&mut self, value: Int) {
            self.send(value).expect("Failed to send value")
        }
    }
}
