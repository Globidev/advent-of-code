use std::collections::VecDeque;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day17.txt");

pub fn day17() -> (usize, usize) {
    let ranges: Vec<_> = parse_input(RAW_INPUT_STR).collect();

    (part1(&ranges), part2(&ranges))
}

pub fn part1(ranges: &[ClayRange]) -> usize {
    World::new(ranges)
        .fill()
        .into_iter()
        .filter(|&tile| tile == Tile::SettledWater || tile == Tile::FlowingWater)
        .count()
}

pub fn part2(ranges: &[ClayRange]) -> usize {
    World::new(ranges)
        .fill()
        .into_iter()
        .filter(|&tile| tile == Tile::SettledWater)
        .count()
}

pub fn parse_input(input: &str) -> impl Iterator<Item = ClayRange> + '_ {
    input.split('\n').map(|line| {
        let mut coords = line.split(", ");

        let left = coords.next().unwrap();
        let right = coords.next().unwrap();

        let coord_left = left[2..].parse().unwrap();
        let mut range_right = right[2..].split("..");

        let from_right = range_right.next().unwrap().parse().unwrap();
        let to_right = range_right.next().unwrap().parse().unwrap();

        match line.bytes().next().unwrap() {
            b'x' => (coord_left..=coord_left, from_right..=to_right),
            b'y' => (from_right..=to_right, coord_left..=coord_left),
            invalid => unreachable!("Invalid coord {}", invalid)
        }
    })
}

use std::ops::RangeInclusive;
type ClayRange = (RangeInclusive<usize>, RangeInclusive<usize>);
type Position = (usize, usize);

struct World {
    map: Vec<Vec<Tile>>,
    min_height: usize,
    max_height: usize
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Tile {
    Sand,
    Clay,
    Spring,
    SettledWater,
    FlowingWater,
}

enum HorizontalFlow {
    Left,
    Right,
}

enum DownwardsFlowResult {
    OutOfBounds,
    MergedWithExistingStream,
    Reached(Position),
}

#[derive(Clone, Copy)]
enum HorizontalFlowResult {
    Stopped(usize),
    ReachedEdge(usize),
}

use self::HorizontalFlow::*;
use self::DownwardsFlowResult::*;
use self::HorizontalFlowResult::*;

const SPRING_POSITION: Position = (500, 0);

fn is_permeable(tile: Tile) -> bool {
    tile == Tile::Sand || tile == Tile::FlowingWater
}

impl World {
    fn new(ranges: &[ClayRange]) -> Self {
        let max_width = *ranges.iter().map(|(xs, _)| xs.end()).max().unwrap();
        let min_height = *ranges.iter().map(|(_, ys)| ys.start()).min().unwrap();
        let max_height = *ranges.iter().map(|(_, ys)| ys.end()).max().unwrap();

        let mut map = vec![vec![Tile::Sand; max_width+1]; max_height+1];
        map[0][500] = Tile::Spring;

        for (xs, ys) in ranges {
            for y in ys.clone() {
                for x in xs.clone() {
                    map[y][x] = Tile::Clay
                }
            }
        }

        Self { map, min_height, max_height }
    }

    fn fill(mut self) -> Self {
        let mut streams = VecDeque::new();
        streams.push_back(SPRING_POSITION);

        while let Some(stream) = streams.pop_front() {
            let (x, y) = match self.flow_downwards(stream) {
                // A stream can be stopped when it goes OOB or merges
                OutOfBounds | MergedWithExistingStream => continue,
                Reached(new_position) => new_position
            };

            let left_stream = self.flow_horizontally((x, y), Left);
            let right_stream = self.flow_horizontally((x, y), Right);

            // If both streams stopped then water can settle
            if let (Stopped(x_left), Stopped(x_right)) = (left_stream, right_stream) {
                self.settle_water(x_left, x_right, y);
                streams.push_back((x, y - 1)) // Go back up the current stream
            }
            // Otherwise add the streams that are not blocked
            if let ReachedEdge(x_left) = left_stream {
                streams.push_back((x_left, y))
            }
            if let ReachedEdge(x_right) = right_stream {
                streams.push_back((x_right, y))
            }
        }

        self
    }

    fn flow_downwards(&mut self, (x, mut y): Position) -> DownwardsFlowResult {
        let start_y = y;

        loop {
            if y >= self.max_height {
                self.map[y][x] = Tile::FlowingWater;
                return OutOfBounds
            }

            let tile_below = self.at((x, y+1));

            if is_permeable(tile_below) {
                self.map[y][x] = Tile::FlowingWater;
                y += 1;
            } else {
                if y > start_y && self.at((x, y)) == Tile::FlowingWater {
                    return MergedWithExistingStream
                } else {
                    return Reached((x, y))
                }
            }
        }
    }

    fn flow_horizontally(&mut self, (mut x, y): Position, direction: HorizontalFlow)
        -> HorizontalFlowResult
    {
        let moved = |x| match direction {
            Left => x - 1,
            Right => x + 1
        };

        self.map[y][x] = Tile::FlowingWater;

        loop {
            let next_x = moved(x);
            let next_tile = self.at((next_x, y));

            if is_permeable(next_tile) {
                x = next_x;
                self.map[y][x] = Tile::FlowingWater;
            }
            else {
                return Stopped(x)
            }

            let tile_below = self.at((x, y+1));

            if is_permeable(tile_below) {
                return ReachedEdge(x)
            }
        }
    }

    fn settle_water(&mut self, from_x: usize, to_x: usize, y: usize) {
        let settled_water = vec![Tile::SettledWater; to_x - from_x + 1];
        self.map[y][from_x..=to_x].copy_from_slice(&settled_water);
    }

    fn at(&mut self, (x, y): Position) -> Tile {
        let row = &mut self.map[y];
        if x == row.len() {
            row.push(Tile::Sand)
        }
        row[x]
    }

    fn into_iter(self) -> impl Iterator<Item = Tile> {
        self.map.into_iter()
            .skip(self.min_height)
            .flat_map(|rows| rows)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let ranges: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&ranges), 30737);
    }

    #[test]
    fn p2() {
        let ranges: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&ranges), 24699);
    }
}
