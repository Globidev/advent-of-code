const RAW_INPUT: &[u8] = include_bytes!("../../inputs/day02.txt");

const WORD_LEN: usize = 26;
const WORD_COUNT: usize = 250;

pub fn parse_input(input: &[u8]) -> impl Iterator<Item = &[u8]> {
    input.split(|&c| c == b'\n')
}

pub fn day02() -> (usize, String) {
    let input: Vec<_> = parse_input(RAW_INPUT).collect();

    (part1(&input), part2_set(&input))
}

pub fn part1(input: &[&[u8]]) -> usize {
    let (pairs, triples) = input.iter()
        .map(|s| box_property(s))
        .fold((0, 0), |(mut pairs, mut triples), prop| {
            if prop.has_pair { pairs += 1 }
            if prop.has_triple { triples += 1 }
            (pairs, triples)
        });

    pairs * triples
}

pub fn part2_pairs(boxes: &[&[u8]]) -> String {
    use itertools::Itertools;

    boxes.iter()
        .tuple_combinations()
        .find_map(|(box1, box2)| {
            let mut diff_indexes = box1.iter().zip(box2.iter())
                .enumerate()
                .filter(|(_, (c1, c2))| c1 != c2)
                .map(|(idx, _)| idx);

            match (diff_indexes.next(), diff_indexes.next()) {
                (Some(idx), None) => /* Only one diff */ {
                    let mut common = box1.to_vec();
                    common.remove(idx);
                    unsafe { Some(String::from_utf8_unchecked(common)) }
                }
                _ => None
            }
        })
        .expect("No solution")
}

pub fn part2_set(boxes: &[&[u8]]) -> String {
    let mut word_parts = hashbrown::HashSet::with_capacity(WORD_COUNT);

    for split_idx in 0..WORD_LEN - 1 {
        for id in boxes {
            let (left, right) = (&id[..split_idx], &id[split_idx + 1..]);
            if !word_parts.insert((left, right)) {
                let left_str = std::str::from_utf8(left).unwrap();
                let right_str = std::str::from_utf8(right).unwrap();
                return format!("{}{}", left_str, right_str)
            }
        }
        word_parts.clear();
    }

    panic!("No solution")
}

fn box_property(r#box: &[u8]) -> BoxProperty {
    let mut counter = [0; WORD_LEN];

    for byte in r#box.iter() {
        counter[(byte - b'a') as usize] += 1;
    }

    counter.iter()
        .fold(BoxProperty::default(), |mut prop, &count| {
            if count == 2 { prop.has_pair = true }
            if count == 3 { prop.has_triple = true }
            prop
        })
}

#[derive(Default)]
struct BoxProperty {
    has_pair: bool,
    has_triple: bool
}
