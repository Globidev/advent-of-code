#![feature(iterator_step_by)]
#![feature(integer_atomics)]
#![feature(inclusive_range_syntax)]

use std::fmt::Display;
use std::fs::File;
use std::io::Read;

type DayPart<T> = fn(&str) -> T;

fn run_parts<T1: Display, T2: Display>(
    file_name: &str, p1: DayPart<T1>, p2: DayPart<T2>
) -> std::io::Result<()> {
    let mut file = File::open(file_name)?;
    let mut input = String::new();
    file.read_to_string(&mut input)?;

    println!("  Part 1: {}", p1(&input));
    println!("  Part 2: {}", p2(&input));
    Ok(())
}

fn run_day<T1: Display, T2: Display>(day: u32, p1: DayPart<T1>, p2: DayPart<T2>) {
    let file_name = format_args!("../inputs/day{:02}", day).to_string();

    println!("Day {:02}:", day);

    match run_parts(&file_name, p1, p2) {
        Err(e) => println!("  {}", e),
        _ => ()
    }
}

mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;

fn main() {
    run_day(01, day01::p1, day01::p2);
    run_day(02, day02::p1, day02::p2);
    run_day(03, day03::p1, day03::p2);
    run_day(04, day04::p1, day04::p2);
    run_day(05, day05::p1, day05::p2);
    run_day(06, day06::p1, day06::p2);
}
