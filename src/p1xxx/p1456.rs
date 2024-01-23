mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1456
impl Solution {
    pub fn max_vowels(s: String, k: i32) -> i32 {
        let k = k as usize;
        assert!(k >= 1);
        assert!(s.len() >= 1);

        let vowels = [b'a', b'e', b'i', b'o', b'u'];

        let mut count = 0;

        for number in s.bytes().take(k) {
            if vowels.contains(&number) {
                count += 1;
            }
        }

        let mut maximum = count;

        for (i, number) in s.bytes().enumerate().skip(k) {
            if vowels.contains(&s.bytes().nth(i - k as usize).unwrap()) {
                count -= 1;
            }
            if vowels.contains(&number) {
                count += 1;
            }
            maximum = maximum.max(count);
        }

        maximum
    }
}
