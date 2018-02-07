extern crate nom;

use std::collections::{VecDeque, HashMap, HashSet};

type Guest = String;
type Happiness = i32;
type HappinessMap<'a> = HashMap<(&'a Guest, &'a Guest), Happiness>;
type GuestSet = HashSet<Guest>;
type Table<'a> = Vec<&'a Guest>;

fn table_happiness(table: &Table, happiness_map: &HappinessMap) -> Happiness {
    let mut happiness = 0;

    for i in 0..table.len() {
        let g1 = table.get(i).unwrap();
        let g2 = table.get((i+1) % table.len()).unwrap();
        happiness += happiness_map.get(&(g2, g1)).unwrap();
        happiness += happiness_map.get(&(g1, g2)).unwrap();
    }

    happiness
}

fn all_table_configurations(guest_set: &GuestSet) -> Vec<Table> {
    let mut tables = Vec::new();
    let mut open_set = VecDeque::new();

    if let Some(guest) = guest_set.iter().next() {
        open_set.push_back(vec![guest]);
    }

    loop {
        if let Some(guests) = open_set.pop_front() {
            match guests.len() == guest_set.len() {
                true  => tables.push(guests),
                false => {
                    let guests_left = guest_set.iter()
                                               .filter(|g| !guests.contains(g));

                    for other_guest in guests_left {
                        let mut new_table = guests.clone();
                        new_table.push(other_guest);
                        open_set.push_back(new_table);
                    }
                }
            }
        }
        else {
            break
        }
    }

    tables
}

fn parse_map_entry(input: &str) -> ((Guest, Guest), Happiness) {
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    type HappinessTransform = fn(Happiness) -> Happiness;

    named!(guest<Guest>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(happiness<Happiness>, do_parse!(
        diff: alt!(
            map!(tag_s!("gain "), |_| (|x|  x) as HappinessTransform) |
            map!(tag_s!("lose "), |_| (|x| -x) as HappinessTransform)
        ) >>
        n: map!(digit, |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap()) >>
        (diff(n))
    ));
    named!(parser<((Guest, Guest), Happiness)>, do_parse!(
        g1: guest         >>
        tag_s!(" would ") >>
        happ: happiness   >>
        tag_s!(" happiness units by sitting next to ")  >>
        g2: guest >>
        ((g1, g2), happ)
    ));

    match parser(input.as_bytes()) {
        IResult::Done(_, entry) => entry,
        _                       => panic!("Wrong map format")
    }
}

pub fn p1(input: &str) -> Happiness {
    let mut guest_set = GuestSet::new();
    let mut happiness_map = HappinessMap::new();

    let entries = input.trim().split('\n')
                              .map(parse_map_entry)
                              .collect::<Vec<_>>();

    for &((ref g1, ref g2), _) in &entries {
        guest_set.insert(g1.clone());
        guest_set.insert(g2.clone());
    }
    for &((ref g1, ref g2), h) in &entries {
        let entry = (guest_set.get(g1).unwrap(), guest_set.get(g2).unwrap());
        happiness_map.insert(entry, h);
    }

    let tables = all_table_configurations(&guest_set);

    tables.iter().map(|t| table_happiness(t, &happiness_map)).max().unwrap_or(0)
}

pub fn p2(input: &str) -> Happiness {
    let mut guest_set = GuestSet::new();
    let mut happiness_map = HappinessMap::new();

    let entries = input.trim().split('\n')
                              .map(parse_map_entry)
                              .collect::<Vec<_>>();

    for &((ref g1, ref g2), _) in &entries {
        guest_set.insert(g1.clone());
        guest_set.insert(g2.clone());
    }

    let me = "Me".to_string();
    guest_set.insert(me.clone());
    let me_entry = guest_set.get(&me).unwrap();

    for &((ref g1, ref g2), h) in &entries {
        let entry = (guest_set.get(g1).unwrap(), guest_set.get(g2).unwrap());
        happiness_map.insert(entry, h);
    }

    for guest in &guest_set {
        if guest != me_entry {
            happiness_map.insert((me_entry, guest), 0);
            happiness_map.insert((guest, me_entry), 0);
        }
    }

    let tables = all_table_configurations(&guest_set);

    tables.iter().map(|t| table_happiness(t, &happiness_map)).max().unwrap_or(0)
}
