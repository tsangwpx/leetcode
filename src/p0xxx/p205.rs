mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 205
impl Solution {
    pub fn is_isomorphic(s: String, t: String) -> bool {
        assert!(s.len() == t.len());

        let mut mapping = [0u8; 256];
        let mut used = [false; 256];

        for (a, b) in s.bytes().zip(t.bytes()) {
            let c = mapping[a as usize];

            if b == c {
                continue;
            }

            if c != 0 || used[b as usize] {
                return false;
            }

            mapping[a as usize] = b;
            used[b as usize] = true;
        }

        true
    }
}
