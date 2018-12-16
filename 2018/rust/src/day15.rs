use std::collections::VecDeque;
use hashbrown::HashSet;
use rayon::prelude::*;

const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day15.txt");

pub fn day15() -> (u32, u32) {
    let world_builder = parse_input(RAW_INPUT);

    (part1(&world_builder), part2(&world_builder))
}

pub fn part1(world_builder: &WorldBuilder) -> u32 {
    let mut world = world_builder.with_elf_ap(3);

    let end_turn = (0..)
        .skip_while(|_| world.resolve_next_round() == RoundResult::Complete)
        .next()
        .unwrap();

    end_turn * world.entities_hps()
}

pub fn part2(world_builder: &WorldBuilder) -> u32 {
    let initial_elves_count = world_builder.parsed_entities.iter()
        .filter(|&parsed_entity| *parsed_entity == ParsedEntity::Elf)
        .count();

    let starting_ap = 4_usize;
    let ending_ap = 200;    // I don't think that having beyond one shotting
                            // capabilities can make a difference

    (starting_ap..ending_ap+1)
        .into_par_iter()
        .filter_map(|ap| {
            let mut world = world_builder.with_elf_ap(ap as u32);

            let end_turn = (0..)
                .skip_while(|_| {
                    let is_complete = world.resolve_next_round() == RoundResult::Complete;
                    let had_elf_casualties = world.elves_count < initial_elves_count;
                    is_complete && !had_elf_casualties
                })
                .next()
                .unwrap();

            if world.elves_count == initial_elves_count {
                Some(end_turn * world.entities_hps())
            } else {
                None
            }
        })
        .find_first(|_| true)
        .unwrap()
}

pub fn parse_input(input: &[u8]) -> WorldBuilder {
    let height = input.iter()
        .position(|&c| c == b'\n')
        .unwrap();

    let parsed_entities = input.split(|&c| c == b'\n')
        .flatten()
        .map(|c| match c {
            b'#' => ParsedEntity::Wall,
            b'.' => ParsedEntity::OpenCavern,
            b'E' => ParsedEntity::Elf,
            b'G' => ParsedEntity::Goblin,
            invalid => panic!("Invalid input byte: {}", invalid)
        })
        .collect();

    WorldBuilder { height, parsed_entities }
}

#[derive(Debug, Clone)]
pub struct World {
    height: usize,
    entities: Vec<Entity>,
    elves_count: usize,
    goblins_count: usize,
}

#[derive(Debug, Clone)]
struct Unit {
    ap: u32,
    hp: u32
}

#[derive(Debug, Clone)]
enum Entity {
    Wall,
    OpenCavern,
    Unit { kind: UnitKind, unit: Unit },
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum UnitKind {
    Elf,
    Goblin,
}

#[derive(PartialEq, Eq)]
enum RoundResult {
    Complete,
    Incomplete
}

#[derive(Debug, Clone)]
pub struct WorldBuilder {
    height: usize,
    parsed_entities: Vec<ParsedEntity>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ParsedEntity {
    Wall,
    OpenCavern,
    Elf,
    Goblin
}

impl World {
    fn resolve_next_round(&mut self) -> RoundResult {
        let mut processed = HashSet::new();

        for position in 0..self.entities.len() {
            // Skip non units and already processed units
            if processed.contains(&position) {
                continue
            }

            let (enemy_kind, ap) = match &self.entities[position] {
                Entity::Unit { kind, unit } => (kind.enemy(), unit.ap),
                _ => continue
            };

            let target = match self.first_enemy_in_range(&enemy_kind, position) {
                // There is already an enemy in range, that's our target
                Some(enemy_pos) => Some(enemy_pos),
                // Otherwise try to find a target to move towards
                None => match self.step_towards_closest_target(&enemy_kind, position) {
                    Some(new_pos) => {
                        self.move_unit(position, new_pos);
                        processed.insert(new_pos);
                        // Try to find a target again
                        self.first_enemy_in_range(&enemy_kind, new_pos)
                    },
                    // No reachable target, maybe there is none left ?
                    // If that's the case, short-circuit the round
                    None => {
                        if self.elves_count == 0 || self.goblins_count == 0 {
                            return RoundResult::Incomplete
                        }
                        None
                    }
                }
            };

            if let Some(enemy_pos) = target {
                self.attack(enemy_pos, ap)
            }
        }

        RoundResult::Complete
    }

