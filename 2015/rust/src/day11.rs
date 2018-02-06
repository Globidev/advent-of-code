use std::collections::HashSet;

type Password = String;

fn valid_password(pwd: &Password) -> bool {
    let has_straight = pwd.chars().zip(pwd.chars().skip(1))
                                  .zip(pwd.chars().skip(2))
                                  .any(|((c1, c2), c3)|
                                    c1 as u8 == c2 as u8 - 1 &&
                                    c2 as u8 == c3 as u8 - 1
                                  );

    let forbidden_letters = "iol";
    let has_forbidden_letters = pwd.chars().any(|c| forbidden_letters.contains(c));

    let mut pairs = HashSet::new();
    let mut iter = pwd.chars().peekable();

    loop {
        match iter.next() {
            None    => break,
            Some(c) => {
                if iter.peek() == Some(&c) {
                    pairs.insert(c);
                    iter.next();
                }
            }
        }
    }

    let has_two_overlapping_pairs = pairs.len() >= 2;

    has_straight && !has_forbidden_letters && has_two_overlapping_pairs
}

fn next_password(pwd: &Password) -> Password {
    let mut change = true;

    let reved: Password = pwd.chars().rev().map(|c| {
        match change {
            false => c,
            true  => match c {
                'z' => 'a',
                c   => { change = false; ((c as u8) + 1) as char }
            }
        }
    }).collect();

    reved.chars().rev().collect()
}

pub fn p1(input: &str) -> Password {
    let mut password = input.trim().to_string();

    while !valid_password(&password) {
        password = next_password(&password);
    }

    password
}

pub fn p2(input: &str) -> Password {
    let mut password = next_password(&p1(input));

    while !valid_password(&password) {
        password = next_password(&password);
    }

    password
}
