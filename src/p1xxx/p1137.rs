mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1137
impl Solution {
    pub fn tribonacci(n: i32) -> i32 {
        let mut t0 = 0;
        let mut t1 = 1;
        let mut t2 = 1;

        match n {
            0 => t0,
            1 => t1,
            2 => t2,
            _ => {
                let mut t3;

                for _ in 3..=n {
                    t3 = t0 + t1 + t2;
                    t0 = t1;
                    t1 = t2;
                    t2 = t3;
                }

                t2
            }
        }
    }
}
