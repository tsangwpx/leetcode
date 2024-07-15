mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 451
impl Solution {
    pub fn frequency_sort(string: String) -> String {
        let mut counter = [0i32; 128];
        let mut string = string.into_bytes();

        for &ch in string.iter() {
            counter[ch as usize] += 1;
        }

        let mut chars = [0; 128];
        for i in 0..128 {
            chars[i] = i as u8;
        }

        chars.sort_unstable_by_key(|&ch| -counter[ch as usize]);

        string.truncate(0);

        for ch in chars.iter().copied() {
            let count = counter[ch as usize] as usize;
            if count == 0 {
                break;
            }
            string.resize(string.len() + count, ch);
        }

        unsafe { String::from_utf8_unchecked(string) }
    }
}
