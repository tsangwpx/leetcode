mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2390
impl Solution {
    pub fn remove_stars(s: String) -> String {
        let mut result = String::new();

        for ch in s.chars() {
            if ch == '*' {
                result.pop();
            } else {
                result.push(ch);
            }
        }

        result
    }
}
