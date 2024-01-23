mod leetcode_prelude;

use std::ops::{Add, Sub};

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1704
impl Solution {
    pub fn halves_are_alike(s: String) -> bool {
        fn is_vowel(ch: u8) -> bool {
            matches!(
                ch,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        }

        let half = s.len() / 2;
        let left = s.bytes().take(half).filter(|&ch| is_vowel(ch)).count();
        let right = s.bytes().skip(half).filter(|&ch| is_vowel(ch)).count();
        left == right
    }

    pub fn halves_are_alike2(s: String) -> bool {
        const VOWELS: [bool; 256] = {
            let mut table = [false; 256];
            table[b'a' as usize] = true;
            table[b'e' as usize] = true;
            table[b'i' as usize] = true;
            table[b'o' as usize] = true;
            table[b'u' as usize] = true;
            table[b'A' as usize] = true;
            table[b'E' as usize] = true;
            table[b'I' as usize] = true;
            table[b'O' as usize] = true;
            table[b'U' as usize] = true;
            table
        };

        fn is_vowel(ch: u8) -> bool {
            matches!(
                ch,
                b'a' | b'e' | b'i' | b'o' | b'u' | b'A' | b'E' | b'I' | b'O' | b'U'
            )
        }

        let half = s.len() / 2;
        let left = s
            .bytes()
            .take(half)
            .filter(|&ch| VOWELS[ch as usize])
            .count();
        let right = s
            .bytes()
            .skip(half)
            .filter(|&ch| VOWELS[ch as usize])
            .count();

        left == right
    }
}
