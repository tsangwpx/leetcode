mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2870
// Similar to problem 2244
impl Solution {
    pub fn min_operations(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;
        let mut counter = HashMap::new();

        for number in nums.iter().copied() {
            counter
                .entry(number)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut operations = 0;

        for &count in counter.values() {
            if count <= 1 {
                return -1;
            }

            match count % 3 {
                0 => operations += count / 3,
                1 => operations += (count / 3) + 1,
                2 => operations += (count / 3) + 1,
                _ => unreachable!(),
            }
        }

        operations
    }
}
