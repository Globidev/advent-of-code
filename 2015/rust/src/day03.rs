use std::collections::HashSet;

enum Direction { Up, Down, Left, Right }
#[derive(PartialEq, Eq, Hash, Clone)]
struct Position { x: i32, y: i32 }

fn parse_direction(sym: char) -> Direction {
    match sym {
        '^' => Direction::Up,
        'v' => Direction::Down,
        '<' => Direction::Left,
        '>' => Direction::Right,
        _ => panic!("Invalid direction")
    }
}

fn move_position(pos: &mut Position, dir: Direction) {
    match dir {
        Direction::Up    => pos.y -= 1,
        Direction::Down  => pos.y += 1,
        Direction::Left  => pos.x -= 1,
        Direction::Right => pos.x += 1,
    }
}

pub fn p1(input: &str) -> usize {
    let mut position = Position { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(position.clone());

    let directions = input.trim().chars().map(parse_direction);

    directions.for_each(|dir| {
        move_position(&mut position, dir);
        visited.insert(position.clone());
    });

    visited.len()
}

pub fn p2(input: &str) -> usize {
    let mut santa_position = Position { x: 0, y: 0 };
    let mut robot_position = Position { x: 0, y: 0 };
    let mut visited = HashSet::new();
    visited.insert(santa_position.clone());

    let directions = input.trim().chars().map(parse_direction);
    let mut positions = [&mut santa_position, &mut robot_position];

    directions.zip([0, 1].iter().cycle()).for_each(|(dir, i)| {
        move_position(&mut positions[*i], dir);
        visited.insert(positions[*i].clone());
    });

    visited.len()
}
