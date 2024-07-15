mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 442
impl Solution {
    pub fn find_duplicates(nums: Vec<i32>) -> Vec<i32> {
        let mut nums = nums;
        let mut out = vec![];

        let mut idx = 0;
        while idx < nums.len() {
            let number = nums[idx];
            if number == 0 || number == idx as i32 + 1 {
                idx += 1;
                continue;
            }
            let dst = number as usize - 1;
            if nums[dst] == number {
                out.push(number);
                nums[idx] = 0;
                idx += 1;
            } else {
                nums[idx] = nums[dst];
                nums[dst] = number;
            }
        }

        out
    }
}
