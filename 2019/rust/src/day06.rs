use std::fmt::Debug;
use itertools::Itertools;
use std::collections::{HashSet, HashMap, VecDeque};

const RAW_INPUT_STR: &str = include_str!("../../inputs/day06.txt");

pub fn day06() -> impl Debug {
    let relations = parse_input(RAW_INPUT_STR).collect_vec();

    (part1(&relations), part2(&relations))
}

pub fn part1(orbit_relations: &[OrbitRelation]) -> usize {
    let uniq_objs = orbit_relations.iter()
        .flatten()
        .collect::<HashSet<_>>();

    let mut orbit_map = HashMap::<&str, Vec<&str>>::new();

    for [obj1, obj2] in orbit_relations {
        orbit_map.entry(obj1)
            .or_default()
            .push(obj2)
    }

    uniq_objs.iter()
        .map(|obj| count_orbits(obj, &orbit_map))
        .sum()
}

pub fn part2(orbit_relations: &[OrbitRelation]) -> usize {
    const SANTA_OBJ_NAME: &str = "SAN";
    const YOU_OBJ_NAME: &str = "YOU";

    let mut orbit_map = HashMap::<&str, Vec<&str>>::new();

    for [obj1, obj2] in orbit_relations {
        orbit_map.entry(obj1)
            .or_default()
            .push(obj2);
        orbit_map.entry(obj2)
            .or_default()
            .push(obj1);
    }

    let start = orbit_relations.iter()
        .find_map(|[obj1, obj2]| if obj2 == YOU_OBJ_NAME { Some(obj1) } else { None })
        .expect("No orbit found for ME!");

    let target = orbit_relations.iter()
        .find_map(|[obj1, obj2]| if obj2 == SANTA_OBJ_NAME { Some(obj1) } else { None })
        .expect("No orbit found for Santa!");

    shortest_route(&orbit_map, start, target)
}

fn count_orbits(obj: &str, orbit_map: &HashMap<&str, Vec<&str>>) -> usize {
    orbit_map.get(obj)
        .map(|objs| objs.len() + objs.iter().map(|obj| count_orbits(obj, orbit_map)).sum::<usize>())
        .unwrap_or(0)
}

fn shortest_route(orbit_map: &HashMap<&str, Vec<&str>>, start: &str, target: &str) -> usize {
    let mut paths = VecDeque::new();
    let mut visited = HashSet::new();

    paths.push_back((start, 0));

    while let Some((obj, distance)) = paths.pop_front() {
        if obj == target {
            return distance
        }

        visited.insert(obj);

        for neighbor in orbit_map.get(obj).into_iter().flatten() {
            if !visited.contains(neighbor) {
                paths.push_back((neighbor, distance + 1))
            }
        }
    }

    panic!("No route found!")
}

pub type OrbitRelation = [String; 2];

pub fn parse_input(input: &str) -> impl Iterator<Item = OrbitRelation> + '_ {
    input.lines()
        .map(|raw_relation| {
            let mut split = raw_relation.split(')');
            [
                split.next().expect("Missing left object").to_owned(),
                split.next().expect("Missing right object").to_owned(),
            ]
        })
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part1(&input), 358_244);
    }

    #[test]
    fn p2() {
        let input: Vec<_> = parse_input(RAW_INPUT_STR).collect();

        assert_eq!(part2(&input), 517);
    }
}
