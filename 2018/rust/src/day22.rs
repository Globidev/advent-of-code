use std::collections::BinaryHeap;
use std::cmp::Ordering;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day22.txt");

pub fn day22() -> (usize, usize) {
    let (depth, target) = parse_input(RAW_INPUT_STR);

    (part1(depth, target), part2(depth, target))
}

pub fn part1(depth: usize, target: Position) -> usize {
    let mut cave = Cave::new(depth, target);

    let (x, y) = target;
    (0..=y).flat_map(|y| (0..=x).map(move |x| (x, y)))
        .map(|pos| cave.at(pos).kind as usize)
        .sum()
}

pub fn part2(depth: usize, target: Position) -> usize {
    let mut cave = Cave::new(depth, target);

    #[derive(PartialEq, Eq)]
    struct Node {
        time: usize,
        estimate: usize,
        position: Position,
        equipment: Equipment,
    }

    let new_node = |position, equipment, time| Node {
        position,
        equipment,
        time,
        estimate: time + manhattan(position, target)
    };

    impl PartialOrd for Node {
        fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
            Some(self.cmp(other))
        }
    }

    impl Ord for Node {
        fn cmp(&self, other: &Node) -> Ordering {
            other.estimate.cmp(&self.estimate)
        }
    }

    let mut open_set = BinaryHeap::new();
    let mut closed_set = hashbrown::HashMap::new();

    open_set.push(new_node((0, 0), Equipment::Torch, 0));

    while let Some(Node { position, equipment, time, .. }) = open_set.pop() {
        if position == target && equipment == Equipment::Torch {
            return time
        }

        let possible_actions = {
            let swap_equipment = (
                position,
                equipment.alternative(cave.at(position).kind),
                time + 7
            );

            let move_around = neighboring_positions(position)
                .filter(|&position| cave.at(position).kind.can_use(equipment))
                .map(|position| (position, equipment, time + 1));

            std::iter::once(swap_equipment).chain(move_around)
        };

        for (position, equipment, time) in possible_actions {
            match closed_set.get(&(position, equipment)) {
                Some(&existing_time) if time >= existing_time => (),
                _unvisited_or_with_a_better_time => {
                    open_set.push(new_node(position, equipment, time));
                    closed_set.insert((position, equipment), time);
                }
            }
        }
    }

    panic!("No path to the target!")
}

fn manhattan((x1, y1): Position, (x2, y2): Position) -> usize {
    (x1 as isize - x2 as isize).abs() as usize +
    (y1 as isize - y2 as isize).abs() as usize
}

fn neighboring_positions((x, y): Position) -> impl Iterator<Item = Position> {
    use arrayvec::ArrayVec;
    let deltas = ArrayVec::from([(1, 0), (0, 1), (-1, 0), (0, -1)]);

    deltas.into_iter()
        .filter_map(move |(dx, dy)| {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx < 0 || ny < 0 { None }
            else { Some((nx as usize, ny as usize)) }
        })
}

pub fn parse_input(input: &str) -> (usize, Position) {
    let mut lines = input.lines();

    let depth = lines.next().unwrap()[7..].parse().unwrap();
    let mut target_split = lines.next().unwrap()[8..].split(',');

    let target_x = target_split.next().unwrap().parse().unwrap();
    let target_y = target_split.next().unwrap().parse().unwrap();

    (depth, (target_x, target_y))
}

type Position = (usize, usize);

#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(dead_code)] // We're only constructing variants by transmuting
enum RegionKind {
    Rocky = 0,
    Wet = 1,
    Narrow = 2,
}

impl RegionKind {
    fn can_use(&self, equipment: Equipment) -> bool {
        match (self, equipment) {
            (RegionKind::Rocky,  Equipment::None)         => false,
            (RegionKind::Wet,    Equipment::Torch)        => false,
            (RegionKind::Narrow, Equipment::ClimbingGear) => false,
            _ => true
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Region {
    kind: RegionKind,
    geologic_index: usize,
    erosion_level: usize,
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Equipment {
    Torch,
    ClimbingGear,
    None
}

impl Equipment {
    fn alternative(&self, kind: RegionKind) -> Self {
        use self::Equipment::*;

        match (kind, self) {
            (RegionKind::Rocky,  Torch)        => ClimbingGear,
            (RegionKind::Rocky,  ClimbingGear) => Torch,
            (RegionKind::Wet,    ClimbingGear) => None,
            (RegionKind::Wet,    None)         => ClimbingGear,
            (RegionKind::Narrow, Torch)        => None,
            (RegionKind::Narrow, None)         => Torch,
            _ => unreachable!("Broke equipment invariant")
        }
    }
}

impl Region {
    fn new(geologic_index: usize, depth: usize) -> Self {
        let erosion_level = erosion_level(geologic_index, depth);
        let kind = unsafe { std::mem::transmute((erosion_level % 3) as u8) };

        Self { kind, geologic_index, erosion_level }
    }
}

struct Cave {
    depth: usize,
    regions: Vec<Vec<Region>>,
}


fn erosion_level(geologic_index: usize, depth: usize) -> usize {
    const EROSION_MOD: usize = 20_183;

    (geologic_index + depth) % EROSION_MOD
}

impl Cave {
    fn new(depth: usize, (tx, ty): Position) -> Self {
        let first_row = std::iter::once(0)
            .chain((1..=tx).map(|x| x * 16_807))
            .map(|geo_index| Region::new(geo_index, depth))
            .collect();

        let mut regions = Vec::with_capacity(ty+1);
        regions.push(first_row);

        let mut this = Self { depth, regions };
        for _ in 1..=ty { this.add_row() }

        this.regions[ty][tx] = Region::new(0, depth);

        this
    }

    fn add_row(&mut self) {
        let y = self.regions.len();
        let row_up = self.regions.last().unwrap();

        let mut row = Vec::with_capacity(row_up.capacity());
        row.push(Region::new(y * 48_271, self.depth));

        for x in 1..row_up.len() {
            let left = row[x-1];
            let up = self.regions[y-1][x];
            let geo_index = left.erosion_level * up.erosion_level;
            row.push(Region::new(geo_index, self.depth));
        }

        self.regions.push(row);
    }

    fn add_column(&mut self) {
        let first_row = &mut self.regions[0];
        let x = first_row.len();

        first_row.push(Region::new(x * 16_807, self.depth));

        for y in 1..self.regions.len() {
            let left = self.regions[y][x-1];
            let up = self.regions[y-1][x];
            let geo_index = left.erosion_level * up.erosion_level;
            self.regions[y].push(Region::new(geo_index, self.depth))
        }

    }

    fn at(&mut self, (x, y): Position) -> &Region {
        while y >= self.regions.len() { self.add_row() }
        while x >= self.regions[y].len() { self.add_column() }

        &self.regions[y][x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let (depth, target) = parse_input(RAW_INPUT_STR);

        assert_eq!(part1(depth, target), 10395);
    }

    #[test]
    fn p2() {
        let (depth, target) = parse_input(RAW_INPUT_STR);

        assert_eq!(part2(depth, target), 1010);
    }
}
