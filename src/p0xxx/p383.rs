mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 383
impl Solution {
    pub fn can_construct(ransom_note: String, magazine: String) -> bool {
        if ransom_note.len() > magazine.len() {
            return false;
        }

        let mut counter = [0i32; 256];
        ransom_note.bytes().for_each(|s| counter[s as usize] += 1);

        magazine.bytes().for_each(|s| counter[s as usize] -= 1);

        counter.iter().all(|&s| s <= 0)
    }
}
