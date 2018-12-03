#[macro_use]
extern crate criterion;

use criterion::{Criterion, Fun};

fn day01(c: &mut Criterion) {
    use aoc_2018::day01;

    const GLOBI_INPUT: &str = include_str!("../../inputs/day01.txt");

    let input: Vec<_> = day01::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));

    use std::collections::{HashSet, BTreeSet};
    use hashbrown::HashSet as SwissTable;

    let p2_funs = vec![
        Fun::new("HashSet",    |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), HashSet::new()))),
        Fun::new("BTreeSet",   |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), BTreeSet::new()))),
        Fun::new("SwissTable", |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), SwissTable::new()))),
    ];

    let input: Vec<_> = day01::parse_input(GLOBI_INPUT).collect();
    c.bench_functions("day01 p2", p2_funs, input);
}

fn day02(c: &mut Criterion) {
    use aoc_2018::day02;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day02.txt");

    let input: Vec<_> = day02::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day02 p1", move |b| b.iter(|| day02::part1(&input)));

    let input: Vec<_> = day02::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day02 p2", move |b| b.iter(|| day02::part2_set(&input)));
}

fn day03(c: &mut Criterion) {
    use aoc_2018::day03;

    const GLOBI_INPUT: &str = include_str!("../../inputs/day03.txt");
    const GLOBI_P1: usize = 110891;
    const GLOBI_P2: u16 = 297;

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));
}

criterion_group!(benches, day01, day02, day03);
criterion_main!(benches);
