use rayon::prelude::*;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day06.txt");

pub fn day06() -> (u32, u32) {
    let coords: Vec<_> = parse_coordinates(RAW_INPUT_STR).collect();

    (part1(&coords), part2(&coords))
}

pub fn part1(coords: &[Coordinate]) -> u32 {
    const LANES: usize = i16x32::lanes();

    let padding = (LANES - (coords.len() % LANES)) % LANES;
    let xs: Vec<_> = coords.iter().map(|c| c.x)
        .chain(std::iter::repeat(i16::max_value() / 2).take(padding))
        .collect();
    let ys: Vec<_> = coords.iter().map(|c| c.y)
        .chain(std::iter::repeat(i16::max_value() / 2).take(padding))
        .collect();

    // Making a pretty balsy assumption here that the input will be <= 64
    // This wouldn't be necessary with a bit more work in `closest_to`
    let p1_xs_1 = i16x32::from_slice_unaligned(&xs[0..0+LANES]);
    let p1_xs_2 = i16x32::from_slice_unaligned(&xs[LANES..LANES+LANES]);
    let p1_ys_1 = i16x32::from_slice_unaligned(&ys[0..0+LANES]);
    let p1_ys_2 = i16x32::from_slice_unaligned(&ys[LANES..LANES+LANES]);

    let idxs_1 = i16x32::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
    let idxs_2 = i16x32::new(32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49, 50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63);

    let closest_to = |x, y| {
        use std::cmp::Ordering::*;

        let p2_xs = i16x32::splat(x);
        let p2_ys = i16x32::splat(y);

        let dists_1 = manhattan_simd(p1_xs_1, p1_ys_1, p2_xs, p2_ys);
        let dists_2 = manhattan_simd(p1_xs_2, p1_ys_2, p2_xs, p2_ys);

        let min_1 = dists_1.min_element();
        let min_2 = dists_2.min_element();

        let (mins, idxs) = match min_1.cmp(&min_2) {
            Equal   => return None,
            Less    => (min_1, idxs_1),
            Greater => (min_2, idxs_2),
        };

        let mask = i16x32::splat(mins);
        let mins = dists_1.eq(mask);
        if mins.select(ONES, ZEROES).wrapping_sum() > 1 {
            None
        } else {
            Some(mins.select(idxs, ZEROES).wrapping_sum() as usize)
        }
    };

    let (top, bottom, left, right) = bounding_rect(coords);

    let mut counts = (left..right+1)
        .into_par_iter()
        .fold(|| vec![0; coords.len()], |mut counts, x| {
            for y in top..=bottom {
                let _ = closest_to(x, y).map(|id| counts[id] += 1);
            }
            counts
        })
        .reduce(|| vec![0; coords.len()], |mut total, counts| {
            for (i, c) in counts.into_iter().enumerate() {
                total[i] += c;
            }
            total
        });

    for x in left-1..=right+1 {
        let _ = closest_to(x, top - 1).map(|id| counts[id] = 0);
        let _ = closest_to(x, bottom + 1).map(|id| counts[id] = 0);
    }

    for y in top-1..=bottom+1 {
        let _ = closest_to(left - 1, y).map(|id| counts[id] = 0);
        let _ = closest_to(right + 1, y).map(|id| counts[id] = 0);
    }

    counts.into_iter().max().expect("No coordinates")
}

pub fn part2(coords: &[Coordinate]) -> u32 {
    const OFFSETS: i16x32 = i16x32::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31);
    const COUNT_MASK: i16x32 = i16x32::splat(10_000);

    let (top, bottom, left, right) = bounding_rect(coords);

    let distances = |x, y_start| {
        let p2_xs = i16x32::splat(x);
        let p2_ys = y_start + OFFSETS;

        coords.iter()
            .fold(ZEROES, |total, &Coordinate { x, y }| {
                let p1_xs = i16x32::splat(x);
                let p1_ys = i16x32::splat(y);
                total + manhattan_simd(p1_xs, p1_ys, p2_xs, p2_ys)
            })
    };

    (left..right+1)
        .into_par_iter()
        .map(|x|
            (top..bottom+1).step_by(i16x32::lanes())
                .map(|y_start|
                    distances(x, y_start).lt(COUNT_MASK)
                        .select(ONES, ZEROES)
                        .wrapping_sum() as u32
                )
                .sum::<u32>()
        )
        .sum()
}

pub fn parse_coordinates(input: &str) -> impl Iterator<Item = Coordinate> + '_ {
    input.lines()
        .map(|line| {
            let mut splitted = line.split(", ");
            Coordinate {
                x: splitted.next().expect("Missing x").parse().expect("Invalid x"),
                y: splitted.next().expect("Missing y").parse().expect("Invalid y"),
            }
        })
}

fn bounding_rect(coords: &[Coordinate]) -> (i16, i16, i16, i16) {
    let top = coords.iter().map(|c| c.y).min().unwrap();
    let left = coords.iter().map(|c| c.x).min().unwrap();
    let bottom = coords.iter().map(|c| c.y).max().unwrap();
    let right = coords.iter().map(|c| c.x).max().unwrap();

    (top, bottom, left, right)
}

use packed_simd::{i16x32};

fn manhattan_simd(p1_xs: i16x32, p1_ys: i16x32, p2_xs: i16x32, p2_ys: i16x32) -> i16x32 {
    abs_simd(p2_xs - p1_xs) + abs_simd(p2_ys - p1_ys)
}

fn abs_simd(vec: i16x32) -> i16x32 {
    let mask = vec >> (std::mem::size_of::<i16>() * 8 - 1) as u32;
    (vec + mask) ^ mask
}

const ZEROES: i16x32 = i16x32::splat(0);
const ONES: i16x32 = i16x32::splat(1);

pub struct Coordinate {
    x: i16,
    y: i16,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let coords: Vec<_> = parse_coordinates(RAW_INPUT_STR).collect();

        assert_eq!(part1(&coords), 3238);
    }

    #[test]
    fn p2() {
        let coords: Vec<_> = parse_coordinates(RAW_INPUT_STR).collect();

        assert_eq!(part2(&coords), 45046);
    }
}
