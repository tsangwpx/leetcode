mod leetcode_prelude;

use core::num;
use std::process::id;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 713
impl Solution {
    pub fn num_subarray_product_less_than_k(nums: Vec<i32>, k: i32) -> i32 {
        if k == 0 {
            return 0;
        }

        let mut product = 1;
        let mut count = 0;
        let mut start = 0;

        for (pos, number) in nums.iter().copied().enumerate() {
            product *= number;

            while product >= k && start <= pos {
                product /= nums[start];
                start += 1;
            }

            if false {
                println!(
                    "{}: {} {}: {} {}",
                    pos,
                    product,
                    start,
                    count,
                    pos + 1 - start
                );
            }

            if product < k {
                count += pos + 1 - start;
            }
        }

        count as _
    }
}
