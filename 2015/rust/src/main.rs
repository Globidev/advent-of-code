#![feature(iterator_step_by)]
#![feature(integer_atomics)]
#![feature(inclusive_range_syntax)]

use std::fmt::Display;
use std::fs::File;
use std::io::Read;

type DayPart<T> = fn(&str) -> T;

fn run_parts<T1: Display, T2: Display>(
    file_name: &str, p1: DayPart<T1>, p2: DayPart<T2>
) -> std::io::Result<(T1, T2)> {
    let mut file = File::open(file_name)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    Ok((p1(&input), p2(&input)))
}

fn run_day<T1: Display, T2: Display>(day: u32, p1: DayPart<T1>, p2: DayPart<T2>) {
    let file_name = format_args!("../inputs/day{:02}", day).to_string();

    println!("Day {:02}:", day);

    match run_parts(&file_name, p1, p2) {
        Err(e) => println!("  {}", e),
        Ok((r1, r2)) => {
            println!("  Part 1: {}", r1);
            println!("  Part 2: {}", r2);
        }
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;
mod day13;
mod day14;
mod day15;

fn main() {
    run_day(01, day01::p1, day01::p2);
    run_day(02, day02::p1, day02::p2);
    run_day(03, day03::p1, day03::p2);
    run_day(04, day04::p1, day04::p2);
    run_day(05, day05::p1, day05::p2);
    run_day(06, day06::p1, day06::p2);
    run_day(07, day07::p1, day07::p2);
    run_day(08, day08::p1, day08::p2);
    run_day(09, day09::p1, day09::p2);
    run_day(10, day10::p1, day10::p2);
    run_day(11, day11::p1, day11::p2);
    run_day(12, day12::p1, day12::p2);
    run_day(13, day13::p1, day13::p2);
    run_day(14, day14::p1, day14::p2);
    run_day(15, day15::p1, day15::p2);
}
