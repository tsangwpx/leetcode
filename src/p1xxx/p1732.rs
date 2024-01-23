mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1732
impl Solution {
    pub fn largest_altitude(gain: Vec<i32>) -> i32 {
        let mut max = 0;
        let mut altitude = 0;

        for change in gain {
            altitude += change;
            max = max.max(altitude);
        }

        max
    }
}
