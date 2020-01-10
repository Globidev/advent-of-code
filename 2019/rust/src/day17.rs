use std::fmt::Debug;
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}, io::Output};
use std::convert::TryInto;
use std::array::IntoIter;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day17.txt");

pub fn day17() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> usize {
    let mapper = VirtualMachine::load(program)
        .with_output_driver::<Mapper>()
        .run()
        .into_output();

    let map_width = mapper.width.expect("Did not encounter a newline somehow");
    let tiles = mapper.tiles;
    let map_height = tiles.len() / map_width;

    let scaffold_positions = tiles.iter()
        .enumerate()
        .filter_map(|(pos_1d, &tile)| {
            if tile == Tile::Scaffold {
                Some(pos_1d)
            } else {
                None
            }
        });

    scaffold_positions
        .filter_map(|pos_1d| {
            let (x, y) = (pos_1d % map_width, pos_1d / map_width);

            let scaffold_left = x > 0 && tiles[pos_1d - 1] == Tile::Scaffold;
            let scaffold_right = x < map_width - 1 && tiles[pos_1d + 1] == Tile::Scaffold;
            let scaffold_up = y > 0 && tiles[pos_1d - map_width] == Tile::Scaffold;
            let scaffold_down = y < map_height - 1 && tiles[pos_1d + map_width] == Tile::Scaffold;

            if (scaffold_left && scaffold_right) && (scaffold_up && scaffold_down) {
                Some(x * y)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(program: &[Int]) -> Int {
    let mapper = VirtualMachine::load(program)
        .with_output_driver::<Mapper>()
        .run()
        .into_output();

    let map_width = mapper.width.expect("Did not encounter a newline somehow");
    let tiles = mapper.tiles;
    let map_height = tiles.len() / map_width;

    let (mut rpos, mut rdir) = tiles.iter()
        .enumerate()
        .find_map(|(idx, tile)| {
            if let Tile::Robot(dir) = tile {
                Some((Pos { x: idx % map_width, y: idx / map_width }, *dir))
            } else {
                None
            }
        })
        .expect("Did not find the robot somehow");

    let scaffold_at = |pos: Pos| -> bool {
        let idx = pos.y * map_width + pos.x;
        tiles[idx] == Tile::Scaffold
    };

    let next_move = |pos: Pos, rdir: Direction| -> Option<(Pos, Movement)> {
        use Movement::*;
        let mut directions = IntoIter::new([Advance, Left, Right]);

        directions
            .find_map(|movement| {
                let direction = rdir.turn(movement);
                let next_pos = pos.translate(direction, map_width, map_height)?;
                if scaffold_at(next_pos) {
                    Some((if movement == Advance { next_pos } else { pos }, movement))
                } else {
                    None
                }
            })
    };

    let mut path = vec![];

    loop {
        let (next_pos, movement) = match next_move(rpos, rdir) {
            Some(next_move) => next_move,
            None => break,
        };
        rpos = next_pos;
        rdir = rdir.turn(movement);
        path.push(movement);
    }

    // let mut path = path.into_iter().peekable();

    // let paths = std::iter::from_fn(|| {
    //     let turn = path.next()?;
    //     let advances = path.by_ref().peeking_take_while(|&m| m == Movement::Advance).count();
    //     Some((turn, advances))
    // });

    let input = "\
B,A,B,C,A,B,C,B,C,A
L,12,R,4,L,12,R,6
L,12,L,8,L,8
R,4,L,12,L,12,R,6
y
";

    println!("{}", input);

    let mut program = program.to_vec();
    program[0] = 2;

    VirtualMachine::load(program)
        .input_iter(input.chars().map(|c| c as _))
        .single_output()
        .run()
        .output()
        .expect("Robot did not report collected dust")
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
enum Movement {
    Left,
    Right,
    Advance
}

#[derive(Debug, Default)]
struct Mapper {
    tiles: Vec<Tile>,
    width: Option<usize>,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Scaffold,
    Space,
    Robot(Direction),
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn turn(self, movement: Movement) -> Self {
        use Direction::*;

        match movement {
            Movement::Left => match self {
                Up => Left,
                Down => Right,
                Left => Down,
                Right => Up,
            },
            Movement::Right => match self {
                Up => Right,
                Down => Left,
                Left => Up,
                Right => Down,
            },
            Movement::Advance => self,
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: usize, y: usize }

impl Pos {
    fn translate(self, dir: Direction, x_bound: usize, y_bound: usize) -> Option<Self> {
        let Pos { x, y } = self;

        let (x, y) = match dir {
            Direction::Up => (x, y.checked_sub(1)?),
            Direction::Down => (x, y + 1),
            Direction::Left => (x.checked_sub(1)?, y),
            Direction::Right => (x + 1, y),
        };

        if x >= x_bound || y >= y_bound {
            return None
        }

        Some(Pos { x, y })
    }
}

impl Output for Mapper {
    fn output(&mut self, value: Int) {
        let ascii_value = value.try_into().expect("Invalid ascii output");

        match ascii_value {
            b'\n' => if let None = self.width { self.width = Some(self.tiles.len()) },
            b'#' => self.tiles.push(Tile::Scaffold),
            b'.' => self.tiles.push(Tile::Space),
            b'^' => self.tiles.push(Tile::Robot(Direction::Up)),
            b'v' => self.tiles.push(Tile::Robot(Direction::Down)),
            b'<' => self.tiles.push(Tile::Robot(Direction::Left)),
            b'>' => self.tiles.push(Tile::Robot(Direction::Right)),
            invalid => panic!("Invalid output value: {}", invalid)
        }

        print!("{}", std::char::from_u32(value as _).expect("??"));
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

        assert_eq!(part1(&code), 6244);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 1_143_523);
    }
}
