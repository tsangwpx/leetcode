mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 992
impl Solution {
    pub fn subarrays_with_k_distinct(nums: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;

        let mut count = 0;

        let mut front_idx = 0;
        let mut front_freq = vec![0; nums.len() + 1];
        let mut front_len = 0;

        let mut back_idx = 0;
        let mut back_freq = vec![0; nums.len() + 1];
        let mut back_len = 0;

        for number in nums.iter().copied() {
            let number = number as usize;

            front_freq[number] += 1;
            if front_freq[number] == 1 {
                front_len += 1;
            }

            while front_len > k {
                let front = nums[front_idx] as usize;
                front_idx += 1;
                front_freq[front] -= 1;

                if front_freq[front] == 0 {
                    front_len -= 1;
                }
            }

            back_freq[number] += 1;
            if back_freq[number] == 1 {
                back_len += 1;
            }
            while back_len >= k {
                let back = nums[back_idx] as usize;
                if back_freq[back] >= 2 {
                    back_freq[back] -= 1;
                    back_idx += 1;
                } else if back_len > k {
                    // assert(back_freq[back] == 1)
                    back_freq[back] -= 1;
                    back_idx += 1;
                    back_len -= 1;
                } else {
                    // assert(back_freq[back] == 1 && back_len == k)
                    break;
                }
            }

            if front_len == k {
                count += back_idx + 1 - front_idx;
            }
        }

        count as _
    }
}
