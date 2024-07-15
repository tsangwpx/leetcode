mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 1002
impl Solution {
    pub fn common_chars(words: Vec<String>) -> Vec<String> {
        let mut dp = [i8::MAX; 26];

        for word in words.iter() {
            let mut counter = [0; 26];
            for ch in word.bytes() {
                let idx = (ch - b'a') as usize;

                if idx < counter.len() {
                    counter[(ch - b'a') as usize] += 1;
                }
            }

            for i in 0..dp.len() {
                dp[i] = dp[i].min(counter[i]);
            }
        }

        let mut res = vec![];

        for (idx, count) in dp.iter().copied().enumerate() {
            if count == 0 {
                continue;
            }

            let ch = (idx as u8 + b'a') as char;
            let letter = ch.to_string();
            res.extend(std::iter::repeat(letter).take(count as usize));
        }

        res
    }
}
