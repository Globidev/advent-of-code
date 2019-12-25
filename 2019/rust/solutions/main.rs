use aoc_2019::*;
use rayon::prelude::*;

fn main() {
    let days: &[fn() -> _] = &[
        || format!("day 01: {:?}\n", day01::day01()),
        || format!("day 02: {:?}\n", day02::day02()),
        || format!("day 03: {:?}\n", day03::day03()),
        || format!("day 04: {:?}\n", day04::day04()),
        || format!("day 05: {:?}\n", day05::day05()),
        || format!("day 06: {:?}\n", day06::day06()),
        || format!("day 07: {:?}\n", day07::day07()),
        || format!("day 08: {:?}\n", day08::day08()),
        // || format!("day 13: {:?}\n", day13::day13()),
        // || format!("day 14: {:?}\n", day14::day14()),
        // || format!("day 15: {:?}\n", day15::day15()),
        // || format!("day 16: {:?}\n", day16::day16()),
        // || format!("day 17: {:?}\n", day17::day17()),
        || format!("day 09: {:?}\n", day09::day09()),
        || format!("day 10: {:?}\n", day10::day10()),
        || format!("day 11: {:?}\n", day11::day11()),
        || format!("day 12: {:?}\n", day12::day12()),
        // || format!("day 18: {:?}\n", day18::day18()),
        // || format!("day 19: {:?}\n", day19::day19()),
        // || format!("day 20: {:?}\n", day20::day20()),
        // || format!("day 21: {:?}\n", day21::day21()),
        // || format!("day 22: {:?}\n", day22::day22()),
        // || format!("day 23: {:?}\n", day23::day23()),
        // || format!("day 24: {:?}\n", day24::day24()),
        // || format!("day 25: {:?}\n", day25::day25()),
    ];

    let filter: Vec<usize> = std::env::args()
        .skip(1)
        .map(|arg| arg.parse().unwrap())
        .collect();

    let result: String = if filter.is_empty() {
        days.par_iter()
            .map(|d| d())
            .collect()
    } else {
        filter.par_iter()
            .map(|i| days[i - 1]())
            .collect()
    };

    println!("{}", result);
}
