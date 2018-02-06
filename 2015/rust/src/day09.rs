extern crate nom;
use std::collections::{HashMap, HashSet};

type City = String;
type Distance = u32;

type DistanceMap = HashMap<(City, City), Distance>;
type TripMap = HashMap<City, Vec<(City, Distance)>>;
type Path = Vec<City>;

fn path_distance(map: &DistanceMap, path: &Path) -> Distance {
    let distance_between = |(c1, c2): (&City, &City)| {
        let from_to = &(c1.clone(), c2.clone());
        let to_from = &(c2.clone(), c1.clone());
        match map.get(from_to) {
            Some(d) => d,
            None    => map.get(to_from).unwrap()
        }
    };

    path.into_iter().zip(path.into_iter().skip(1))
               .map(distance_between)
               .sum()
}

fn paths_from(trips: &TripMap, path: Path, l: usize) -> Vec<Path> {
    if path.len() == l {
        vec![path]
    }
    else {
        let city = path.get(path.len() - 1).unwrap();
        let no_destinations = Vec::new();
        let possible_destinations = trips.get(city).unwrap_or(&no_destinations)
                                                   .iter();
        let unvisited_destinations = possible_destinations.filter(|&&(ref c, _)|
            !path.contains(c)
        );
        unvisited_destinations.flat_map(|&(ref to, _)| {
            let mut new_path = path.clone();
            new_path.push(to.clone());
            paths_from(trips, new_path, l)
        }).collect()
    }
}

fn all_paths(map: &DistanceMap) -> Vec<Path> {
    let mut unique_cities = HashSet::new();
    let mut trip_map = TripMap::new();

    map.iter().for_each(|(&(ref from, ref to), &d)| {
        {
        let destinations_from = trip_map.entry(to.clone()).or_insert(Vec::new());
        destinations_from.push((from.clone(), d));
        }
        let destinations_to = trip_map.entry(from.clone()).or_insert(Vec::new());
        destinations_to.push((to.clone(), d));
    });

    map.keys().for_each(|&(ref from, ref to)| {
        unique_cities.insert(from);
        unique_cities.insert(to);
    });

    let paths_from_city = |c: &&City| paths_from(
        &trip_map,
        vec![(*c).clone()],
        unique_cities.len()
    );

    unique_cities.iter().flat_map(paths_from_city).collect()
}

fn parse_map_entry(input: &str) -> ((City, City), Distance) {
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    named!(city<City>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(distance<Distance>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap()
    ));

    named!(parser<((City, City), Distance)>, do_parse!(
        from: city     >>
        tag_s!(" to ") >>
        to: city       >>
        tag_s!(" = ")  >>
        dist: distance >>
        ((from, to), dist)
    ));

    match parser(input.as_bytes()) {
        IResult::Done(_, entry) => entry,
        _                       => panic!("Wrong map format")
    }
}

pub fn p1(input: &str) -> Distance {
    let map: DistanceMap = input.trim().split('\n')
                                       .map(parse_map_entry)
                                       .collect();
    let paths = all_paths(&map);

    paths.iter().map(|p| path_distance(&map, &p)).min().unwrap_or(0)
}

pub fn p2(input: &str) -> u32 {
    let map: DistanceMap = input.trim().split('\n')
                                       .map(parse_map_entry)
                                       .collect();
    let paths = all_paths(&map);

    paths.iter().map(|p| path_distance(&map, &p)).max().unwrap_or(0)
}
