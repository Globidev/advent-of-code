use std::fmt::Debug;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use crate::intcode::{Int, vm::{VirtualMachine, VMBuilder}, io::{Output, Input}};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day15.txt");

pub fn day15() -> impl Debug {
    let program = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&program), part2(&program))
}

pub fn part1(program: &[Int]) -> usize {
    let mapper = VirtualMachine::load(program)
        .with_driver::<Mapper>()
        .run()
        .driver;

    let oxygen_system_pos = mapper.oxygen_system_position()
        .expect("Oxygen system must be found at this point!");

    mapper.shortest_route(Pos { x: 0, y: 0 }, oxygen_system_pos).0
}

pub fn part2(program: &[Int]) -> usize {
    VirtualMachine::load(program)
        .with_driver::<Mapper>()
        .run()
        .driver
        .oxygen_fill_time()
}

struct Mapper {
    tiles: HashMap<Pos, Tile>,
    pos: Pos,
    last_direction: Direction,
    pos_to_visit: HashSet<Pos>,
    pos_target: Option<Pos>,
}

impl Default for Mapper {
    fn default() -> Self {
        let initial_pos = Pos { x: 0, y: 0 };

        let tiles = std::iter::once((initial_pos, Tile::Empty))
            .collect();

        let pos_to_visit = Direction::all()
            .map(|dir| initial_pos.translate(dir))
            .collect();

        Self {
            tiles,
            pos: initial_pos,
            last_direction: Direction::North,
            pos_to_visit,
            pos_target: None,
        }
    }
}

impl Mapper {
    fn oxygen_system_position(&self) -> Option<Pos> {
        self.tiles.iter()
            .find_map(|(&pos, &tile)| if tile == Tile::OxygenSystem { Some(pos) } else { None })
    }

    fn shortest_route(&self, start_pos: Pos, destination: Pos) -> (usize, Direction) {
        let mut open_set: VecDeque<_> = Direction::all()
            .filter_map(|dir| {
                let neighbor_pos = start_pos.translate(dir);

                if let Some(Tile::Wall) = self.tiles.get(&neighbor_pos) {
                    None
                } else {
                    Some((neighbor_pos, (1, dir)))
                }
            })
            .collect();

        let mut closed_set: HashSet<_> = std::iter::once(start_pos).collect();

        while let Some((pos, (move_count, start_dir))) = open_set.pop_front() {
            if pos == destination {
                return (move_count, start_dir)
            }

            closed_set.insert(pos);

            for direction in Direction::all() {
                let neighbor_pos = pos.translate(direction);
                if neighbor_pos == destination {
                    return (move_count + 1, start_dir)
                }

                if let None | Some(Tile::Wall) = self.tiles.get(&neighbor_pos) {
                    continue
                }

                if !closed_set.contains(&neighbor_pos) {
                    open_set.push_back((neighbor_pos, (move_count + 1, start_dir)));
                }
            }
        }

        panic!("No route found!")
    }

    fn oxygen_fill_time(&self) -> usize {
        let oxygen_start_pos = self.oxygen_system_position()
            .expect("Oxygen system must be found at this point!");

        let mut empty_tiles: HashSet<_> = self.tiles.iter()
            .filter_map(|(&pos, &tile)| if tile == Tile::Empty { Some(pos) } else { None })
            .collect();

        let mut oxygen_positions = vec![oxygen_start_pos];
        let mut minutes = 0;

        while !empty_tiles.is_empty() {
            let next_oxygen_positions = oxygen_positions.into_iter()
                .flat_map(|pos| {
                    Direction::all()
                        .map(move |dir| pos.translate(dir))
                        .filter(|pos| empty_tiles.contains(&pos))
                })
                .collect();

            for next_pos in &next_oxygen_positions {
                empty_tiles.remove(next_pos);
            }

            oxygen_positions = next_oxygen_positions;
            minutes += 1;
        }

        minutes
    }
}

impl Output for Mapper {
    fn output(&mut self, status_code: Int) {
        let pos_tried = self.pos.translate(self.last_direction);
        if self.pos_target == Some(pos_tried) {
            self.pos_target = None;
        }

        let tile = match status_code {
            0 => Tile::Wall,
            1 => Tile::Empty,
            2 => Tile::OxygenSystem,
            unknown => panic!("Droid sent an unknown status code: {}", unknown),
        };

        self.tiles.insert(pos_tried, tile);
        self.pos_to_visit.remove(&pos_tried);

        if tile != Tile::Wall {
            self.pos = pos_tried;

            for direction in Direction::all() {
                let pos = self.pos.translate(direction);
                if !self.tiles.contains_key(&pos) {
                    self.pos_to_visit.insert(pos);
                }
            }
        }
    }
}

impl Input for Mapper {
    fn input(&mut self) -> Int {
        if self.pos_to_visit.is_empty() {
            return 0 // Invalid input should stop the program
        }

        if self.pos_target.is_none() {
            let &nearest = self.pos_to_visit.iter()
                .min_by_key(|pos| pos.manhattan_dist(self.pos))
                .expect("Broke non empty pos to visit invariant");

            self.pos_target = Some(nearest);
        }

        let target = self.pos_target.unwrap();

        let direction = self.shortest_route(self.pos, target).1;
        self.last_direction = direction;

        direction as Int
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Empty,
    OxygenSystem,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Pos { x: i32, y: i32 }

impl Pos {
    fn manhattan_dist(self, other: Self) -> i32 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Pos {
    fn translate(self, dir: Direction) -> Self {
        let Pos { x, y } = self;

        let (x, y) = match dir {
            Direction::North => (x, y - 1),
            Direction::South => (x, y + 1),
            Direction::West => (x - 1, y),
            Direction::East => (x + 1, y),
        };

        Pos { x, y }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

impl Direction {
    fn all() -> impl Iterator<Item = Self> {
        use Direction::*;
        use std::array::IntoIter;

        IntoIter::new([North, South, West, East])
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

        assert_eq!(part1(&code), 266);
    }

    #[test]
    fn p2() {
        let code = parse_input(RAW_INPUT_STR).collect_vec();

        assert_eq!(part2(&code), 274);
    }
}
