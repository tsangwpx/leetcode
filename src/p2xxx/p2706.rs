mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2706
impl Solution {
    pub fn buy_choco(prices: Vec<i32>, money: i32) -> i32 {
        let mut small = i32::MAX;
        let mut smaller = i32::MAX;

        for price in prices {
            if price < smaller {
                small = smaller;
                smaller = price;
            } else if price < small {
                small = price;
            }
        }

        if money >= small + smaller {
            money - small - smaller
        } else {
            money
        }
    }
}
