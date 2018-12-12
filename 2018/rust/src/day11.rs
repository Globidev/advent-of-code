const INPUT: u16 = 1723;

pub fn day11() -> ((usize, usize), (usize, usize, usize)) {
    ((part1(INPUT)), part2(INPUT))
}

pub fn part1(serial_number: u16) -> (usize, usize) {
    let grid = fuel_grid(serial_number);

    let mut max_x = 0;
    let mut max_y = 0;
    let mut max = 0;

    for y in 0..=300-3 {
        for x in 0..=300-3 {
            let sum = level_sum(x, y, 3, &grid);
            if sum > max {
                max = sum;
                max_x = x;
                max_y = y;
            }
        }
    }

    (max_x, max_y)
}

use rayon::prelude::*;
use packed_simd::i32x16;

pub fn part2(serial_number: u16) -> (usize, usize, usize) {
    const IDXS: i32x16 = i32x16::new(0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15);
    const ZEROES: i32x16 = i32x16::splat(0);

    const M: i32 = i32::min_value() / 2;
    const MASKS: [i32x16; 16] = [
        i32x16::new(0, M, M, M, M, M, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, M, M, M, M, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, M, M, M, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, M, M, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, M, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, M, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, M, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, M, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, M, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, M),
        i32x16::new(0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0),
    ];

    let mut grid = [0; 301 * 301];

    for y in 1..=300 {
        for x in 1..=300 {
            grid[y * 300 + x] = power_level(x, y, serial_number) as i32 +
                                grid[(y-1) * 300 + x] +
                                grid[y * 300 + (x-1)] -
                                grid[(y-1) * 300 + (x-1)];
        }
    }

    let (_, max_x, max_y, max_s) = (1_usize..300+1)
        .into_par_iter()
        .map(|s| {
            let mut max_x = 0;
            let mut max_y = 0;
            let mut max = 0;

            for y in s..=300 {
                for x in (s..=300).step_by(i32x16::lanes()) {
                    let br = i32x16::from_slice_unaligned(&grid[y * 300 + x..]);
                    let tr = i32x16::from_slice_unaligned(&grid[(y-s) * 300 + x..]);
                    let bl = i32x16::from_slice_unaligned(&grid[y * 300 + (x-s)..]);
                    let tl = i32x16::from_slice_unaligned(&grid[(y-s) * 300 + (x-s)..]);

                    let mut totals = br - tr - bl + tl;

                    if x + i32x16::lanes() > 300 {
                        totals += MASKS[300-x];
                    }

                    let total = totals.max_element();
                    if total > max {
                        let masked = totals.eq(i32x16::splat(total));
                        let idx = masked.select(IDXS, ZEROES);
                        max = total; max_x = x + idx.wrapping_sum() as usize; max_y = y;
                    }
                }
            }
            (max, max_x - s + 1, max_y - s + 1, s)
        })
        .max_by_key(|(mm, ..)| *mm)
        .unwrap();

    (max_x, max_y, max_s)
}

fn fuel_grid(serial_number: u16) -> FuelGrid {
    let mut fuel_grid = [0; 300 * 300];

    for x in 0..300 {
        for y in 0..300 {
            fuel_grid[y * 300 + x] = power_level(x, y, serial_number);
        }
    }

    fuel_grid
}

fn power_level(x: usize, y: usize, serial_number: u16) -> Cell {
    let rack_id = x + 10;
    let level = rack_id * y;
    let level = level + serial_number as usize;
    let level = level * rack_id;
    ((level / 100) % 10) as i8 - 5
}

fn level_sum(x: usize, y: usize, size: usize, grid: &FuelGrid) -> i32 {
    let mut total = 0;
    for y in y..y+size {
        for x in x..x+size {
            total += grid[y * 300 + x] as i32
        }
    }
    total
}

type FuelGrid = [Cell; 300 * 300];
type Cell = i8;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), (34, 13));
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT), (280, 218, 11));
    }
}
