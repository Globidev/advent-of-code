fn solve(target_gifts: u32, gift_factor: u32, gift_limit: u32) -> Option<u32> {
    use std::cmp::min;

    let limit = target_gifts / gift_factor;
    let mut houses = vec![0; limit as usize];

    for elf in 1..limit {
        let house_count = min(limit / elf, gift_limit);
        for house in 1..house_count {
            let house_no = house * elf;
            houses[house_no as usize] += elf * gift_factor;
        }
        if houses[elf as usize] >= target_gifts {
            return Some(elf)
        }
    }

    None
}

pub fn p1(input: &str) -> u32 {
    let num_gifts = input.trim().parse::<u32>().unwrap();
    solve(num_gifts, 10, num_gifts / 10).unwrap_or(0)
}

pub fn p2(input: &str) -> u32 {
    let num_gifts = input.trim().parse::<u32>().unwrap();
    solve(num_gifts, 11, 50).unwrap_or(0)
}
