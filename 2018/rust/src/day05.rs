use rayon::prelude::*;
use arrayvec::ArrayVec;

const RAW_INPUT_STR: &str = include_str!("../../inputs/day05.txt");

pub fn day05() -> (usize, usize) {
    (part1(RAW_INPUT_STR), part2(RAW_INPUT_STR))
}

pub fn part1(input: &str) -> usize {
    react(input.bytes()).len()
}

pub fn part2(input: &str) -> usize {
    let reduced = react(input.bytes());

    (b'a'..b'z' + 1)
        .into_par_iter()
        .map(|lower| {
            reduced.iter()
                .filter(move |&c| c | 32 != lower)
                .cloned()
        })
        .map(|polymer| react(polymer).len())
        .min()
        .expect("Empty polymer")
}

fn react(polymer: impl Iterator<Item = u8>) -> ArrayVec<[u8; 16384]> {
    let mut reduced = ArrayVec::<[u8; 16384]>::new();

    for byte in polymer {
        let top_is_opposite = reduced.last()
            .map(|&c| c ^ 32 == byte)
            .unwrap_or(false);

        if top_is_opposite {
            let _ = reduced.pop();
        } else {
            reduced.push(byte);
        }
    }

    reduced
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        assert_eq!(part1(RAW_INPUT_STR), 10978);
    }

    #[test]
    fn p2() {
        assert_eq!(part2(RAW_INPUT_STR), 4840);
    }
}
