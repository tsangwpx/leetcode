mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 997
impl Solution {
    pub fn find_judge(n: i32, trust: Vec<Vec<i32>>) -> i32 {
        let mut src_count = vec![0i32; n as usize + 1];
        let mut dst_count = src_count.clone();

        for row in trust {
            let b = row[1];
            let a = row[0];

            src_count[a as usize] += 1;
            dst_count[b as usize] += 1;
        }

        for i in 1..=n {
            if src_count[i as usize] == 0 && dst_count[i as usize] == n - 1 {
                return i;
            }
        }

        -1
    }
}