    fn step_towards_closest_target(&self, enemy_kind: &UnitKind, start_pos: usize)
        -> Option<usize>
    {
        struct Node {
            position: usize,
            step_pos: usize,
            distance: usize
        }

        let mut distance_treshold = self.entities.len();
        let mut solutions = Vec::with_capacity(4);

        let mut open_set = VecDeque::with_capacity(self.entities.len());
        let mut closed_set = HashSet::with_capacity(self.entities.len());

        closed_set.insert(start_pos);

        // Queue the free positions around the unit
        for (position, adjacent_entity) in self.adjacents(start_pos) {
            if let Entity::OpenCavern = adjacent_entity {
                closed_set.insert(position);
                open_set.push_back(Node { position, step_pos: position, distance: 0 })
            }
        }

        while let Some(Node { position, step_pos, distance }) = open_set.pop_front() {
            if distance > distance_treshold {
                break
            }

            for (adjacent_pos, adjacent_entity) in self.adjacents(position) {
                if closed_set.contains(&adjacent_pos) {
                    continue
                }

                match adjacent_entity {
                    // The current position is in range of an enemy which makes
                    // it valid candidate. Save it and adjust the threshold
                    Entity::Unit { kind, .. } if kind == enemy_kind => {
                        distance_treshold = distance;
                        solutions.push((position, step_pos))
                    },
                    Entity::OpenCavern => {
                        closed_set.insert(adjacent_pos);
                        open_set.push_back(Node { position: adjacent_pos,
                            step_pos,
                            distance: distance + 1
                        })
                    },
                    _ => ()
                }
            }
        }

        // Return the solution that has the smallest position and break ties
        // using the smallest step position
        solutions
            .into_iter()
            .min()
            .map(|(_target_pos, start_pos)| start_pos)
    }

    fn first_enemy_in_range(&self, enemy_kind: &UnitKind, position: usize) -> Option<usize> {
        self.adjacents(position)
            .filter_map(|(pos, entity)| match entity {
                Entity::Unit { kind, unit } if kind == enemy_kind => Some((pos, unit)),
                _ => None
            })
            .min_by_key(|(_, unit)| unit.hp)
            .map(|(pos, _)| pos)
    }

    fn attack(&mut self, position: usize, power: u32) {
        let enemy = &mut self.entities[position];

        if let Entity::Unit { kind, unit } = enemy {
            unit.hp = unit.hp.saturating_sub(power);
            if unit.hp == 0 {
                match kind {
                    UnitKind::Elf => self.elves_count -= 1,
                    UnitKind::Goblin => self.goblins_count -= 1,
                }
                self.entities[position] = Entity::OpenCavern;
            }
        }
    }

    fn move_unit(&mut self, from: usize, to: usize) {
        // Move the unit and let a free space behind
        let unit_to_move = std::mem::replace(
            &mut self.entities[from],
            Entity::OpenCavern
        );
        self.entities[to] = unit_to_move;
    }

    fn adjacents(&self, position: usize) -> impl Iterator<Item = (usize, &Entity)> {
        let height = self.height as isize;

        arrayvec::ArrayVec::from([-height, -1, 1, height])
            .into_iter()
            .map(move |delta| {
                let position = (position as isize + delta) as usize;
                (position, &self.entities[position])
            })
    }

    fn entities_hps(&self) -> u32 {
        self.entities.iter()
            .filter_map(|entity| match entity {
                Entity::Unit{ unit, .. } => Some(unit.hp),
                _ => None
            })
            .sum()
    }
}

impl Unit {
    fn with_ap(ap: u32) -> Self {
        Self { ap, hp: 200 }
    }
}

impl UnitKind {
    fn enemy(&self) -> Self {
        match self {
            UnitKind::Elf => UnitKind::Goblin,
            UnitKind::Goblin => UnitKind::Elf,
        }
    }

    fn to_entity_with_ap(self, ap: u32) -> Entity {
        Entity::Unit { kind: self, unit: Unit::with_ap(ap) }
    }
}

impl WorldBuilder {
    fn with_elf_ap(&self, ap: u32) -> World {
        let mut elves_count = 0;
        let mut goblins_count = 0;

        let entities = self.parsed_entities.iter()
            .map(|parsed_entity| match parsed_entity {
                ParsedEntity::Wall => Entity::Wall,
                ParsedEntity::OpenCavern => Entity::OpenCavern,
                ParsedEntity::Elf => {
                    elves_count += 1;
                    UnitKind::Elf.to_entity_with_ap(ap)
                },
                ParsedEntity::Goblin => {
                    goblins_count += 1;
                    UnitKind::Goblin.to_entity_with_ap(3)
                },
            })
            .collect();

        World { height: self.height, entities, elves_count, goblins_count }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let tests =
[
(&b"#######
#.G...#
#...EG#
#.#.#G#
#..G#E#
#.....#
#######"[..], 27730),
(b"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",36334),
(b"#######
#G..#E#
#E#E.E#
#G.##.#
#...#E#
#...E.#
#######",36334),
(b"#######
#E..EG#
#.#G.E#
#E.##E#
#G..#.#
#..E#.#
#######", 39514),
(b"#######
#E.G#.#
#.#G..#
#G.#.G#
#G..#.#
#...E.#
#######", 27755),
(b"#######
#.E...#
#.#..G#
#.###.#
#E#G#G#
#...#G#
#######", 28944),
(b"#########
#G......#
#.E.#...#
#..##..G#
#...##..#
#...#...#
#.G...G.#
#.....G.#
#########", 18740)
        ];

        for (input, answer) in tests.iter() {
            let world_builder = parse_input(&input[..]);
            assert_eq!(part1(&world_builder), *answer);
        }
        let world_builder = parse_input(RAW_INPUT);
        assert_eq!(part1(&world_builder), 229798);
    }

    #[test]
    fn p2() {
        let world_builder = parse_input(RAW_INPUT);

        assert_eq!(part2(&world_builder), 52972);
    }
}
