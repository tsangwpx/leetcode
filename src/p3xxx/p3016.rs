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

// Problem 3016
impl Solution {
    pub fn minimum_pushes(word: String) -> i32 {
        let mut counter = [0i32; 26];
        word.bytes().for_each(|ch| {
            counter[(ch - b'a') as usize] += 1;
        });

        counter.sort_unstable();

        let mut hit = 1;
        let mut mapped = 0;
        let mut cost = 0;

        for i in (0..26).rev() {
            cost += counter[i] * hit;
            mapped += 1;
            if mapped >= 8 {
                mapped = 0;
                hit += 1;
            }
        }

        cost
    }
}
