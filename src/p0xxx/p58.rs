mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 58

impl Solution {
    pub fn length_of_last_word(s: String) -> i32 {
        let mut count = 0;
        for ch in s.bytes().rev() {
            if ch == b' ' {
                if count > 0 {
                    break;
                }
            } else {
                count += 1
            }
        }

        count
    }

    pub fn length_of_last_word2(s: String) -> i32 {
        s.rsplit_terminator(|ch| ch == ' ')
            .filter_map(|s| {
                if s.len() > 0 {
                    Some(s.len() as i32)
                } else {
                    None
                }
            })
            .next()
            .unwrap()
    }
}
