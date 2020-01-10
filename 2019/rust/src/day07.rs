use std::fmt::Debug;
use std::thread;
use std::sync::mpsc::channel;
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}, io::ext::Split};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day07.txt");

pub fn day07() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> Int {
    (0..=4)
        .permutations(5)
        .map(|settings| run_amplifiers(program, &settings))
        .max()
        .expect("No settings")
}

pub fn part2(program: &[Int]) -> Int {
    (5..=9)
        .permutations(5)
        .map(|settings| run_amplifiers_feedback_loop(program, &settings))
        .max()
        .expect("No settings")
}

fn run_amplifiers(program: &[Int], settings: &[Int]) -> Int {
    let (mut tx, mut rx) = channel();
    let init_tx = tx.clone();

    let amplifiers = settings.iter()
        .map(|&setting| {
            let (next_tx, next_rx) = channel();
            tx.send(setting).expect("Failed to send phase setting");
            tx = next_tx.clone();
            let rx = std::mem::replace(&mut rx, next_rx);

            VirtualMachine::load(program)
                .input_driver(rx)
                .output_driver(next_tx)
                .build()
        });

    let handles = amplifiers
        .map(|amp| thread::spawn(move || amp.run()))
        .collect_vec();

    init_tx.send(0).expect("Failed to send initial input value");

    let result = rx.recv().expect("Failed to get output");

    for handle in handles {
        handle.join().expect("Failed to join amplifiers");
    }

    result
}

fn run_amplifiers_feedback_loop(program: &[Int], settings: &[Int]) -> Int {
    let (mut tx, mut rx) = channel();
    let init_tx = tx.clone();

    let amplifiers = settings[1..].iter()
        .map(|&setting| {
            let (next_tx, next_rx) = channel();
            tx.send(setting).expect("Failed to send phase setting");
            tx = next_tx.clone();
            let rx = std::mem::replace(&mut rx, next_rx);
            VirtualMachine::load(program)
                .input_driver(rx)
                .output_driver(next_tx)
                .build()
        })
        .collect_vec();

    let first_amp = VirtualMachine::load(program)
        .input_driver(rx)
        .output_driver(init_tx)
        .build();

    tx.send(settings[0]).expect("Failed to send phase setting");
    tx.send(0).expect("Failed to initial input value");

    let handles = std::iter::once(first_amp).chain(amplifiers.into_iter())
        .map(|amp| thread::spawn(move || amp.run()))
        .collect_vec();

    let mut amplifiers = handles.into_iter()
        .map(|handle| handle.join().expect("Failed to join amplifiers"))
        .collect_vec();

    amplifiers.first_mut()
        .expect("No amplifiers")
        .input().recv()
        .expect("Failed to get output")
}

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

        assert_eq!(part1(&code), 38_834);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 69_113_332);
    }
}
