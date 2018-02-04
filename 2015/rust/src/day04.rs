extern crate crypto;

use day04::crypto::md5::Md5;
use day04::crypto::digest::Digest;
use std::thread;
use std::sync::Arc;
use std::sync::atomic::{AtomicU32, Ordering};

fn md5(input: &str) -> String {
    let mut md5 = Md5::new();
    md5.input_str(input);
    md5.result_str()
}

fn md5_step_compute(key: String, start: u32, step: u32, pat: &str, spinlock: Arc<AtomicU32>) {
    let mut iter = start;
    loop {
        let stored = spinlock.load(Ordering::SeqCst);
        if stored == 0 || iter < stored {
            let seed = format_args!("{}{}", key, iter).to_string();
            if md5(&seed).starts_with(pat) {
                spinlock.store(iter, Ordering::SeqCst);
            }
            iter += step;
        }
        else {
            break
        }
    }
}

fn mine_block_threaded(th_count: u32, key: &str, pattern: &'static str) -> u32 {
    let spinlock = Arc::new(AtomicU32::new(0));

    let threads: Vec<_> = (0u32..th_count).map(|i| {
        let key_clone = key.to_string().clone();
        let spinlock_clone = spinlock.clone();
        thread::spawn(move || {
            md5_step_compute(key_clone, i, th_count, pattern, spinlock_clone)
        })
    }).collect();

    for thread in threads {
        match thread.join() {
            Err(e) => panic!(e),
            _ => ()
        }
    }

    spinlock.load(Ordering::SeqCst)
}

pub fn p1(input: &str) -> u32 {
    let key = input.trim();
    mine_block_threaded(4, key, "00000")
}

pub fn p2(input: &str) -> u32 {
    let key = input.trim();
    mine_block_threaded(4, key, "000000")
}
