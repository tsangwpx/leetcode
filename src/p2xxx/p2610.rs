mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2610
impl Solution {
    pub fn find_matrix(nums: Vec<i32>) -> Vec<Vec<i32>> {
        let mut counter = [0u16; 201];
        let mut res = vec![vec![0i32]; 0];
        for number in nums.iter().copied() {
            let counter_idx = number as usize;
            let res_idx = counter[counter_idx] as usize;
            counter[counter_idx] += 1;

            if res_idx < res.len() {
                res[res_idx].push(number);
            } else {
                res.push(vec![number]);
            }
        }

        res
    }
}
