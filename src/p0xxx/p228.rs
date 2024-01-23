mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 228
impl Solution {
    pub fn summary_ranges(nums: Vec<i32>) -> Vec<String> {
        let mut res = vec![];
        let mut i = 0;

        while i < nums.len() {
            let a = nums[i] as i64;
            i += 1;

            let mut b = a + 1;

            while i < nums.len() && b == nums[i] as i64 {
                b += 1;
                i += 1;
            }

            b -= 1;

            if a == b {
                res.push(format!("{}", a));
            } else {
                res.push(format!("{}->{}", a, b));
            }
        }

        res
    }
}
