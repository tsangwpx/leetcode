mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1318
impl Solution {
    pub fn min_flips(a: i32, b: i32, c: i32) -> i32 {
        // count the different bits in the union
        let mut count = ((a | b) ^ c).count_ones();

        // Two flips are needed if a bit is one in both a and b
        // This adds the count of second flips.
        count += (a & b & ((a | b) ^ c)).count_ones();

        count as i32
    }
}
