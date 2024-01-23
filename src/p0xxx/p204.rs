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

// Problem 204
impl Solution {
    pub fn count_primes(n: i32) -> i32 {
        let n = n as usize;
        let mut count = 0;
        let mut is_prime = vec![true; n];

        for i in 2..n {
            if !is_prime[i as usize] {
                continue;
            }

            count += 1;

            let mut m = 2;

            while i * m < n {
                is_prime[i * m] = false;
                m += 1;
            }
        }

        count
    }
}
