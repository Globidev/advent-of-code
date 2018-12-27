#[macro_use]
extern crate criterion;

use criterion::{Criterion, Fun};

fn day01(c: &mut Criterion) {
    use aoc_2018::day01;

    const GLOBI_INPUT: &str = include_str!("../../inputs/day01.txt");

    let input: Vec<_> = day01::parse_input(GLOBI_INPUT).collect();
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));
}

fn day01_2(c: &mut Criterion) {
    use aoc_2018::day01;

    const GLOBI_INPUT: &str = include_str!("../../inputs/day01.txt");

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

fn day09(c: &mut Criterion) {
    use aoc_2018::day09;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day09.txt");

    let config = day09::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day09 p1", move |b| b.iter(|| day09::part1(&config)));
}

fn day09_2(c: &mut Criterion) {
    use aoc_2018::day09;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day09.txt");

    let config = day09::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day09 p2", move |b| b.iter(|| day09::part2(&config)));
}

fn day10(c: &mut Criterion) {
    use aoc_2018::day10;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day10.txt");

    let points: Vec<_> = day10::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day10 p1", move |b| b.iter(|| day10::part1(&points)));

    let points: Vec<_> = day10::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day10 p2", move |b| b.iter(|| day10::part2(&points)));
}

fn day11(c: &mut Criterion) {
    use aoc_2018::day11;

    const GLOBI_INPUT: u16 = 1723;

    c.bench_function("day11 p1", move |b| b.iter(|| day11::part1(GLOBI_INPUT)));
}

fn day11_2(c: &mut Criterion) {
    use aoc_2018::day11;

    const GLOBI_INPUT: u16 = 1723;

    c.bench_function("day11 p2", move |b| b.iter(|| day11::part2(GLOBI_INPUT)));
}

fn day12(c: &mut Criterion) {
    use aoc_2018::day12;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day12.txt");

    let (state, rules) = day12::parse_input(GLOBI_INPUT);
    c.bench_function("day12 p1", move |b| b.iter(|| day12::part1(&state, &rules)));

    let (state, rules) = day12::parse_input(GLOBI_INPUT);
    c.bench_function("day12 p2", move |b| b.iter(|| day12::part2(&state, &rules)));
}

fn day13(c: &mut Criterion) {
    use aoc_2018::day13;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day13.txt");

    let (world, trains) = day13::parse_input(GLOBI_INPUT);
    c.bench_function("day13 p1", move |b| b.iter(|| day13::part1(&world, &trains)));

    let (world, trains) = day13::parse_input(GLOBI_INPUT);
    c.bench_function("day13 p2", move |b| b.iter(|| day13::part2(&world, &trains)));
}

fn day14(c: &mut Criterion) {
    use aoc_2018::day14;

    const GLOBI_INPUT: usize = 920831;
    const GLOBI_INPUT_AS_STR: &str = "920831";

    c.bench_function("day14 p1", move |b| b.iter(|| day14::part1(GLOBI_INPUT)));
    c.bench_function("day14 p2", move |b| b.iter(|| day14::part2(GLOBI_INPUT_AS_STR)));
}

fn day15(c: &mut Criterion) {
    use aoc_2018::day15;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day15.txt");

    let world = day15::parse_input(GLOBI_INPUT);
    c.bench_function("day15 p1", move |b| b.iter(|| day15::part1(&world)));
    let world = day15::parse_input(GLOBI_INPUT);
    c.bench_function("day15 p2", move |b| b.iter(|| day15::part2(&world)));
}

fn day16(c: &mut Criterion) {
    use aoc_2018::day16;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day16.txt");

    let (samples, _) = day16::parse_input(GLOBI_INPUT);
    let packed = day16::vectorize(&samples);
    // c.bench_function("day16 p1", move |b| b.iter(|| day16::part1_vectorized(&samples)));

    let p1_funs = vec![
        Fun::new("Normal",     |b, (samples, _): &(Vec<_>, _)| b.iter(|| day16::part1(samples))),
        Fun::new("Vectorized", |b, (_,  packed): &(Vec<_>, _)| b.iter(|| day16::part1_vectorized(packed))),
    ];

    c.bench_functions("day16 p1", p1_funs, (samples, packed));

    let (samples, program) = day16::parse_input(GLOBI_INPUT);
    c.bench_function("day16 p2", move |b| b.iter(|| day16::part2(&samples, &program)));
}

fn day17(c: &mut Criterion) {
    use aoc_2018::day17;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day17.txt");

    let ranges: Vec<_> = day17::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day17 p1", move |b| b.iter(|| day17::part1(&ranges)));
    let ranges: Vec<_> = day17::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day17 p2", move |b| b.iter(|| day17::part2(&ranges)));
}

fn day18(c: &mut Criterion) {
    use aoc_2018::day18;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day18.txt");

    let world = day18::parse_input(GLOBI_INPUT);
    c.bench_function("day18 p1", move |b| b.iter(|| day18::part1(&world)));
    let world = day18::parse_input(GLOBI_INPUT);
    c.bench_function("day18 p2", move |b| b.iter(|| day18::part2(&world)));
}

fn day19(c: &mut Criterion) {
    use aoc_2018::day19;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day19.txt");

    let (pc_idx, instrs) = day19::parse_input(GLOBI_INPUT);
    c.bench_function("day19 p1", move |b| b.iter(|| day19::part1(pc_idx, &instrs)));
    let (pc_idx, instrs) = day19::parse_input(GLOBI_INPUT);
    c.bench_function("day19 p2", move |b| b.iter(|| day19::part2(pc_idx, &instrs)));
}

fn day20(c: &mut Criterion) {
    use aoc_2018::day20;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day20.txt");

    let directions = day20::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day20 p1", move |b| b.iter(|| day20::part1(&directions)));
    let directions = day20::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day20 p2", move |b| b.iter(|| day20::part2(&directions)));
}

fn day21(c: &mut Criterion) {
    use aoc_2018::day21;

    const GLOBI_INPUT: &[u8] = include_bytes!("../../inputs/day21.txt");

    let (pc_idx, instrs) = day21::parse_input(GLOBI_INPUT);
    c.bench_function("day21 p1", move |b| b.iter(|| day21::part1(pc_idx, &instrs)));
    let (pc_idx, instrs) = day21::parse_input(GLOBI_INPUT);
    c.bench_function("day21 p2", move |b| b.iter(|| day21::part2(pc_idx, &instrs)));
}

fn day22(c: &mut Criterion) {
    use aoc_2018::day22;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day22.txt");

    let (depth, target) = day22::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day22 p1", move |b| b.iter(|| day22::part1(depth, target)));
    let (depth, target) = day22::parse_input(GLOBI_INPUT_STR);
    c.bench_function("day22 p2", move |b| b.iter(|| day22::part2(depth, target)));
}

fn day23(c: &mut Criterion) {
    use aoc_2018::day23;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day23.txt");

    let nanobots: Vec<_> = day23::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day23 p1", move |b| b.iter(|| day23::part1(&nanobots)));
    let nanobots: Vec<_> = day23::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day23 p2", move |b| b.iter(|| day23::part2(&nanobots)));
}

fn day24(c: &mut Criterion) {
    use aoc_2018::day24;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day24.txt");

    let groups: Vec<_> = day24::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day24 p1", move |b| b.iter(|| day24::part1(&groups)));
    let groups: Vec<_> = day24::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day24 p2", move |b| b.iter(|| day24::part2(&groups)));
}

fn day25(c: &mut Criterion) {
    use aoc_2018::day25;

    const GLOBI_INPUT_STR: &str = include_str!("../../inputs/day25.txt");

    let points: Vec<_> = day25::parse_input(GLOBI_INPUT_STR).collect();
    c.bench_function("day25 p1", move |b| b.iter(|| day25::part1(&points)));
}

criterion_group!(benches,
    day01, day02, day03, day04, day05, day06, day07, day08, day09, day10,
    day11, day12, day13, day16, day17, day23
);

criterion_group!{
    name = slower_benches;
    config = Criterion::default().sample_size(10);
    targets = day01_2, day09_2, day11_2, day14, day15, day18, day19, day20,
    day21, day22, day24, day25
}
criterion_main!(benches, slower_benches);
