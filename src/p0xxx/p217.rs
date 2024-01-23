mod leetcode_prelude;

use std::ffi::FromVecWithNulError;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 217
impl Solution {
    pub fn contains_duplicate(nums: Vec<i32>) -> bool {
        let len = nums.len();
        let set = nums.into_iter().collect::<std::collections::HashSet<i32>>();

        len > set.len()
    }
}
