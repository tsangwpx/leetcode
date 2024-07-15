mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3075
impl Solution {
    pub fn maximum_happiness_sum(happiness: Vec<i32>, k: i32) -> i64 {
        use std::collections::BinaryHeap;

        let mut pq = BinaryHeap::from(happiness);

        let mut res = 0;

        for time in 0..k {
            let Some(top) = pq.pop() else {
                break;
            };

            if top <= time {
                break;
            }

            res += (top - time) as i64;
        }

        res
    }
}
