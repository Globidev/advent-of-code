const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day18.txt");

pub fn day18() -> (u32, u32) {
    let world = parse_input(RAW_INPUT);

    (part1(&world), part2(&world))
}

pub fn part1(world: &World) -> u32 {
    (0..10)
        .fold(world.to_simd_world(), |world, _| world.tick())
        .resource_value()
}

pub fn part2(world: &World) -> u32 {
    let (cycle_len, cycle_start, world_after_cycle) = floyd(
        world.to_simd_world(),
        SimdWorld::tick,
        PartialEq::eq
    );

    let remaining_steps = (1_000_000_000 - cycle_start) % cycle_len;

    (0..remaining_steps)
        .fold(world_after_cycle, |world, _| world.tick())
        .resource_value()
}

fn floyd<T: Clone, F: Fn(T) -> T, Cmp: Fn(&T, &T) -> bool>(x0: T, f: F, cmp: Cmp)
    -> (usize, usize, T)
{
    let mut tortoise = f(x0.clone());
    let mut hare = f(f(x0.clone()));

    while !cmp(&tortoise, &hare) {
        tortoise = f(tortoise);
        hare = f(f(hare));
    }

    let mut mu = 0;
    let mut tortoise = x0;
    while !cmp(&tortoise, &hare) {
        tortoise = f(tortoise);
        hare = f(hare);
        mu += 1
    }

    let after_cycle = hare;
    let mut lam = 1;
    let mut hare = f(tortoise.clone());
    while !cmp(&tortoise, &hare) {
        hare = f(hare);
        lam += 1
    }

    (lam, mu, after_cycle)
}

pub fn parse_input(input: &[u8]) -> World {
    let tiles = input.split(|&c| c == b'\n')
        .flat_map(|line| {
            line.iter().map(|c| match c {
                b'.' => Tile::OpenGround,
                b'|' => Tile::Tree,
                b'#' => Tile::Lumberyard,
                invalid => panic!("Invalid tile byte: {}", invalid)
            })
        })
        .collect();

    World { tiles }
}

use packed_simd::{u8x64, shuffle, m8x64};

#[derive(Clone)]
pub struct SimdWorld {
    tiles: [u8x64; 52],
}

impl PartialEq for SimdWorld {
    fn eq(&self, other: &SimdWorld) -> bool {
        self.tiles[1..51]
            .iter()
            .zip(other.tiles[1..51].iter())
            .all(|(t1, t2)| t1 == t2)
    }
}

fn shuffle_right(v: u8x64) -> u8x64 {
    shuffle!(
        v,
        [
            63, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
            23, 24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44,
            45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62
        ]
    )
}

fn shuffle_left(v: u8x64) -> u8x64 {
    shuffle!(
        v,
        [
            1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24,
            25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46,
            47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 0
        ]
    )
}

const ZEROS: u8x64 = u8x64::splat(0);
const ONES: u8x64 = u8x64::splat(1);
const TWOS: u8x64 = u8x64::splat(2);
const THREES: u8x64 = u8x64::splat(3);
const SIXES: u8x64 = u8x64::splat(6);
const OPEN_GROUNDS: u8x64 = u8x64::splat(Tile::OpenGround as u8);
const TREES: u8x64 = u8x64::splat(Tile::Tree as u8);
const LUMBERYARDS: u8x64 = u8x64::splat(Tile::Lumberyard as u8);
const AREA_MASK: m8x64 = m8x64::new(
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, true, true, true, true, true, true, true, true, true, true,
    true, true, true, true, true, false, false, false, false, false, false, false, false, false,
    false, false, false, false, false,
);

impl SimdWorld {
    fn tick(self) -> SimdWorld {
        let mut tiles = [OPEN_GROUNDS; 52];

        for (dst, src_rows) in tiles[1..51].iter_mut().zip(self.tiles.windows(3)) {
            let top = src_rows[0];
            let mid = src_rows[1];
            let bot = src_rows[2];

            let top_left  = shuffle_left(top);
            let top_right = shuffle_right(top);
            let mid_left  = shuffle_left(mid);
            let mid_right = shuffle_right(mid);
            let bot_left  = shuffle_left(bot);
            let bot_right = shuffle_right(bot);

            let tree_counts =
                (top_left  & TREES) +
                (top       & TREES) +
                (top_right & TREES) +
                (mid_left  & TREES) +
                (mid_right & TREES) +
                (bot_left  & TREES) +
                (bot       & TREES) +
                (bot_right & TREES)
            ;

            let yard_counts =
                (top_left  & LUMBERYARDS) +
                (top       & LUMBERYARDS) +
                (top_right & LUMBERYARDS) +
                (mid_left  & LUMBERYARDS) +
                (mid_right & LUMBERYARDS) +
                (bot_left  & LUMBERYARDS) +
                (bot       & LUMBERYARDS) +
                (bot_right & LUMBERYARDS)
            ;

            let open_to_tree = tree_counts.ge(THREES).select(TREES, OPEN_GROUNDS);
            let tree_to_yard = yard_counts.ge(SIXES).select(LUMBERYARDS, TREES);
            let yard_to_open = (tree_counts.ge(ONES) & yard_counts.ge(TWOS))
                .select(LUMBERYARDS, OPEN_GROUNDS);

            let is_tree = mid.eq(TREES);
            let is_yard = mid.eq(LUMBERYARDS);

            let transformed = is_tree.select(
                tree_to_yard,
                is_yard.select(yard_to_open, open_to_tree)
            );

            *dst = AREA_MASK.select(transformed, ZEROS);
        }

        SimdWorld { tiles }
    }

    fn resource_value(&self) -> u32 {
        let (trees, lumberyards) = self.tiles[1..51]
            .iter()
            .fold((0, 0), |(trees, lumberyards), &row| (
                trees + (row & TREES).wrapping_sum() as u32,
                lumberyards + (row & LUMBERYARDS).wrapping_sum() as u32 / 2
            ));

        trees * lumberyards
    }
}

#[derive(Debug, Clone)]
pub struct World {
    tiles: Vec<Tile>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    OpenGround = 0,
    Tree = 1,
    Lumberyard = 2,
}

impl World {
    fn to_simd_world(&self) -> SimdWorld {
        let padding = u8x64::splat(Tile::OpenGround as u8);

        let mut tiles = [padding; 52];

        (1..51).for_each(|i| {
            let mut arr = [Tile::OpenGround as u8; 64];

            (0..50).for_each(|x| {
                arr[x] = self.tiles[(i-1) * 50 + x] as u8
            });

            tiles[i] = u8x64::from_slice_unaligned(&arr);
        });

        SimdWorld { tiles }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let world = parse_input(RAW_INPUT);

        assert_eq!(part1(&world), 582494);
    }

    #[test]
    fn p2() {
        let world = parse_input(RAW_INPUT);

        assert_eq!(part2(&world), 174584);
    }
}
