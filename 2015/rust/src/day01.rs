fn symbol_delta(sym: char) -> i32 {
    match sym {
        '(' => 1,
        ')' => -1,
        _ => 0
    }
}

pub fn p1(input: &str) -> i32 {
    input.chars()
         .map(symbol_delta)
         .sum()
}

pub fn p2(input: &str) -> u32 {
    let mut current = 0;
    match input.chars().zip(0..).find(|&(c, _)| {
        current += symbol_delta(c);
        current < 0
    }) {
        Some((_, i)) => i + 1,
        None         => panic!("Should not happen")
    }
}
