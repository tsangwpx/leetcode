mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 1249
impl Solution {
    pub fn min_remove_to_make_valid2(s: String) -> String {
        let mut res = s.into_bytes();

        let mut count = 0;

        for i in 0..res.len() {
            let ch = res[i];
            match ch {
                b'(' => count += 1,
                b')' => {
                    if count >= 1 {
                        count -= 1;
                    } else {
                        res[i] = b'\0';
                    }
                }
                _ => {}
            }
        }

        let mut count = 0;
        for i in (0..res.len()).rev() {
            let ch = res[i];
            match ch {
                b')' => count += 1,
                b'(' => {
                    if count >= 1 {
                        count -= 1;
                    } else {
                        res[i] = b'\0';
                    }
                }
                _ => {}
            }
        }

        res.retain(|&ch| ch != b'\0');

        unsafe { String::from_utf8_unchecked(res) }
    }
}
