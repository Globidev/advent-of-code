fn memory_size(s: &str) -> usize {
    let mut total = 0usize;
    let mut iter = s.chars();

    loop {
        match iter.next() {
            None => break,
            Some(c) => match c {
                '\\' => {
                    if iter.next() == Some('x') {
                        iter.next();
                        iter.next();
                    }
                },
                _ => ()
            }
        };
        total += 1
    }

    total - 2
}

fn encoded_size(s: &str) -> usize {
    let mut total = 0usize;
    let mut iter = s.chars();

    loop {
        match iter.next() {
            None => break,
            Some(c) => match c {
                '\\' => total += 1,
                '"'  => total += 1,
                _   => ()
            }
        };
        total += 1
    }

    total + 2
}

pub fn p1(input: &str) -> usize {
    input.trim().split('\n').map(|s| s.len() - memory_size(s))
                            .sum()
}

pub fn p2(input: &str) -> usize {
    input.trim().split('\n').map(|s| encoded_size(s) - s.len())
                            .sum()
}
