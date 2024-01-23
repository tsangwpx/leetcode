mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 119
impl Solution {
    pub fn get_row(row_index: i32) -> Vec<i32> {
        let row_index = row_index as usize;
        let mut result = vec![1i32; row_index + 1];

        if row_index <= 1 {
            return result;
        }

        let mut next = result.clone();

        for row in 1..row_index {
            for i in 1..row + 1 {
                next[i] = result[i - 1] + result[i];
            }

            std::mem::swap(&mut result, &mut next);
        }

        result
    }
}
