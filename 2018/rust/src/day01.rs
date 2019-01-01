use std::collections::{HashSet, BTreeSet};
use std::hash::Hash;
use std::mem;

use hashbrown::HashSet as SwissTable;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day01.txt");

pub fn day01() -> (i32, i32) {
    let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (part1(&input), part2(&input))
}

pub fn part1(input: &[i32]) -> i32 {
    input.iter().sum()
}

pub fn part2(input: &[i32]) -> i32 {
    part2_impl(input, SwissTable::new())
}

pub fn part2_impl(input: &[i32], mut scanned_freqs: impl Set<i32>) -> i32 {
    looped_frequencies(input)
        .find(|&freq| !scanned_freqs.insert(freq))
        .expect("Empty input")
}

fn looped_frequencies(input: &[i32]) -> impl Iterator<Item = i32> + '_ {
    input.iter()
        .cycle()
        .scan(0, |freq, delta| Some(mem::replace(freq, *freq + delta)))
}

pub fn parse_input(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.lines()
        .map(|s| s.parse().expect("Badly formatted number in the input"))
}

pub trait Set<T> {
    fn insert(&mut self, value: T) -> bool;
}

impl<T: Eq + Hash> Set<T> for HashSet<T> {
    fn insert(&mut self, value: T) -> bool { HashSet::insert(self, value) }
}

impl<T: Eq + Hash + Ord> Set<T> for BTreeSet<T> {
    fn insert(&mut self, value: T) -> bool { BTreeSet::insert(self, value) }
}

impl<T: Eq + Hash> Set<T> for SwissTable<T> {
    fn insert(&mut self, value: T) -> bool { SwissTable::insert(self, value) }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 442);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 59908);
    }
}
