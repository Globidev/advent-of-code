use std::fmt::Debug;
use std::cmp::{Ord, Ordering};
use std::hash::{Hash, Hasher};
use std::collections::BTreeMap;
use std::f32::consts::PI;

use itertools::Itertools;
use rayon::prelude::*;
use packed_simd::f32x2;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day10.txt");

pub fn day10() -> impl Debug {
    let asteroids = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&asteroids), part2(&asteroids))
}

pub fn part1(asteroids: &[Pos]) -> usize {
    find_best_station(asteroids).visible_asteroids
}

pub fn part2(asteroids: &[Pos]) -> usize {
    let Station { position: laser_pos, idx, .. } = find_best_station(asteroids);

    let mut laser_hit_map = BTreeMap::new();
    let asteroids = Iterator::chain(
        asteroids[..idx].iter(),
        asteroids[idx+1..].iter(),
    );

    #[derive(Debug, Clone, Copy)]
    struct AsteroidInfo {
        laser_distance: f32,
        position: Pos,
    }

    let hit_infos = asteroids
        .map(|&asteroid_pos| {
            let angle = (-laser_pos.angle(asteroid_pos)).rem_euclid(2. * PI);
            let info = AsteroidInfo {
                laser_distance: laser_pos.distance(asteroid_pos),
                position: asteroid_pos
            };
            (angle, info)
        });

    for (angle, info) in hit_infos {
        laser_hit_map.entry(FloatOrd(angle))
            .and_modify(|prev_info: &mut AsteroidInfo| {
                if info.laser_distance < prev_info.laser_distance {
                    *prev_info = info
                }
            })
            .or_insert(info);
    }

    let target_asteroid_info = laser_hit_map.values()
        .nth(199)
        .expect("Not enough direct targets");

    let [x, y] = target_asteroid_info.position.components();

    (x * 100. + y) as _
}

fn find_best_station(asteroids: &[Pos]) -> Station {
    asteroids.par_iter()
        .enumerate()
        .map(|(idx, &station_pos)| {
            let other_asteroids = Iterator::chain(
                asteroids[..idx].iter(),
                asteroids[idx+1..].iter()
            );

            let visible_asteroids = other_asteroids
                .map(|&asteroid_pos| station_pos.angle(asteroid_pos))
                .map(FloatOrd)
                .unique()
                .count();

            Station {
                position: station_pos,
                visible_asteroids,
                idx,
            }
        })
        .max_by_key(|station| station.visible_asteroids)
        .expect("No asteroids")
}

struct Station {
    position: Pos,
    visible_asteroids: usize,
    idx: usize,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pos(f32x2);

impl Pos {
    fn distance(self, other: Self) -> f32 {
        (self.0 - other.0).abs().sum()
    }

    fn angle(self, other: Self) -> f32 {
        let [x, y] = Pos(self.0 - other.0).components();
        x.atan2(y)
    }

    fn components(self) -> [f32; 2] {
        [
            self.0.extract(0),
            self.0.extract(1),
        ]
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct FloatOrd(f32);

impl Ord for FloatOrd {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).expect("Got NaNs")
    }
}

impl Eq for FloatOrd { }

impl Hash for FloatOrd {
    fn hash<H: Hasher>(&self, state: &mut H) {
        state.write_u32(self.0.to_bits())
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Pos> + '_ {
    input.lines()
        .enumerate()
        .flat_map(|(y, line)|
            line.chars()
                .enumerate()
                .filter_map(move |(x, ch)| match ch {
                    '#' => Some(Pos(f32x2::new(x as _, y as _))),
                    _ => None
                })
        )
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 292);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 317);
    }
}
