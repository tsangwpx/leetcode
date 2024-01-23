mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 219
impl Solution {
    pub fn contains_nearby_duplicate(nums: Vec<i32>, k: i32) -> bool {
        use std::collections::HashSet;

        if k == 0 {
            return false;
        }

        assert!(k >= 0);
        // let k = k as usize + 1;
        let window = k as usize + 1;

        let mut seen = HashSet::<i32>::new();

        for &number in nums.iter().take(window) {
            if !seen.insert(number) {
                return true;
            }
        }

        for (i, &number) in nums.iter().enumerate().skip(window) {
            seen.remove(&nums[i - window]);
            if !seen.insert(number) {
                return true;
            }
        }

        false
    }
}
