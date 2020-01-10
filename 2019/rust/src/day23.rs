use std::fmt::{Debug};
use itertools::Itertools;
use crate::intcode::{Int, vm::VirtualMachine, io::{Input, Output}};
use std::{sync::{Arc, Mutex}};
use std::collections::{VecDeque, HashSet};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day23.txt");

pub fn day23() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    let network = Arc::new(Mutex::new(Network::new(50)));

    let vms = (0..50)
        .map(|addr| VirtualMachine::new(program, BoundNet::new(network.clone(), addr)));

    let handles = vms.map(|vm| std::thread::spawn(|| vm.run() ))
        .collect_vec();

    for handle in handles {
        handle.join().expect("Failed to run vm");
    }

    let net = network.lock().unwrap();

    net.nat
        .expect("Did not get a message on addr 255")
        .y
}

pub fn part2(_program: &[Int]) -> Int {
    42
}

#[derive(Debug)]
struct Network {
    queue: Vec<VecDeque<Int>>,
    nat: Option<Message>,
    nat_values_sent: HashSet<Int>,
    input_requests_while_empty: usize,
}

impl Network {
    fn new(size: usize) -> Self {
        Self {
            queue: (0..size)
                .map(|x|
                    std::iter::once(x as _)
                    .collect()
                )
                .collect(),
            nat: None,
            nat_values_sent: HashSet::new(),
            input_requests_while_empty: 0,
        }
    }
}

struct BoundNet {
    net: Arc<Mutex<Network>>,
    addr: Int,
    input_state: InputState,
}

impl BoundNet {
    fn new(net: Arc<Mutex<Network>>, addr: Int) -> Self {
        Self {
            net,
            addr,
            input_state: InputState::Dest,
        }
    }
}

impl Output for BoundNet {
    fn output(&mut self, value: Int) {
        self.net.lock().unwrap()
            .input_requests_while_empty = 0;

        let next_state = match self.input_state {
            InputState::Dest => InputState::X { addr: value },
            InputState::X { addr } => InputState::Y { addr, x: value },
            InputState::Y { addr, x } => {
                let y = value;

                let mut net = self.net.lock().unwrap();

                if addr == 255 {
                    net.nat = Some(Message { x, y });
                } else {
                    let queue = &mut net.queue[addr as usize];

                    queue.push_back(x);
                    queue.push_back(y);
                }

                InputState::Dest
            },
        };

        self.input_state = next_state;
    }
}

impl Input for BoundNet {
    fn input(&mut self) -> Int {
        let mut net = self.net.lock().unwrap();

        if net.queue.iter().all(|q| q.is_empty()) {
            if net.input_requests_while_empty >= 100000 {
                if let Some(Message { x, y }) = net.nat.take() {
                    if !net.nat_values_sent.insert(y) {
                        dbg!(y);
                    }
                    net.queue[0].push_back(x);
                    net.queue[0].push_back(y);
                }
            } else {
                net.input_requests_while_empty += 1;
            }
        }

        net.queue[self.addr as usize]
            .pop_front()
            .unwrap_or(-1)
    }
}

enum InputState {
    Dest,
    X { addr: Int },
    Y { addr: Int, x: Int },
}

#[derive(Debug, Clone, Copy)]
struct Message { x: Int, y: Int }

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

        assert_eq!(part2(&code), 11_462);
    }
}
