use std::fmt::{self, Debug};
use std::collections::HashMap;
use itertools::Itertools;
use std::cmp::Ordering;
use crate::intcode::{Int, vm::VirtualMachine, io::{Output, Input}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day13.txt");

pub fn day13() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> usize {
    let mut game = ArcadeGame::default();

    let vm = VirtualMachine::new(program, &mut game);
    vm.run();

    game.screen.tiles.values()
        .filter(|&&t| t == Tile::Block)
        .count()
}

pub fn part2(program: &[Int]) -> Int {
    let mut program = program.to_vec();
    program[0] = 2; // Play for free

    let mut game = ArcadeGame::default();

    let vm = VirtualMachine::new(program, &mut game);
    vm.run();

    game.score
}

#[derive(Debug, Default)]
struct Screen {
    tiles: HashMap<Pos, Tile>,
}

#[derive(Debug, Clone, Copy)]
enum JoyStick {
    Neutral = 0,
    Left = -1,
    Right = 1,
}

#[derive(Debug, Default)]
struct ArcadeGame {
    screen: Screen,
    state: OutputState,
    score: Int,
    ball_pos: Pos,
    paddle_pos: Pos,
}

impl Input for ArcadeGame {
    fn input(&mut self) -> Int {
        let target_x = self.ball_pos.x;
        let paddle_x = self.paddle_pos.x;

        let joystick = match paddle_x.cmp(&target_x) {
            Ordering::Less => JoyStick::Right,
            Ordering::Equal => JoyStick::Neutral,
            Ordering::Greater => JoyStick::Left,
        };

        joystick as _
    }
}

impl Output for ArcadeGame {
    fn output(&mut self, value: Int) {
        let next_state = match self.state {
            OutputState::X => OutputState::Y { x: value },
            OutputState::Y { x } => {
                let y = value;
                if x == -1 && y == 0 {
                    OutputState::Score
                } else {
                    OutputState::TileId { pos: Pos { x, y } }
                }
            },
            OutputState::TileId { pos } => {
                let tile = Tile::from(value);
                self.screen.tiles.insert(pos, tile);
                match tile {
                    Tile::Ball => self.ball_pos = pos,
                    Tile::Paddle => self.paddle_pos = pos,
                    _ => (),
                }
                OutputState::X
            },
            OutputState::Score => {
                self.score = value;
                OutputState::X
            },
        };

        self.state = next_state;
    }
}

#[derive(Debug)]
enum OutputState {
    X,
    Y { x: Int },
    TileId { pos: Pos },
    Score,
}

impl Default for OutputState {
    fn default() -> Self { Self::X }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: Int, y: Int }

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball
}

impl From<Int> for Tile {
    fn from(val: Int) -> Self {
        match val {
            0 => Self::Empty,
            1 => Self::Wall,
            2 => Self::Block,
            3 => Self::Paddle,
            4 => Self::Ball,
            invalid => panic!("Invalid tile id: {}", invalid),
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

        assert_eq!(part1(&code), 320);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 15_156);
    }
}
