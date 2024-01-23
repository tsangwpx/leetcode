mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 728
impl Solution {
    pub fn self_dividing_numbers(left: i32, right: i32) -> Vec<i32> {
        let mut res = vec![];

        'outer: for n in left..=right {
            let mut remaining = n;

            while remaining != 0 {
                let d = remaining % 10;
                remaining = remaining / 10;

                if d == 0 {
                    continue 'outer;
                }

                if n % d != 0 {
                    continue 'outer;
                }
            }

            res.push(n);
        }

        res
    }
}
