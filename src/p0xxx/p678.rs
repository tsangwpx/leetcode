mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 678
impl Solution {
    pub fn check_valid_string(s: String) -> bool {
        let mut upper = 0;
        let mut lower = 0;
        for ch in s.bytes() {
            match ch {
                b'(' => {
                    upper += 1;
                    lower += 1;
                }
                b')' => {
                    if upper == 0 {
                        return false;
                    }
                    upper -= 1;
                    lower = 0.max(lower - 1);
                }
                b'*' => {
                    upper += 1;
                    lower = 0.max(lower - 1);
                }
                _ => unreachable!(),
            }
        }

        lower == 0
    }
}
