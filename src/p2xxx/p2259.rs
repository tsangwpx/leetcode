mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2259
impl Solution {
    pub fn remove_digit(mut number: String, digit: char) -> String {
        assert!(number.len() >= 2);

        let digit = digit as u8;

        for (i, ch) in number.bytes().enumerate() {
            if ch == digit && i + 1 < number.len() && number.bytes().nth(i + 1).unwrap() > digit {
                number.remove(i);
                return number;
            }
        }

        let last = number.bytes().rposition(|s| s == digit).unwrap();

        number.remove(last);

        number
    }
}
