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

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));
}

fn day04(c: &mut Criterion) {
    use aoc_2018::day04;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day04.txt");

    let input = day04::parse_input(GLOBI_INPUT);
    c.bench_function("day04 p1", move |b| b.iter(|| day04::part1(&input)));

    let input = day04::parse_input(GLOBI_INPUT);
    c.bench_function("day04 p2", move |b| b.iter(|| day04::part2(&input)));
}

fn day05(c: &mut Criterion) {
    use aoc_2018::day05;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day05.txt");

    c.bench_function("day05 p1", move |b| b.iter(|| day05::part1(GLOBI_INPUT_STR)));
    c.bench_function("day05 p2", move |b| b.iter(|| day05::part2(GLOBI_INPUT_STR)));
}

fn day06(c: &mut Criterion) {
    use aoc_2018::day06;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day06.txt");

    let coords: Vec<_> = day06::parse_coordinates(GLOBI_INPUT_STR).collect();
    c.bench_function("day06 p1", move |b| b.iter(|| day06::part1(&coords)));
    let coords: Vec<_> = day06::parse_coordinates(GLOBI_INPUT_STR).collect();
    c.bench_function("day06 p2", move |b| b.iter(|| day06::part2(&coords)));
}

fn day07(c: &mut Criterion) {
    use aoc_2018::day07;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day07.txt");

    let coords: Vec<_> = day07::parse_relations(GLOBI_INPUT).collect();
    c.bench_function("day07 p1", move |b| b.iter(|| day07::part1(coords.iter().cloned())));
    let coords: Vec<_> = day07::parse_relations(GLOBI_INPUT).collect();
    c.bench_function("day07 p2", move |b| b.iter(|| day07::part2(coords.iter().cloned())));
}

fn day08(c: &mut Criterion) {
    use aoc_2018::day08;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day08.txt");

    let data: Vec<_> = day08::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day08 p1", move |b| b.iter(|| day08::part1(&data)));
    let data: Vec<_> = day08::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day08 p2", move |b| b.iter(|| day08::part2(&data)));
}

criterion_group!(benches, day01, day02, day03, day04, day05, day06, day07, day08);
criterion_main!(benches);
