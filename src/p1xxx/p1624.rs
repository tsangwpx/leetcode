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

// Problem 1624
impl Solution {
    pub fn max_length_between_equal_characters(s: String) -> i32 {
        let mut start = [usize::MAX; 26];
        let mut max_len = -1i32;

        for (stop, ch) in s.bytes().enumerate() {
            let idx = (ch - b'a') as usize;
            assert!(idx < 26);

            if start[idx] == usize::MAX {
                start[idx] = stop;
            } else {
                max_len = max_len.max((stop - start[idx] - 1) as i32);
            }
        }

        max_len
    }
}
