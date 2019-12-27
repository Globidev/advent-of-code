use std::fmt::Debug;
use itertools::Itertools;
use std::collections::HashSet;
use std::ops::{Add, AddAssign};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day03.txt");

pub fn day03() -> impl Debug {
    let (wire1_it, wire2_it) = parse_input(RAW_INPUT_STR);

    let wire1 = wire1_it.collect_vec();
    let wire2 = wire2_it.collect_vec();

    (part1(&wire1, &wire2), part2(&wire1, &wire2))
}

pub fn part1(wire1: &[Movement], wire2: &[Movement]) -> i32 {
    wire_intersections(wire1, wire2)
        .map(|pos| pos.manhattan_dist(CENTRAL_PORT_POS))
        .min()
        .expect("No intersections found")
}

pub fn part2(wire1: &[Movement], wire2: &[Movement]) -> usize {
    wire_intersections(wire1, wire2)
        .map(|pos| {
            let steps1 = wire_steps_to_reach(wire1, pos);
            let steps2 = wire_steps_to_reach(wire2, pos);

            steps1 + steps2
        })
        .min()
        .expect("No intersections found")
}

fn wire_intersections(wire1: &[Movement], wire2: &[Movement]) -> impl Iterator<Item = Pos> {
    let visited1 = lay_wire(wire1);
    let visited2 = lay_wire(wire2);

    (&visited1 & &visited2).into_iter()
}

fn lay_wire(wire: &[Movement]) -> HashSet<Pos> {
    let mut pos = Pos(0, 0);
    let mut visited = HashSet::new();

    for &Movement { direction, amount } in wire {
        for _step in 0..amount {
            pos += direction;
            visited.insert(pos);
        }
    }

    visited
}

fn wire_steps_to_reach(wire: &[Movement], target_pos: Pos) -> usize {
    let mut pos = Pos(0, 0);
    let mut steps = 0;

    for &Movement { direction, amount } in wire {
        for _step in 0..amount {
            steps += 1;
            pos += direction;
            if pos == target_pos {
                return steps
            }
        }
    }

    panic!("The provided wire doesn't reach the target position")
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Pos(i32, i32);

const CENTRAL_PORT_POS: Pos = Pos(0, 0);

impl Pos {
    fn manhattan_dist(self, other: Pos) -> i32 {
        let Pos(x1, y1) = self;
        let Pos(x2, y2) = other;

        (x1 - x2).abs() + (y1 - y2).abs()
    }
}

impl Add<Direction> for Pos {
    type Output = Pos;

    fn add(self, dir: Direction) -> Self::Output {
        let Pos(x, y) = self;

        match dir {
            Direction::Up => Pos(x, y - 1),
            Direction::Down => Pos(x, y + 1),
            Direction::Left => Pos(x - 1, y),
            Direction::Right => Pos(x + 1, y),
        }
    }
}

impl AddAssign<Direction> for Pos {
    fn add_assign(&mut self, dir: Direction) {
        *self = *self + dir;
    }
}

#[derive(Debug)]
pub struct Movement {
    direction: Direction,
    amount: i32,
}

impl Movement {
    fn parse(raw: &str) -> Self {
        let mut chars = raw.chars();

        let direction = Direction::parse(chars.next().expect("Missing direction"));
        let amount = chars.as_str().parse().expect("Invalid movement amount");

        Self { direction, amount }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn parse(raw: char) -> Self {
        match raw {
            'U' => Self::Up,
            'D' => Self::Down,
            'L' => Self::Left,
            'R' => Self::Right,
            invalid => panic!("invalid direction character: {}", invalid),
        }
    }
}

pub fn parse_input(input: &'_ str) -> (impl Iterator<Item = Movement> + '_, impl Iterator<Item = Movement> + '_) {
    input.lines()
        .map(|line| line.split(',').map(Movement::parse))
        .next_tuple()
        .expect("Missing wire movements")
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (wire1_it, wire2_it) = parse_input(RAW_INPUT_STR);

        let wire1 = wire1_it.collect_vec();
        let wire2 = wire2_it.collect_vec();

        assert_eq!(part1(&wire1, &wire2), 352);
    }

    #[test]
    fn p2() {
        let (wire1_it, wire2_it) = parse_input(RAW_INPUT_STR);

        let wire1 = wire1_it.collect_vec();
        let wire2 = wire2_it.collect_vec();

        assert_eq!(part2(&wire1, &wire2), 43848);
    }
}
