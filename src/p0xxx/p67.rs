mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 67
impl Solution {
    pub fn add_binary(a: String, b: String) -> String {
        let mut a = a.into_bytes();
        a.reverse();

        let mut b = b.into_bytes();
        b.reverse();

        if a.len() < b.len() {
            std::mem::swap(&mut a, &mut b);
        }

        let mut carry = false;
        for (p, q) in a.iter_mut().zip(b.iter().copied()) {
            let mut count = if carry { 1 } else { 0 };
            if *p == b'1' {
                count += 1;
            }

            if q == b'1' {
                count += 1;
            }

            *p = if count & 1 == 1 { b'1' } else { b'0' };
            carry = count & 2 == 2;
        }

        for p in a.iter_mut().skip(b.len()) {
            if !carry {
                break;
            }
            if *p == b'1' {
                *p = b'0';
            } else {
                *p = b'1';
                carry = false;
                break;
            }
        }

        if carry {
            a.push(b'1');
            carry = false;
        }

        a.reverse();

        unsafe { String::from_utf8_unchecked(a) }
    }
}
