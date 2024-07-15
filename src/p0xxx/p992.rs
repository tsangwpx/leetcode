mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 992
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::hash_map::Entry::{Occupied, Vacant};
        use std::collections::HashMap;

        let k = k as usize;

        let mut count = 0;
        let mut left = 0;
        let mut left_counter = HashMap::with_capacity(k as usize);
        let mut right = 0;
        let mut right_counter = HashMap::with_capacity(k as usize);

        for number in nums.iter().copied() {
            left_counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            while left_counter.len() > k {
                let Occupied(mut entry) = left_counter.entry(nums[left]) else {
                    unreachable!("Missing entry {}", nums[left]);
                };

                left += 1;

                let count = entry.get_mut();
                if *count >= 2 {
                    *count -= 1;
                } else {
                    entry.remove();
                }
            }

            right_counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);

            loop {
                let right_len = right_counter.len();

                if right_len < k {
                    break;
                }

                let Occupied(mut entry) = right_counter.entry(nums[right]) else {
                    unreachable!("Missing entry {}", nums[right]);
                };

                let count = entry.get_mut();
                if *count >= 2 {
                    *count -= 1;
                    right += 1;
                } else if right_len > k {
                    entry.remove();
                    right += 1;
                } else {
                    // right_len == k && *count == 1
                    // Dont remove
                    break;
                }
            }

            if left_counter.len() == k {
                count += right + 1 - left;
            }
        }

        count as _
    }
}
