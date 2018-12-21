use std::collections::VecDeque;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day20.txt");

pub fn day20() -> (usize, usize) {
    let directions = parse_input(RAW_INPUT_STR);

    (part1(&directions), part2(&directions))
}

pub fn part1(directions: &[Direction]) -> usize {
    visit_facility(&directions)
        .map(|(_pos, distance)| distance)
        .max()
        .unwrap()
}

pub fn part2(directions: &[Direction]) -> usize {
    visit_facility(&directions)
        .filter(|(_pos, distance)| *distance >= 1000)
        .count()
}

fn walk(directions: &[Direction], (mut x, mut y): Position, map: &mut Map) {
    for direction in directions {
        let (dx, dy) = match direction {
            Direction::North => (0, -1),
            Direction::West  => (-1, 0),
            Direction::South => (0,  1),
            Direction::East  => (1,  0),
            Direction::Choice(choices) => {
                for choice in choices {
                    walk(choice, (x, y), map)
                }
                continue
            }
        };

        map.entry((x, y))
            .or_default()
            .push((x + dx, y + dy));
        x += dx;
        y += dy;
    }
}

fn visit_facility(directions: &[Direction])
    -> impl Iterator<Item = (Position, usize)>
{
    let initial_position = (0, 0);

    let mut map = Map::new();
    walk(directions, initial_position, &mut map);

    let mut open_set = VecDeque::with_capacity(map.len());
    open_set.push_back(Node { pos: initial_position, distance: 0 });
    FacilityVisitor {
        open_set,
        closed_set: hashbrown::HashSet::with_capacity(map.len()),
        map,
    }
}

struct Node {
    pos: Position,
    distance: usize,
}

struct FacilityVisitor {
    map: Map,
    open_set: VecDeque<Node>,
    closed_set: hashbrown::HashSet<Position>,
}

impl Iterator for FacilityVisitor {
    type Item = (Position, usize);

    fn next(&mut self) -> Option<Self::Item> {
        match self.open_set.pop_back() {
            None => None,
            Some(Node { pos, distance }) => {
                if let Some(adjacent_positions) = self.map.get(&pos) {
                    let new_distance = distance + 1;

                    for adjacent_pos in adjacent_positions {
                        if self.closed_set.contains(adjacent_pos) {
                            continue
                        }

                        let new_room = Node {
                            pos: *adjacent_pos,
                            distance: new_distance
                        };

                        self.open_set.push_back(new_room);
                        self.closed_set.insert(*adjacent_pos);
                    }
                }
                Some((pos, distance))
            }
        }
    }
}

pub fn parse_input(input: &str) -> Directions {
    parse_input_impl(&mut input.bytes()).0
}

fn parse_input_impl(input: &mut impl Iterator<Item = u8>) -> (Directions, ParseNext) {
    let mut directions = Vec::new();

    let next = loop {
        match input.next().expect("Unexpected EOF") {
            b'^' => continue,
            b'$' => break ParseNext::End,
            b'N' => directions.push(Direction::North),
            b'W' => directions.push(Direction::West),
            b'S' => directions.push(Direction::South),
            b'E' => directions.push(Direction::East),
            b'(' => {
                let mut choices = Vec::new();
                loop {
                    let (choice, next) = parse_input_impl(input);
                    choices.push(choice);
                    match next {
                        ParseNext::NextChoice => continue,
                        ParseNext::EndChoice => break,
                        ParseNext::End => panic!("Unmatched closing parens")
                    }
                }
                directions.push(Direction::Choice(choices))
            },
            b')' => break ParseNext::EndChoice,
            b'|' => break ParseNext::NextChoice,
            invalid => panic!("Invalid input byte: {}", invalid)
        }
    };

    (directions, next)
}

type Map = hashbrown::HashMap<Position, Adjacents>;
type Position = (isize, isize);
type Adjacents = arrayvec::ArrayVec<[Position; 4]>;

#[derive(Debug)]
pub enum Direction {
    North,
    West,
    South,
    East,
    Choice(Vec<Directions>),
}

type Directions = Vec<Direction>;

enum ParseNext {
    NextChoice,
    EndChoice,
    End
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let directions = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(&directions), 4432);
    }

    #[test]
    fn p2() {
        let directions = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(&directions), 8681);
    }
}
