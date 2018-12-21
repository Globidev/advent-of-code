const INPUT: usize = 920831;
const INPUT_AS_STR: &str = "920831";

pub fn day14() -> (String, usize) {
    (part1(INPUT), part2(INPUT_AS_STR))
}

const MEMOIZED_STATE: [u8; 20] = [3, 7, 1, 0, 1, 0, 1, 2, 4, 5, 1, 5, 8, 9, 1, 6, 7, 7, 9, 2];
const MEMOIZED_INIT: (usize, usize) = (8, 4);

pub fn part1(recipe_count: usize) -> String {
    let mut recipes = vec![1; recipe_count + 16];
    let mut count = MEMOIZED_STATE.len();
    recipes[0..count].copy_from_slice(&MEMOIZED_STATE);
    let (mut elf1, mut elf2) = MEMOIZED_INIT;

    while count < recipe_count + 10 {
        let (move1, move2) = (recipes[elf1] + 1, recipes[elf2] + 1);
        let mut recipe_sum = recipes[elf1] + recipes[elf2];
        if recipe_sum >= 10 {
            recipe_sum -= 10;
            count += 1;
        }
        recipes[count] = recipe_sum;
        count += 1;

        elf1 = elf1 + move1 as usize;
        elf2 = elf2 + move2 as usize;
        if elf1 >= count { elf1 -= count }
        if elf2 >= count { elf2 -= count }
    }

    let ten_recipes = recipes[recipe_count..recipe_count+10]
        .into_iter()
        .map(|c| c + b'0')
        .collect();

    String::from_utf8(ten_recipes).unwrap()
}

pub fn part2(recipe_score: &str) -> usize {
    let score_len = recipe_score.len();
    let score_as_u8s: Vec<u8> = recipe_score.bytes()
        .into_iter()
        .map(|b| b - b'0')
        .collect();
    let mut needle = Needle { data: &score_as_u8s, len: score_len, pos: 0 };

    let mut recipes = vec![1; 22_000_000];
    let mut count = MEMOIZED_STATE.len();
    recipes[0..count].copy_from_slice(&MEMOIZED_STATE);
    let (mut elf1, mut elf2) = MEMOIZED_INIT;

    loop {
        let (move1, move2) = (recipes[elf1] + 1, recipes[elf2] + 1);
        let mut recipe_sum = recipes[elf1] + recipes[elf2];
        if recipe_sum >= 10 {
            if needle.find(1) { break count+1 - score_len }
            count += 1;
            recipe_sum -= 10;
        }
        recipes[count] = recipe_sum;
        if needle.find(recipe_sum) { break count+1 - score_len }
        count += 1;

        elf1 = elf1 + move1 as usize;
        elf2 = elf2 + move2 as usize;
        if elf1 >= count { elf1 -= count }
        if elf2 >= count { elf2 -= count }
    }
}

struct Needle<'a, T> {
    data: &'a [T],
    len: usize,
    pos: usize
}

impl<'a, T: Eq> Needle<'a, T> {
    fn find(&mut self, t: T) -> bool {
        if self.data[self.pos] == t {
            self.pos += 1;
            self.pos == self.len
        } else {
            self.pos = 0;
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn p1() {
        assert_eq!(part1(INPUT), "7121102535");
    }

    #[test]
    fn p2() {
        assert_eq!(part2(INPUT_AS_STR), 20236441);
    }
}
