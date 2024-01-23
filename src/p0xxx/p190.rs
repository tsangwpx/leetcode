mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 190
impl Solution {
    pub fn reverse_bits(x: u32) -> u32 {
        x.reverse_bits()
    }
}
