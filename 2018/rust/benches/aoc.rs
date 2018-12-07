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

    const SNEK_INPUT: &str = include_str!("../../inputs/day03-snek.txt");
    const SNEK_P1: usize = 118322;
    const SNEK_P2: u16 = 1178;

    const OPTIONCAT_INPUT: &str = include_str!("../../inputs/day03-optioncat.txt");
    const OPTIONCAT_P1: usize = 118322;
    const OPTIONCAT_P2: u16 = 1178;

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p1", move |b| b.iter(|| day03::part1(&input)));

    let input: Vec<_> = day03::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day03 p2", move |b| b.iter(|| day03::part2(&input)));

    let p1_funs = vec![
        Fun::new("Globi/Globi", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(GLOBI_INPUT).collect();
            assert_eq!(day03::globi::part1(&input), GLOBI_P1);
            b.iter(|| day03::globi::part1(&input))
        }),
        Fun::new("Optioncat/Globi", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(GLOBI_INPUT);
            assert_eq!(day03::optioncat::part1(&input), GLOBI_P1 as i32);
            b.iter(|| day03::optioncat::part1(&input))
        }),
        Fun::new("Globi/Snek", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(SNEK_INPUT).collect();
            assert_eq!(day03::globi::part1(&input), SNEK_P1);
            b.iter(|| day03::globi::part1(&input))
        }),
        Fun::new("Optioncat/Snek", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(SNEK_INPUT);
            assert_eq!(day03::optioncat::part1(&input), SNEK_P1 as i32);
            b.iter(|| day03::optioncat::part1(&input))
        }),
        Fun::new("Globi/Optioncat", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(OPTIONCAT_INPUT).collect();
            assert_eq!(day03::globi::part1(&input), OPTIONCAT_P1);
            b.iter(|| day03::globi::part1(&input))
        }),
        Fun::new("Optioncat/Optioncat", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(OPTIONCAT_INPUT);
            assert_eq!(day03::optioncat::part1(&input), OPTIONCAT_P1 as i32);
            b.iter(|| day03::optioncat::part1(&input))
        }),
    ];

    c.bench_functions("day03 p1 discord", p1_funs, ());

    let p2_funs = vec![
        Fun::new("Globi/Globi", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(GLOBI_INPUT).collect();
            assert_eq!(day03::globi::part2(&input), GLOBI_P2);
            b.iter(|| day03::globi::part2(&input))
        }),
        Fun::new("Snek/Globi", |b, _:&()| {
            let input: Vec<_> = day03::snek::parse_input(GLOBI_INPUT);
            assert_eq!(day03::snek::part2(&input), GLOBI_P2 as u32);
            b.iter(|| day03::snek::part2(&input))
        }),
        Fun::new("Optioncat/Globi", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(GLOBI_INPUT);
            assert_eq!(day03::optioncat::part2(&input), GLOBI_P2 as i32);
            b.iter(|| day03::optioncat::part2(&input))
        }),
        Fun::new("Globi/Snek", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(SNEK_INPUT).collect();
            assert_eq!(day03::globi::part2(&input), SNEK_P2);
            b.iter(|| day03::globi::part2(&input))
        }),
        Fun::new("Snek/Snek", |b, _:&()| {
            let input: Vec<_> = day03::snek::parse_input(SNEK_INPUT);
            assert_eq!(day03::snek::part2(&input), SNEK_P2 as u32);
            b.iter(|| day03::snek::part2(&input))
        }),
        Fun::new("Optioncat/Snek", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(SNEK_INPUT);
            assert_eq!(day03::optioncat::part2(&input), SNEK_P2 as i32);
            b.iter(|| day03::optioncat::part2(&input))
        }),
        Fun::new("Globi/Optioncat", |b, _:&()| {
            let input: Vec<_> = day03::globi::parse_input(OPTIONCAT_INPUT).collect();
            assert_eq!(day03::globi::part2(&input), OPTIONCAT_P2);
            b.iter(|| day03::globi::part2(&input))
        }),
        Fun::new("Snek/Optioncat", |b, _:&()| {
            let input: Vec<_> = day03::snek::parse_input(OPTIONCAT_INPUT);
            assert_eq!(day03::snek::part2(&input), OPTIONCAT_P2 as u32);
            b.iter(|| day03::snek::part2(&input))
        }),
        Fun::new("Optioncat/Optioncat", |b, _:&()| {
            let input: Vec<_> = day03::optioncat::read_input(OPTIONCAT_INPUT);
            assert_eq!(day03::optioncat::part2(&input), OPTIONCAT_P2 as i32);
            b.iter(|| day03::optioncat::part2(&input))
        }),
    ];

    c.bench_functions("day03 p2 discord", p2_funs, ());
}

fn day04(c: &mut Criterion) {
    use aoc_2018::day04;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day04.txt");
    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day04.txt");
    const GLOBI_P1: u32 = 106710;
    const GLOBI_P2: u32 = 10491;

    let input = day04::parse_input(GLOBI_INPUT);
    c.bench_function("day04 p1", move |b| b.iter(|| day04::part1(&input)));

    let input = day04::parse_input(GLOBI_INPUT);
    c.bench_function("day04 p2", move |b| b.iter(|| day04::part2(&input)));

    let parse_funs = vec![
        // Fun::new("Globi/Globi nom", |b, _:&()| b.iter(|| day04::parse_input(GLOBI_INPUT))),
        // Fun::new("Globi/Globi regex", |b, _:&()| b.iter(|| day04::parse_input_regex(GLOBI_INPUT_STR))),
        // Fun::new("Snek/Globi manual", |b, _:&()| b.iter(|| day04::parse_input_manual(GLOBI_INPUT_STR))),
        Fun::new("0e4ef622/Globi manual", |b, _:&()| b.iter(|| day04::oe4ef622::parse(GLOBI_INPUT))),
    ];

    c.bench_functions("day04 parse", parse_funs, ());

    let p1_funs = vec![
        Fun::new("Globi/Globi", |b, _:&()| {
            let input: Vec<_> = day04::globi::parse_input(GLOBI_INPUT);
            assert_eq!(day04::globi::part1(&input), GLOBI_P1);
            b.iter(|| day04::globi::part1(&input))
        }),
        Fun::new("0e4ef622/Globi", |b, _:&()| {
            let input: Vec<_> = day04::oe4ef622::parse(GLOBI_INPUT);
            assert_eq!(day04::oe4ef622::part1(&input), GLOBI_P1 as usize);
            b.iter(|| day04::oe4ef622::part1(&input))
        }),
    ];

    c.bench_functions("day04 p1 discord", p1_funs, ());

    let p2_funs = vec![
        Fun::new("Globi/Globi", |b, _:&()| {
            let input: Vec<_> = day04::globi::parse_input(GLOBI_INPUT);
            assert_eq!(day04::globi::part2(&input), GLOBI_P2);
            b.iter(|| day04::globi::part2(&input))
        }),
        Fun::new("0e4ef622/Globi", |b, _:&()| {
            let input: Vec<_> = day04::oe4ef622::parse(GLOBI_INPUT);
            assert_eq!(day04::oe4ef622::part2(&input), GLOBI_P2 as usize);
            b.iter(|| day04::oe4ef622::part2(&input))
        }),
    ];

    c.bench_functions("day04 p2 discord", p2_funs, ());
}

fn day05(c: &mut Criterion) {
    use aoc_2018::day05;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day05.txt");
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

criterion_group!(benches, day01, day02, day03, day04, day05, day06);
criterion_main!(benches);
