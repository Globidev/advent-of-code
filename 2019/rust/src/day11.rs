use std::fmt::{self, Debug};
use std::collections::HashMap;
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}, io::{Input, Output}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day11.txt");

pub fn day11() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), Banner(part2(&program)))
}

pub fn part1(program: &[Int]) -> usize {
    VirtualMachine::load(program)
        .with_driver::<HullPaintingRobot>()
        .run()
        .driver.painted.values().count()
}

pub fn part2(program: &[Int]) -> String {
    let mut robot = HullPaintingRobot::default();
    robot.grid.insert(Pos { x: 0, y: 0 }, Color::White);

    let robot = VirtualMachine::load(program)
        .driver(robot)
        .run()
        .driver;

    let mut canvas = std::iter::repeat_with(|| vec!['░'; 43]).take(6).collect_vec();
    for (Pos { x, y }, color) in robot.grid {
        canvas[y as usize][x as usize] = match color {
            Color::Black => '░',
            Color::White => '▓',
        }
    };
    canvas.into_iter().map(|l| l.into_iter().collect::<String>()).join("\n")
}

pub struct Banner(String);

impl Debug for Banner {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "\n{}", self.0)
    }
}

struct HullPaintingRobot {
    direction: Direction,
    position: Pos,
    grid: HashMap<Pos, Color>,
    painted: HashMap<Pos, i64>,
    next_order: Order
}

impl HullPaintingRobot {
    fn paint(&mut self, color: Color) {
        self.grid.entry(self.position)
            .and_modify(|e| *e = color)
            .or_insert(color);
        *self.painted.entry(self.position)
            .or_insert(0) += 1;
    }

    fn turn(&mut self, turn: Turn) {
        let new_direction = self.direction.turn(turn);
        self.position = self.position.translate(new_direction);
        self.direction = new_direction;
    }

    fn current_color(&self) -> Color {
        self.grid.get(&self.position)
            .copied()
            .unwrap_or(Color::Black)
    }
}

impl Default for HullPaintingRobot {
    fn default() -> Self {
        Self {
            direction: Direction::Up,
            position: Pos { x: 0, y: 0 },
            grid: HashMap::new(),
            next_order: Order::Paint,
            painted: HashMap::new(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: i32, y: i32 }

impl Pos {
    fn translate(self, direction: Direction) -> Self {
        let Pos { x, y } = self;

        let (x, y) = match direction {
            Direction::Up => (x, y - 1),
            Direction::Down => (x, y + 1),
            Direction::Left => (x - 1, y),
            Direction::Right => (x + 1, y),
        };

        Pos { x, y }
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
    fn turn(self, turn: Turn) -> Self {
        use Direction::*;

        match turn {
            Turn::Left => match self {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            },
            Turn::Right => match self {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            },
        }
    }
}

enum Turn {
    Left,
    Right
}

enum Order {
    Paint,
    Turn,
}

#[derive(Debug, Clone, Copy)]
enum Color {
    Black,
    White
}

impl From<Int> for Color {
    fn from(num: Int) -> Self {
        match num {
            0 => Self::Black,
            1 => Self::White,
            other => panic!("Invalid color: {}", other),
        }
    }
}

impl From<Int> for Turn {
    fn from(num: Int) -> Self {
        match num {
            0 => Self::Left,
            1 => Self::Right,
            other => panic!("Invalid turn: {}", other),
        }
    }
}

impl Output for HullPaintingRobot {
    fn output(&mut self, value: Int) {
        let order = match self.next_order {
            Order::Paint => {
                let color = Color::from(value);
                self.paint(color);
                Order::Turn
            },
            Order::Turn => {
                let turn = Turn::from(value);
                self.turn(turn);
                Order::Paint
            },
        };

        self.next_order = order;
    }
}

impl Input for HullPaintingRobot {
    fn input(&mut self) -> Int {
        match self.current_color() {
            Color::Black => 0,
            Color::White => 1,
        }
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Int> + '_ {
    input.split(',')
        .map(|raw_number| raw_number.parse().expect("Invalid integer code"))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part1(&code), 2_018);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), "\
░░▓▓░░▓▓▓░░▓▓▓▓░▓░░▓░▓▓▓░░▓░░▓░▓▓▓░░▓▓▓░░░░
░▓░░▓░▓░░▓░▓░░░░▓░▓░░▓░░▓░▓░▓░░▓░░▓░▓░░▓░░░
░▓░░▓░▓░░▓░▓▓▓░░▓▓░░░▓░░▓░▓▓░░░▓▓▓░░▓░░▓░░░
░▓▓▓▓░▓▓▓░░▓░░░░▓░▓░░▓▓▓░░▓░▓░░▓░░▓░▓▓▓░░░░
░▓░░▓░▓░░░░▓░░░░▓░▓░░▓░▓░░▓░▓░░▓░░▓░▓░▓░░░░
░▓░░▓░▓░░░░▓░░░░▓░░▓░▓░░▓░▓░░▓░▓▓▓░░▓░░▓░░░");
    }
}
