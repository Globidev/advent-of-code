use hashbrown::HashSet;

const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day13.txt");

pub fn day13() -> ((u16, u16), (u16, u16)) {
    let (world, trains) = parse_input(RAW_INPUT);

    (part1(&world, &trains), part2(&world, &trains))
}

pub fn part1(world: &World, trains: &Trains) -> (u16, u16) {
    let mut trains = trains.iter().cloned().collect();

    loop {
        let crashes = tick(world, &mut trains);

        if let Some(&Position { x, y }) = crashes.first() {
            break (x, y)
        }
    }
}

pub fn part2(world: &World, trains: &Trains) -> (u16, u16) {
    let mut trains: Vec<_> = trains.iter().cloned().collect();

    while trains.len() > 1 {
        tick(world, &mut trains);
    }

    match trains.first() {
        Some(&Train { pos: Position { x, y }, .. }) => (x, y),
        None => panic!("No more trains left")
    }
}

fn tick(world: &World, trains: &mut Trains) -> Collisions {
    let mut train_positions: HashSet<_> = trains.iter()
        .map(|train| train.pos.clone())
        .collect();

    let mut collisions = Collisions::new();

    trains.sort_by_key(|train| (train.pos.y, train.pos.x));

    for train in trains.iter_mut() {
        if collisions.contains(&train.pos) {
            continue
        }

        train_positions.remove(&train.pos);
        train.tick(world);

        if !train_positions.insert(train.pos.clone()) {
            collisions.push(train.pos.clone());
        }
    }

    trains.retain(|train| !collisions.contains(&train.pos));

    collisions
}

pub fn parse_input(input: &[u8]) -> (World, Trains) {
    let mut trains = Trains::with_capacity(32);
    let height = input.iter().position(|&c| c == b'\n').unwrap();

    let rails: Vec<_> = input.split(|&c| c == b'\n')
        .flatten()
        .enumerate()
        .map(|(i, c)| {
            let mut add_train = |direction| {
                let x = i % height;
                let y = i / height;

                trains.push(Train {
                    pos: Position { x: x as u16, y: y as u16 },
                    direction,
                    turn_strategy: TurnStrategy::Left
                })
            };

            match c {
                b'^' => add_train(Direction::Up),
                b'v' => add_train(Direction::Down),
                b'<' => add_train(Direction::Left),
                b'>' => add_train(Direction::Right),
                _ => ()
            }

            match c {
                b' '        => Rail::Empty,
                b'-' | b'|' => Rail::Straight,
                b'/'        => Rail::CurveRight,
                b'\\'       => Rail::CurveLeft,
                b'+'        => Rail::Intersection,
                _           => Rail::Straight // Trains are only placed on straight lines
            }
        })
        .collect();

    (World { rails, height }, trains)
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Position {
    x: u16,
    y: u16
}

#[derive(Debug)]
pub enum Rail {
    Empty,
    Straight,
    CurveRight,
    CurveLeft,
    Intersection
}

#[derive(Debug)]
pub struct World {
    rails: Vec<Rail>,
    height: usize
}

#[derive(Debug, Clone)]
pub struct Train {
    pos: Position,
    direction: Direction,
    turn_strategy: TurnStrategy
}

type Trains = Vec<Train>;
type Collisions = Vec<Position>;

#[derive(Debug, Clone)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

#[derive(Debug, Clone)]
pub enum TurnStrategy {
    Left,
    Straight,
    Right
}

impl Position {
    fn moved(&self, direction: &Direction) -> Self {
        use self::Direction::*;

        let &Position { x, y } = self;

        match direction {
            Up    => Position { x: x + 0, y: y - 1 },
            Down  => Position { x: x + 0, y: y + 1 },
            Left  => Position { x: x - 1, y: y + 0 },
            Right => Position { x: x + 1, y: y + 0 },
        }
    }
}

impl World {
    fn at(&self, pos: &Position) -> &Rail {
        &self.rails[pos.y as usize * self.height + pos.x as usize]
    }
}

impl Train {
    fn tick(&mut self, world: &World) {
        use self::Direction::*;

        let new_pos = self.pos.moved(&self.direction);

        let turn_direction = match world.at(&new_pos) {
            Rail::Straight     => TurnStrategy::Straight,
            Rail::Intersection => self.turn_strategy.next(),
            Rail::CurveLeft    => {
                match self.direction {
                    Up   | Down  => TurnStrategy::Left,
                    Left | Right => TurnStrategy::Right,
                }
            },
            Rail::CurveRight => {
                match self.direction {
                    Up   | Down  => TurnStrategy::Right,
                    Left | Right => TurnStrategy::Left,
                }
            },
            Rail::Empty => panic!("A train went off the tracks!"),
        };

        let new_direction = self.direction.turned(&turn_direction);

        self.pos = new_pos;
        self.direction = new_direction;
    }
}

impl Direction {
    fn turned(&self, way: &TurnStrategy) -> Self {
        use self::Direction::*;
        use self::TurnStrategy::{Left as TurnLeft, Right as TurnRight};

        match (way, self) {
            (TurnLeft, Up)    => Left,
            (TurnLeft, Down)  => Right,
            (TurnLeft, Left)  => Down,
            (TurnLeft, Right) => Up,

            (TurnRight, Up)    => Right,
            (TurnRight, Down)  => Left,
            (TurnRight, Left)  => Up,
            (TurnRight, Right) => Down,

            (_straight, current_direction) => current_direction.clone(),
        }
    }
}

impl TurnStrategy {
    fn next(&mut self) -> Self {
        use self::TurnStrategy::*;

        let next = match self {
            Left => Straight,
            Straight => Right,
            Right => Left,
        };

        std::mem::replace(self, next)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (world, trains) = parse_input(RAW_INPUT);

        assert_eq!(part1(&world, &trains), (38, 72));
    }

    #[test]
    fn p2() {
        let (world, trains) = parse_input(RAW_INPUT);

        assert_eq!(part2(&world, &trains), (68, 27));
    }
}
