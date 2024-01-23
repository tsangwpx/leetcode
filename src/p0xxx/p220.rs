mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 220
impl Solution {
    pub fn contains_nearby_almost_duplicate(
        nums: Vec<i32>,
        index_diff: i32,
        value_diff: i32,
    ) -> bool {
        use std::collections::HashMap;

        let idx_diff = index_diff as usize;

        let mut buckets = HashMap::<i32, i32>::with_capacity(nums.len());

        #[inline]
        fn get_key(item: i32, diff: i32) -> i32 {
            match diff {
                0 => item,
                _ => item / diff,
            }
        }

        for (idx, &item) in nums.iter().enumerate() {
            let bucket_key = get_key(item, value_diff);

            // If value_diff is 0, we have only one target.
            // Otherwise, search the nearby buckets
            let bucket_range = match value_diff {
                0 => bucket_key..=bucket_key,
                _ => bucket_key - 1..=bucket_key + 1,
            };

            for key in bucket_range {
                if let Some(&idx2) = buckets.get(&key) {
                    let idx2 = idx2 as usize;
                    if (idx - idx2) <= idx_diff && (item - nums[idx2]).abs() <= value_diff {
                        return true;
                    }
                }
            }

            buckets.insert(bucket_key, idx as i32);
        }

        false
    }
}
