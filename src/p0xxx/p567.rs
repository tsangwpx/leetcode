mod leetcode_prelude;

use std::ffi::FromVecWithNulError;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 567
impl Solution {
    pub fn check_inclusion(s1: String, mut s2: String) -> bool {
        let mut balance = [0; 26];

        s1.bytes().for_each(|ch| {
            let idx = (ch - b'a') as usize;
            assert!(idx < 26);
            balance[idx] += 1;
        });

        s2.bytes().take(s1.len()).for_each(|ch| {
            let idx = (ch - b'a') as usize;
            assert!(idx < 26);
            balance[idx] -= 1;
        });

        // println!("{:?}", balance);

        if balance.iter().all(|&s| s == 0) {
            return true;
        }

        for (i, ch) in s2.bytes().enumerate().skip(s1.len()) {
            let idx = (ch - b'a') as usize;
            let idx2 = (s2.bytes().nth(i - s1.len()).unwrap() - b'a') as usize;

            balance[idx] -= 1;
            balance[idx2] += 1;

            // println!("{:?}", balance);
            if balance[idx] == 0 && balance[idx2] == 0 && balance.iter().all(|&s| s == 0) {
                return true;
            }
        }

        false
    }
}
