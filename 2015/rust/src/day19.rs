extern crate nom;

use std::collections::HashSet;

type Transform = (String, String);

fn parse_transform(raw_transform: &str) -> Transform {
    use self::nom::*;

    use std::str::from_utf8;

    named!(s<String>, map!(
        take_while1!(is_alphabetic),
        |s| from_utf8(s).unwrap().to_string()
    ));

    named!(transform<Transform>, do_parse!(
        before: s >>
        tag_s!(" => ") >>
        after: s >>
        ((before, after))
    ));

    match transform(raw_transform.as_bytes()) {
        IResult::Done(_, transform) => transform,
        _                           => panic!("Wrong transform format")
    }
}

fn possible_transforms(input: String, transforms: &Vec<Transform>) -> HashSet<String> {
    let mut all = HashSet::new();

    for &(ref before, ref after) in transforms.iter() {
        for (idx, _) in input.match_indices(before) {
            let mut cpy = input.to_string();
            for _ in 0..before.len() {
                cpy.remove(idx);
            }
            cpy.insert_str(idx, after);
            all.insert(cpy);
        }
    }

    all
}

pub fn p1(input: &str) -> usize {
    let puzzle_parts = input.trim().split("\n\n")
                                   .collect::<Vec<_>>();
    let (raw_transforms, initial_molecule) = (puzzle_parts[0], puzzle_parts[1]);

    let transforms = raw_transforms.split('\n')
                                   .map(parse_transform)
                                   .collect::<Vec<_>>();

    possible_transforms(initial_molecule.to_string(), &transforms).len()
}

pub fn p2(input: &str) -> usize {
    let puzzle_parts = input.trim().split("\n\n")
                                   .collect::<Vec<_>>();
    let target_molecule = puzzle_parts[1];

    let count_str = |x| target_molecule.matches(x).collect::<Vec<_>>().len();

    target_molecule.chars().filter(|c| c.is_uppercase()).count()
        - count_str("Rn") - count_str("Ar") - 2 * count_str("Y") - 1
}
