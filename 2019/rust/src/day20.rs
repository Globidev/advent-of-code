use std::fmt::Debug;
use std::collections::{HashMap, HashSet, VecDeque};
use itertools::Itertools;
use std::array::IntoIter;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day20.txt");

pub fn day20() -> impl Debug {
    let maze = parse_input(RAW_INPUT_STR);

    (part1(&maze), part2(&maze))
}

pub fn part1(maze: &Maze) -> usize {
    let mut open_set: VecDeque<_> = std::iter::once((maze.entrance, 0)).collect();
    let mut closed_set = HashSet::new();

    while let Some((pos, moves)) = open_set.pop_front() {
        if pos == maze.exit {
            return moves
        }

        if closed_set.contains(&pos) {
            continue;
        }

        closed_set.insert(pos);

        for (adj_pos, _) in maze.adjacent(pos) {
            open_set.push_back((adj_pos, moves + 1))
        }
    }

    panic!("No solution!")
}

pub fn part2(maze: &Maze) -> usize {
    let mut open_set: VecDeque<_> = std::iter::once((maze.entrance, 0, 0)).collect();
    let mut closed_set = HashSet::new();

    while let Some((pos, level, moves)) = open_set.pop_front() {
        if pos == maze.exit && level == 0 {
            return moves
        }

        if closed_set.contains(&(pos, level)) {
            continue;
        }

        closed_set.insert((pos, level));

        for (adj_pos, delta_level) in maze.adjacent(pos) {
            let next_level = level + delta_level;
            if next_level >= 0 {
                open_set.push_back((adj_pos, next_level, moves + 1))
            }
        }
    }

    panic!("No solution!")
}

#[derive(Debug, Clone)]
pub struct Maze {
    tiles: Vec<Tile>,
    width: usize,
    teleports: HashMap<Pos, Pos>,
    entrance: Pos,
    exit: Pos,
}

impl Maze {
    fn adjacent(&self, pos: Pos) -> impl Iterator<Item = (Pos, i32)> + '_ {
        adjacent_pos(pos, self.width)
            .filter_map(move |pos| {
                match self.tiles[pos] {
                    Tile::Wall => None,
                    Tile::Teleport([b'A', b'A'], _) | Tile::Teleport([b'Z', b'Z'], _) => None,
                    Tile::Passage => Some((pos, 0)),
                    Tile::Teleport(..) => {
                        let x = pos % self.width;
                        let y = pos / self.width;
                        let height = self.tiles.len() / self.width;
                        let delta_level = if y <= 3 || y >= height - 3 || x <= 3 || x >= self.width - 3 {
                            -1
                        } else {
                            1
                        };
                        Some((self.teleports[&pos], delta_level))
                    }
                }
            })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Wall,
    Passage,
    Teleport([u8; 2], Pos),
}


type Pos = usize;

pub fn parse_input(input: &str) -> Maze {
    let mut width = None;

    let raw_tiles = input.lines()
        .flat_map(|line| {
            width = Some(line.len());
            line.bytes()
        })
        .collect_vec();

    let width = width.expect("Empty maze");

    let tiles = raw_tiles.iter()
        .copied()
        .enumerate()
        .map(|(idx, raw_tile)| match raw_tile {
            b' ' | b'#' => Tile::Wall,
            b'.' => Tile::Passage,
            b'A'..=b'Z' => {
                let adjacent_letter = adjacent_pos(idx, width)
                    .find_map(|pos| match raw_tiles.get(pos)? {
                        ch @ b'A'..=b'Z' => Some(*ch),
                        _ => None
                    })
                    .expect("single letter ?");

                let adjacent_passage = adjacent_pos(idx, width)
                    .find(|&pos| raw_tiles.get(pos) == Some(&b'.'));

                match adjacent_passage {
                    Some(pos) => Tile::Teleport([raw_tile, adjacent_letter], pos),
                    None => Tile::Wall,
                }
            },
            invalid => panic!("Invalid maze descriptor character: {}", invalid)
        })
        .collect_vec();

    let mut entrance = None;
    let mut exit = None;

    let teleports = tiles.iter()
        .enumerate()
        .filter_map(|(pos, tile)| {
            if let Tile::Teleport([c1, c2], tp_pos) = tile {
                match (c1, c2) {
                    (b'A', b'A') => { entrance = Some(*tp_pos); None },
                    (b'Z', b'Z') => { exit = Some(*tp_pos); None },
                    _ => {
                        let connected_portal = tiles.iter()
                            .find_map(|tile| {
                                match tile {
                                    Tile::Teleport([c3, c4], tp_pos_2) => {
                                        if tp_pos == tp_pos_2 {
                                            None
                                        } else if [c1, c2] == [c3, c4] || [c1, c2] == [c4, c3] {
                                            Some(tp_pos_2)
                                        } else {
                                            None
                                        }
                                    },
                                    _ => None
                                }
                            })
                            .expect("No connected portal!");

                        Some((pos, *connected_portal))
                    },
                }
            } else {
                None
            }
        })
        .collect();

    Maze {
        tiles,
        width,
        teleports,
        entrance: entrance.expect("Did not find any entrance!"),
        exit: exit.expect("Did not find any exit!"),
    }
}

fn adjacent_pos(pos: Pos, width: usize) -> impl Iterator<Item = Pos> {
    IntoIter::new([
        pos - 1,
        pos + 1,
        pos - width,
        pos + width,
    ])
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(&input), 606);
    }

    #[test]
    fn p2() {
        let input = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(&input), 7_186);
    }
}
