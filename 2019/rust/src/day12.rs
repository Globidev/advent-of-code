use std::fmt::Debug;
use std::ops::{Add, AddAssign, Sub};
use std::iter::Sum;
use itertools::Itertools;
use num::Integer;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day12.txt");

pub fn day12() -> impl Debug {
    let positions = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&positions), part2(&positions))
}

pub fn part1(positions: &[Vec3]) -> i32 {
    const SIM_STEPS: usize = 1_000;

    let mut moons = positions.iter()
        .map(|&position| Moon { position, velocity: Vec3::default() })
        .collect_vec();

    for _ in 0..SIM_STEPS {
        for moon_idx in 0..moons.len() {
            let moon = &moons[moon_idx];
            let other_moons = Iterator::chain(
                moons[..moon_idx].iter(),
                moons[moon_idx + 1..].iter(),
            );

            let delta_velocity = other_moons
                .map(|other_moon| {
                    let delta_pos = other_moon.position - moon.position;
                    delta_pos
                        .min(Vec3::new(1, 1, 1))
                        .max(Vec3::new(-1, -1, -1))
                })
                .sum();

            moons[moon_idx].velocity += delta_velocity;
        }

        for moon in &mut moons {
            moon.position += moon.velocity;
        }
    }

    moons.iter()
        .map(Moon::energy)
        .sum()
}

pub fn part2(positions: &[Vec3]) -> usize {
    let moons = positions.iter()
        .map(|&position| Moon { position, velocity: Vec3::default() })
        .collect_vec();

    let next_state = |mut moons: Vec<Moon>| {
        for moon_idx in 0..moons.len() {
            let moon = &moons[moon_idx];
            let other_moons = Iterator::chain(
                moons[..moon_idx].iter(),
                moons[moon_idx + 1..].iter(),
            );

            let delta_velocity = other_moons
                .map(|other_moon| {
                    let delta_pos = other_moon.position - moon.position;
                    delta_pos
                        .min(Vec3::new(1, 1, 1))
                        .max(Vec3::new(-1, -1, -1))
                })
                .sum();

            moons[moon_idx].velocity += delta_velocity;
        }

        for moon in &mut moons {
            moon.position += moon.velocity;
        }

        moons
    };

    let eq_on = |getter: fn(Vec3) -> i32| {
        move |m1: &Vec<Moon>, m2: &Vec<Moon>| Iterator::eq(
            m1.iter().map(|m| (getter(m.position), getter(m.velocity))),
            m2.iter().map(|m| (getter(m.position), getter(m.velocity))),
        )
    };

    let (lam_x, _) = floyd(moons.clone(), next_state, eq_on(|v| v.x));
    let (lam_y, _) = floyd(moons.clone(), next_state, eq_on(|v| v.y));
    let (lam_z, _) = floyd(moons.clone(), next_state, eq_on(|v| v.z));

    lam_x.lcm(&lam_y).lcm(&lam_z)
}

fn floyd<T: Clone, F: Fn(T) -> T, Cmp: Fn(&T, &T) -> bool>(x0: T, f: F, cmp: Cmp)
    -> (usize, usize)
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

    let mut lam = 1;
    let mut hare = f(tortoise.clone());
    while !cmp(&tortoise, &hare) {
        hare = f(hare);
        lam += 1
    }

    (lam, mu)
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Moon {
    position: Vec3,
    velocity: Vec3
}

impl Moon {
    fn potential_energy(&self) -> i32 {
        let Vec3 { x, y, z } = self.position;
        x.abs() + y.abs() + z.abs()
    }

    fn kinetic_energy(&self) -> i32 {
        let Vec3 { x, y, z } = self.velocity;
        x.abs() + y.abs() + z.abs()
    }

    fn energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug, Default, Clone, Copy, Hash, PartialEq, Eq)]
pub struct Vec3 {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3 {
    pub fn new(x: i32, y: i32, z: i32) -> Self { Self { x, y, z } }

    fn min(self, rhs: Self) -> Self {
        Self {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
            z: self.z.min(rhs.z),
        }
    }

    fn max(self, rhs: Self) -> Self {
        Self {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
            z: self.z.max(rhs.z),
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) { *self = *self + rhs }
}

impl Sum for Vec3 {
    fn sum<I: Iterator<Item = Vec3>>(iter: I) -> Self {
        iter.fold(Vec3::default(), Add::add)
    }
}

pub fn parse_input(input: &str) -> impl Iterator<Item = Vec3> + '_ {
    input.lines()
        .map(|raw_pos| {
            let stripped = &raw_pos[1..raw_pos.len()-1]; // skip '<' and '>'
            let mut components = stripped.split(", ")
                .map(|raw_comp| raw_comp[2..].parse().expect("Malformed axis value"));
            let (x, y, z) = components.next_tuple().expect("Missing axis component");
            Vec3 { x, y, z }
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 7077);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 402_951_477_454_512);
    }
}
