mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 2597
impl Solution {
    pub fn beautiful_subsets(nums: Vec<i32>, k: i32) -> i32 {
        // since the nums length <= 20
        // we may use brute force to compute the result.
        // the power set is of size 2 ** 20

        let mut res = 0;
        let mut counter = [0i8; 1001];

        fn recurse(nums: &[i32], k: i32, counter: &mut [i8; 1001], res: &mut i32) {
            for (idx, &item) in nums.iter().enumerate() {
                let left = (item - k) as usize;
                let right = (item + k) as usize;

                if left < counter.len() && counter[left] > 0 {
                    continue;
                }
                if right < counter.len() && counter[right] > 0 {
                    continue;
                }

                // Put item to form a new set
                *res += 1;

                counter[item as usize] += 1;

                // Passing the remaining items recursively
                recurse(&nums[idx + 1..], k, counter, res);

                counter[item as usize] -= 1;
            }
        }

        recurse(&nums, k, &mut counter, &mut res);

        res
    }
}
