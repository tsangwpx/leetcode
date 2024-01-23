mod leetcode_prelude;

use std::iter::FromIterator;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 3014
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut remaining = word.len() as i32;
        let mut cost = 0;
        let mut hit = 1;

        while remaining > 0 {
            let used = if remaining >= 8 { 8 } else { remaining };
            remaining -= used;

            cost += hit * used;
            hit += 1;
        }

        cost
    }
}
