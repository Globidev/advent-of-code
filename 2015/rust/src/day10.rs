type Seq = String;

fn next_sequence(seq: &Seq) -> Seq {
    use std::str::FromStr;

    let mut next = Seq::new();
    let mut iter = seq.chars();
    let mut x = iter.next();

    loop {
        match x {
            None => break,
            Some(c) => {
                let mut count = 1;
                while { x = iter.next(); x == Some(c) } {
                    count += 1;
                }
                next += &count.to_string();
                next.push(c);
            }
        }
    }

    next
}

fn look_and_say(input: &str, iter_count: u32) -> Seq {
    let mut seq = input.to_string();

    for _ in 0..iter_count {
        seq = next_sequence(&seq);
    }

    seq
}

pub fn p1(input: &str) -> usize {
    look_and_say(input.trim(), 40).len()
}

pub fn p2(input: &str) -> usize {
    look_and_say(input.trim(), 50).len()
}
