#[macro_use]
extern crate criterion;

use criterion::{Criterion, Fun};

fn day01(c: &mut Criterion) {
    use aoc_2018::day01;

    let input: Vec<_> = day01::parsed_input().collect();
    c.bench_function("day01 p1", move |b| b.iter(|| day01::part1(&input)));

    use std::collections::{HashSet, BTreeSet};
    use hashbrown::HashSet as SwissTable;

    let p2_funs = vec![
        Fun::new("HashSet",    |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), HashSet::new()))),
        Fun::new("BTreeSet",   |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), BTreeSet::new()))),
        Fun::new("SwissTable", |b, i: &Vec<_>| b.iter(|| day01::part2_impl(i.as_slice(), SwissTable::new()))),
    ];

    let input: Vec<_> = day01::parsed_input().collect();
    c.bench_functions("day01 p2", p2_funs, input);
}

criterion_group!(benches, day01);
criterion_main!(benches);
