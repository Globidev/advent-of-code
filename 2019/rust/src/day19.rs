use std::fmt::Debug;
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}};
use std::array::IntoIter;
use rayon::prelude::*;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day19.txt");

pub fn day19() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> usize {
    beam_map(program, 50, 50)
        .filter(|&state| state == DroneState::PulledIn)
        .count()
}

pub fn part2(program: &[Int]) -> usize {
    let w = 1200_usize;
    let h = 1200_usize;

    let coordinates = (100..h)
        .into_par_iter()
        .flat_map(move |y| (100..w).into_par_iter().map(move |x| (x, y)));

    let (x, y) = coordinates
        .find_first(|&(x, y)| {
            let corners = [(x, y), (x + 99, y), (x, y + 99), (x + 99, y + 99)];
            IntoIter::new(corners)
                .all(|(x, y)| beam_state(program, x, y) == DroneState::PulledIn)
        })
        .expect("No place to put the ship");

    x * 10_000 + y
}

fn beam_state(program: &[Int], x: usize, y: usize) -> DroneState {
    let raw_beam_state = VirtualMachine::load(program)
        .input_iter(IntoIter::new([x as _, y as _]))
        .single_output()
        .run()
        .output()
        .expect("Did not get a beam state output");

    match raw_beam_state {
        0 => DroneState::Stationary,
        1 => DroneState::PulledIn,
        invalid => panic!("Invalid state output: {}", invalid),
    }
}

fn beam_map(program: &[Int], width: usize, height: usize) -> impl ParallelIterator<Item = DroneState> + '_ {
    let coordinates = (0..height)
        .into_par_iter()
        .flat_map(move |y| (0..width).into_par_iter().map(move |x| (x, y)));

    coordinates
        .map(move |(x, y)| beam_state(program, x, y))
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum DroneState {
    Stationary,
    PulledIn,
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

        assert_eq!(part1(&code), 152);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 10_730_411);
    }
}
