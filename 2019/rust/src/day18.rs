use std::fmt::Debug;
use itertools::Itertools;
use std::collections::{VecDeque, HashSet, HashMap, BinaryHeap};
use std::cmp::Ordering;
use std::array::IntoIter;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day18.txt");

pub fn day18() -> impl Debug {
    let maze = parse_input(RAW_INPUT_STR);

    (part1(&maze), part2(&maze))
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct KeySet(u32);

impl KeySet {
    fn new() -> Self {
        Self(0)
    }

    fn push(self, bit: u8) -> Self {
        Self(self.0 | (1 << bit))
    }

    fn contains(self, bit: u8) -> bool {
        let mask = 1 << bit;
        self.0 & mask != 0
    }

    fn superset_of(self, other: KeySet) -> bool {
        (self.0 & other.0) == other.0
    }

    fn total(self) -> u32 {
        self.0.count_ones()
    }
}

pub fn part1(maze: &Maze) -> usize {
    let positioned_keys = maze.tiles
        .iter()
        .enumerate()
        .filter_map(|(idx, tile)| match tile {
            Tile::Key(c) => Some((idx, *c)),
            _ => None
        })
        .collect_vec();

    let mut key_paths = HashMap::new();

    for &(start_pos, _k1) in &positioned_keys {
        for &(end_pos, _k2) in &positioned_keys {
            let path = shortest_path(maze, start_pos, end_pos).expect("No path between keys!!");
            key_paths.insert((start_pos, end_pos), path);
        }

        let path = shortest_path(maze, maze.entrance, start_pos).expect("No path from entrance!!");
        key_paths.insert((maze.entrance, start_pos), path);
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct State {
        pos: Pos,
        keys: KeySet,
        steps: usize,
        estimate: usize,
    }

    impl State {
        fn metric(&self) -> usize {
            self.steps + self.estimate
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.metric().cmp(&other.metric()).reverse()
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let new_state = |pos, keys: KeySet, steps| {
        let estimate = positioned_keys
            .iter()
            .filter(|&(_, k)| !keys.contains(*k))
            .map(|&(k_pos, _)| key_paths[&(pos, k_pos)].0)
            .max()
            .unwrap_or(0);

        State {
            pos,
            keys,
            steps,
            estimate
        }
    };

    let initial_state = new_state(maze.entrance, KeySet::new(), 0);

    let mut open_set: BinaryHeap<_> = std::iter::once(initial_state).collect();
    let mut closed_set = HashSet::new();

     while let Some(state) = open_set.pop() {
        if state.keys.total() == positioned_keys.len() as _ {
            return state.steps
        }

        if closed_set.contains(&(state.pos, state.keys)) {
            continue;
        }

        closed_set.insert((state.pos, state.keys));

        for &(key_pos, k) in &positioned_keys {
            if state.keys.contains(k) {
                continue;
            }

            let &(steps, req_keys) = &key_paths[&(state.pos, key_pos)];
            if state.keys.superset_of(req_keys) {
                open_set.push(new_state(
                    key_pos,
                    state.keys.push(k),
                    state.steps + steps
                ));
            }
        }
    }

    panic!("No solution!")
}

fn shortest_path(maze: &Maze, start_pos: Pos, end_pos: Pos) -> Option<(usize, KeySet)> {
    let mut open_set: VecDeque<_> = std::iter::once((start_pos, KeySet::new(), 0)).collect();
    let mut closed_set = HashSet::new();

    while let Some((pos, keys, moves)) = open_set.pop_front() {
        if pos == end_pos {
            return Some((moves, keys))
        }

        if closed_set.contains(&pos) {
            continue;
        }

        closed_set.insert(pos);

        for (adj_pos, key) in maze.adjacent(pos) {
            let mut new_keys = keys.clone();
            if let Some(key) = key {
                new_keys = new_keys.push(key)
            }
            open_set.push_back((adj_pos, new_keys, moves + 1))
        }
    }

    None
}

pub fn part2(maze: &Maze) -> usize {
    let width = maze.width;
    let robot_pos = [
        maze.entrance - width - 1,
        maze.entrance - width + 1,
        maze.entrance + width - 1,
        maze.entrance + width + 1,
    ];

    let mut maze = maze.clone();
    maze.tiles[maze.entrance] = Tile::Wall;
    maze.tiles[maze.entrance + 1] = Tile::Wall;
    maze.tiles[maze.entrance - 1] = Tile::Wall;
    maze.tiles[maze.entrance - width] = Tile::Wall;
    maze.tiles[maze.entrance + width] = Tile::Wall;

    let positioned_keys = maze.tiles
        .iter()
        .enumerate()
        .filter_map(|(idx, tile)| match tile {
            Tile::Key(c) => Some((idx, *c)),
            _ => None
        })
        .collect_vec();

    let mut key_paths = HashMap::new();

    for &(start_pos, _k1) in &positioned_keys {
        for &(end_pos, _k2) in &positioned_keys {
            if let Some(path) = shortest_path(&maze, start_pos, end_pos) {
                key_paths.insert((start_pos, end_pos), path);
            }
        }

        for &pos in &robot_pos {
            if let Some(path) = shortest_path(&maze, pos, start_pos) {
                key_paths.insert((pos, start_pos), path);
            }
        }
    }

    #[derive(Debug, PartialEq, Eq, Hash)]
    struct State {
        robot_pos: [Pos; 4],
        keys: KeySet,
        steps: usize,
        estimate: usize,
    }

    impl State {
        fn metric(&self) -> usize {
            self.steps + self.estimate
        }
    }

    impl Ord for State {
        fn cmp(&self, other: &Self) -> Ordering {
            self.metric().cmp(&other.metric()).reverse()
        }
    }

    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    let new_state = |robot_pos, keys: KeySet, steps| {
        let estimate = 0;

        State {
            robot_pos,
            keys,
            steps,
            estimate
        }
    };

    let initial_state = new_state(robot_pos, KeySet::new(), 0);

    let mut open_set: BinaryHeap<_> = std::iter::once(initial_state).collect();
    let mut closed_set = HashSet::new();

     while let Some(state) = open_set.pop() {
        if state.keys.total() == positioned_keys.len() as _ {
            return state.steps
        }

        if closed_set.contains(&(state.robot_pos, state.keys)) {
            continue;
        }

        closed_set.insert((state.robot_pos, state.keys));

        for &(key_pos, k) in &positioned_keys {
            if state.keys.contains(k) {
                continue;
            }

            let (robot_idx, &(steps, req_keys)) = state.robot_pos
                .iter()
                .enumerate()
                .find_map(|(idx, &pos)| {
                    let req = key_paths.get(&(pos, key_pos))?;
                    Some((idx, req))
                })
                .expect("No robot can move");

            if state.keys.superset_of(req_keys) {
                let mut robot_pos = state.robot_pos.clone();
                robot_pos[robot_idx] = key_pos;
                open_set.push(new_state(
                    robot_pos,
                    state.keys.push(k),
                    state.steps + steps
                ));
            }
        }
    }

    panic!("No solution!")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Passage,
    Door(u8),
    Key(u8),
}

#[derive(Debug, Clone)]
pub struct Maze {
    tiles: Vec<Tile>,
    width: usize,
    entrance: Pos
}

impl Maze {
    fn adjacent<'a>(&'a self, pos: Pos) -> impl Iterator<Item = (Pos, Option<u8>)> + 'a {
        IntoIter::new([pos - 1, pos + 1, pos + self.width, pos - self.width])
            .filter_map(move |pos| match self.tiles[pos] {
                Tile::Passage | Tile::Key(_) => Some((pos, None)),
                Tile::Door(c) => Some((pos, Some(c))),
                _ => None
            })
    }
}

type Pos = usize;

pub fn parse_input(input: &str) -> Maze {
    let mut width = None;
    let mut entrance = None;

    let tiles = input.lines()
        .enumerate()
        .flat_map(|(y, line)| {
            width = Some(line.len());
            line.bytes().enumerate().map(|(x, ch)| match ch {
                b'#' => Tile::Wall,
                b'.' => Tile::Passage,
                b'@' => { entrance = Some(y * line.len() + x); Tile::Passage },
                b'A'..=b'Z' => Tile::Door(ch.to_ascii_lowercase() - b'a'),
                b'a'..=b'z' => Tile::Key(ch - b'a'),
                invalid => panic!("Invalid maze character: {}", invalid),
            }).collect_vec()
        })
        .collect();

    Maze {
        tiles,
        width: width.expect("Empty maze"),
        entrance: entrance.expect("Did not find any entrance!")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(&input), 3_646);
    }

    #[test]
    fn p2() {
        let input = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(&input), 1_730);
    }
}
