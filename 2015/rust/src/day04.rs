extern crate crypto;

use day04::crypto::md5::Md5;
use day04::crypto::digest::Digest;

fn md5(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);
    md5.result_str()
}

pub fn p1(input: &str) -> u32 {
    (0u32..).find(|i| {
        let seed = format_args!("{}{}", input.trim(), i).to_string();
        md5(&seed).starts_with("00000")
    }).unwrap()
}

pub fn p2(input: &str) -> u32 {
    (0u32..).find(|i| {
        let seed = format_args!("{}{}", input.trim(), i).to_string();
        md5(&seed).starts_with("000000")
    }).unwrap()
}
