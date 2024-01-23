mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 209
impl Solution {
    pub fn min_sub_array_len(target: i32, nums: Vec<i32>) -> i32 {
        let mut pos = 0;
        let mut sum = 0;
        let mut minimum: Option<usize> = Option::None;

        for (idx, &number) in nums.iter().enumerate() {
            sum += number;

            while pos < idx && sum - nums[pos] >= target {
                sum -= nums[pos];
                pos += 1;
            }

            if sum >= target {
                minimum = Some(match minimum {
                    None => idx + 1 - pos,
                    Some(s) => s.min(idx + 1 - pos),
                });
            }

            // println!("{}: {} {} {} {:?}", idx, number, sum, pos, minimum);
        }

        minimum.unwrap_or(0) as i32
    }
}
