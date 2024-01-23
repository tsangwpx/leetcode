mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 290
impl Solution {
    pub fn word_pattern(pattern: String, s: String) -> bool {
        use std::collections::HashSet;

        let words = s.split_ascii_whitespace().collect::<Vec<_>>();

        if pattern.len() != words.len() {
            return false;
        }

        let mut mapping = [Option::<&str>::None; 256];
        let mut seen = HashSet::<&str>::new();

        for (ch, &word) in pattern.bytes().zip(words.iter()) {
            if let Some(word2) = mapping[ch as usize] {
                if word == word2 {
                    continue;
                } else {
                    return false;
                }
            }

            if !seen.insert(word) {
                return false;
            }

            mapping[ch as usize] = Some(word);
        }

        true
    }
}
