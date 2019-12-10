use std::fmt::Debug;
use std::ops::RangeInclusive;
use itertools::Itertools;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day04.txt");

pub fn day04() -> impl Debug {
    let range = parse_input(RAW_INPUT_STR);

    (part1(range.clone()), part2(range))
}

pub fn part1(range: RangeInclusive<Password>) -> usize {
    range.filter(|&p| matching_password_p1(p))
        .count()
}

pub fn part2(range: RangeInclusive<Password>) -> usize {
    range.filter(|&p| matching_password_p2(p))
        .count()
}

fn matching_password_p1(password: Password) -> bool {
    let digits = compute_digits(password);

    let increasing_digits = digits.iter().tuple_windows()
        .all(|(d1, d2)| d2 >= d1);

    let two_digits_equal = digits.iter().tuple_windows()
        .any(|(d1, d2)| d1 == d2);

    two_digits_equal && increasing_digits
}

fn matching_password_p2(password: Password) -> bool {
    let digits = compute_digits(password);

    let increasing_digits = digits.iter().tuple_windows()
        .all(|(d1, d2)| d2 >= d1);

    let two_digits_equal = digits.iter().enumerate().tuple_windows()
        .any(|((i1, d1), (i2, d2))| {
            d1 == d2 && digits.get(i1.wrapping_sub(1)) != Some(d1) && digits.get(i2 + 1) != Some(d1)
        });

    two_digits_equal && increasing_digits
}

fn compute_digits(password: Password) -> [u32; 6] {
    [
        password / 100_000 % 10,
        password / 10_000 % 10,
        password / 1_000 % 10,
        password / 100 % 10,
        password / 10 % 10,
        password % 10
    ]
}

pub fn parse_input(input: &str) -> RangeInclusive<Password> {
    let mut parts = input.split("-");

    let lower_bound = parts.next().expect("Missing lower bound")
        .parse().expect("Malformed lower bound");
    let upper_bound = parts.next().expect("Missing upper bound")
        .parse().expect("Malformed upper bound");

    lower_bound..=upper_bound
}

pub type Password = u32;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let range = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(range), 1_099);
    }

    #[test]
    fn p2() {
        let range = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(range), 710);
    }
}
