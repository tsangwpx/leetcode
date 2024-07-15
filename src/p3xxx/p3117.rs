mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3117
impl Solution {
    pub fn minimum_value_sum2(nums: Vec<i32>, and_values: Vec<i32>) -> i32 {
        /*
         * HashMap is slow.
         * Observe the pattern of the hash map
         * Given an and_idx, there are at most two bits.
         * We may convert the hash map into array-like things, but it is more complex
         * due to duplicated code.
         */

        use std::collections::HashMap;

        let mut dp0 = HashMap::<(i32, usize), i32>::new();
        let mut dp1 = dp0.clone();

        dp0.insert((i32::MAX, 0), 0);

        for number in nums.iter().copied() {
            for ((bits, and_idx), value) in dp0.drain() {
                if and_idx >= and_values.len() {
                    continue;
                }

                let bits = bits & number;

                if bits >= and_values[and_idx] {
                    dp1.entry((bits, and_idx))
                        .and_modify(|sum| *sum = value.min(*sum))
                        .or_insert(value);
                }

                if bits == and_values[and_idx] {
                    let and_idx = and_idx + 1;
                    let value = value + number;
                    dp1.entry((i32::MAX, and_idx))
                        .and_modify(|sum| *sum = value.min(*sum))
                        .or_insert(value);
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
            println!("{:?}", dp0);
        }

        dp0.get(&(i32::MAX, and_values.len()))
            .copied()
            .unwrap_or(-1)
    }
}
