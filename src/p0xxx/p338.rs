mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 338
impl Solution {
    pub fn count_bits(n: i32) -> Vec<i32> {
        let n = n as usize;
        let mut res = vec![0i32; n + 1];

        res[0] = 0;

        for shift in 0..20u32 {
            let start = 2usize.pow(shift);
            let stop = 2usize.pow(shift + 1).min(res.len());
            let mask = start - 1;
            let range = start..stop;

            for i in range {
                res[i] = res[i & mask] + 1;
            }
        }

        res
    }
}
