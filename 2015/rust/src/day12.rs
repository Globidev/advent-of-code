extern crate serde_json;

use self::serde_json::{Value, Error};

fn count_numbers(data: &Value) -> i32 {
    use self::Value::*;

    match data {
        &Object(ref map) => map.values().map(count_numbers).sum(),
        &Array(ref arr)  => arr.iter().map(count_numbers).sum(),
        &Number(ref n)   => n.as_i64().unwrap() as i32,
        _                => 0
    }
}

fn count_numbers_filtered(data: &Value) -> i32 {
    use self::Value::*;

    match data {
        &Object(ref map) => {
            match map.values().any(|v| v == &String("red".to_string())) {
                true  => 0,
                false => map.values().map(count_numbers_filtered).sum()
            }
        },
        &Array(ref arr)  => arr.iter().map(count_numbers_filtered).sum(),
        &Number(ref n)   => n.as_i64().unwrap() as i32,
        _                => 0
    }
}

pub fn p1(input: &str) -> i32 {
    let data = serde_json::from_str::<Value>(input.trim()).unwrap();

    count_numbers(&data)
}

pub fn p2(input: &str) -> i32 {
    let data = serde_json::from_str::<Value>(input.trim()).unwrap();

    count_numbers_filtered(&data)
}
