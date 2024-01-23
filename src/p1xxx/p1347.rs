mod leetcode_prelude;

use std::ops::{Add, Sub};

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1347
impl Solution {
    pub fn min_steps(s: String, t: String) -> i32 {
        let mut freq = [0i32; 26];
        for ch in s.bytes() {
            let idx = (ch - b'a') as usize;
            *unsafe { freq.get_unchecked_mut(idx) } += 1;
        }

        for ch in t.bytes() {
            let idx = (ch - b'a') as usize;
            *unsafe { freq.get_unchecked_mut(idx) } -= 1;
        }

        freq.iter().cloned().filter(|&s| s > 0).sum()
    }
}
