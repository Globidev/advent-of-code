extern crate nom;

use std::collections::HashMap;

#[derive(Debug)]
struct Property {
    name: String,
    value: u32,
}

#[derive(Debug)]
struct Aunt {
    id: u32,
    properties: [Property; 3],
}

fn parse_aunt(raw_aunt: &str) -> Aunt {
    use self::nom::*;

    use std::str::FromStr;
    use std::str::from_utf8;

    named!(number<u32>, map!(
        digit,
        |d| FromStr::from_str(from_utf8(d).unwrap()).unwrap()
    ));
    named!(name<String>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));
    named!(property<Property>, do_parse!(
        name: name    >>
        tag_s!(": ")  >>
        value: number >>
        (Property { name: name, value: value })
    ));

    named!(aunt<Aunt>, do_parse!(
        tag_s!("Sue ") >>
        id: number     >>
        tag_s!(": ")   >>
        p1: property   >>
        tag_s!(", ")   >>
        p2: property   >>
        tag_s!(", ")   >>
        p3: property   >>
        (Aunt { id: id, properties: [p1, p2, p3] })
    ));

    match aunt(raw_aunt.as_bytes()) {
        IResult::Done(_, aunt) => aunt,
        _                      => panic!("Wrong aunt format")
    }
}

fn matching_aunt_p1(aunt: &Aunt) -> bool {
    let properties_to_match: HashMap<String, u32> = [
        ("children",    3),
        ("cats",        7),
        ("samoyeds",    2),
        ("pomeranians", 3),
        ("akitas",      0),
        ("vizslas",     0),
        ("goldfish",    5),
        ("trees",       3),
        ("cars",        2),
        ("perfumes",    1),
    ].iter().map(|&(n, v)| (n.to_string(), v)).collect();

    aunt.properties.iter().all(|p| {
        match properties_to_match.get(&p.name) {
            Some(v) => *v == p.value,
            None    => false
        }
    })
}

fn matching_aunt_p2(aunt: &Aunt) -> bool {
    type PropertyValidator = fn(u32) -> bool;

    let properties_to_match: HashMap<String, PropertyValidator> = [
        ("children",    (|x| x == 3) as PropertyValidator),
        ("cats",        (|x| x >  7) as PropertyValidator),
        ("samoyeds",    (|x| x == 2) as PropertyValidator),
        ("pomeranians", (|x| x <  3) as PropertyValidator),
        ("akitas",      (|x| x == 0) as PropertyValidator),
        ("vizslas",     (|x| x == 0) as PropertyValidator),
        ("goldfish",    (|x| x <  5) as PropertyValidator),
        ("trees",       (|x| x >  3) as PropertyValidator),
        ("cars",        (|x| x == 2) as PropertyValidator),
        ("perfumes",    (|x| x == 1) as PropertyValidator),
    ].iter().map(|&(n, v)| (n.to_string(), v)).collect();

    aunt.properties.iter().all(|p| {
        match properties_to_match.get(&p.name) {
            Some(pred) => pred(p.value),
            None       => false
        }
    })
}

pub fn p1(input: &str) -> u32 {
    input.trim().split('\n')
                .map(parse_aunt)
                .find(matching_aunt_p1)
                .map_or(0, |aunt| aunt.id)
}

pub fn p2(input: &str) -> u32 {
    input.trim().split('\n')
                .map(parse_aunt)
                .find(matching_aunt_p2)
                .map_or(0, |aunt| aunt.id)
}
