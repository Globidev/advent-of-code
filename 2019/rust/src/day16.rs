use std::fmt::Debug;
use itertools::Itertools;
use rayon::prelude::*;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day16.txt");

pub fn day16() -> impl Debug {
    let sequence = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&sequence), part2(&sequence))
}

pub fn part1(sequence: &[i32]) -> i32 {
    let mut sequence = sequence.to_owned();

    for _phase in 0..100 {
        apply_phase(&mut sequence);
    }

    sequence.iter()
        .take(8)
        .copied()
        .fold1(|s, x| s * 10 + x)
        .expect("Empty input signal")
}

pub fn part2(sequence: &[i32]) -> i32 {
    // let sequence = parse_input("02935109699940807407585447034323").collect_vec();
    let message_offset = sequence.iter()
        .take(7)
        .copied()
        .fold1(|s, x| s * 10 + x)
        .expect("Empty input signal");

    let mut sequence = sequence.repeat(10_000);

    for _phase in 0..100 {
        println!("PHASE");
        apply_phase_2(&mut sequence, message_offset as usize);
    }

    sequence[message_offset as usize..].iter()
        .take(8)
        .copied()
        .fold1(|s, x| s * 10 + x)
        .expect("Empty input signal")
}

fn apply_phase(sequence: &mut [i32]) {
    for (idx, pattern) in patterns().take(sequence.len()).enumerate() {
        let h = Iterator::zip(sequence.iter(), pattern)
            .map(|(&x, y)| x * y)
            .sum::<i32>();

        sequence[idx] = h.abs() % 10;
    }
}

fn apply_phase_2(sequence: &mut [i32], offset: usize) {
    let s = (offset..sequence.len())
        .into_par_iter()
        .map(|idx| {
            let mut h = 0_i32;
            let mut start_idx = idx;
            let mut pos = true;

            while start_idx < sequence.len() {
                let end_idx = (start_idx + idx + 1).min(sequence.len());
                let chunk = &sequence[start_idx..end_idx];

                let sum = chunk.iter().copied()
                    .sum::<i32>();

                h += if pos { sum } else { -sum };
                pos = !pos;

                start_idx = end_idx + idx + 1; // skip 0
            }

            h.abs() % 10
        })
        .collect::<Vec<_>>();

    sequence[offset..].copy_from_slice(&s);
}

fn patterns() -> impl Iterator<Item = impl Iterator<Item = i32>> {
    (1..)
        .map(|repeat| {
            std::iter::repeat(0).take(repeat)
                .chain(std::iter::repeat(1).take(repeat))
                .chain(std::iter::repeat(0).take(repeat))
                .chain(std::iter::repeat(-1).take(repeat))
                .cycle()
                .skip(1)
        })
}

pub fn parse_input(input: &str) -> impl Iterator<Item = i32> + '_ {
    input.chars()
        .map(|c| c.to_digit(10).expect("Malformed digit") as _)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 18_933_364);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 28_872_305);
    }
}
