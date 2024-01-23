mod leetcode_prelude;

use std::collections::HashMap;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 166
impl Solution {
    pub fn fraction_to_decimal(numerator: i32, denominator: i32) -> String {
        if numerator == 0 {
            return "0".to_owned();
        }

        let mut result = String::new();

        // Normalize both numerator and denominator to positive
        if numerator.signum() * denominator.signum() < 0 {
            result.push('-');
        }

        // Use u64 to hold the values
        let mut numerator = numerator.unsigned_abs() as u64;
        let denominator = denominator.unsigned_abs() as u64;
        assert!(denominator != 0);

        // write the part before decimal separator
        if numerator >= denominator {
            result.push_str(&(numerator / denominator).to_string());
            numerator = numerator % denominator;
        } else {
            result.push('0');
        }

        if numerator == 0 {
            return result;
        }

        // write decimal separator
        result.push('.');

        let mut table = std::collections::HashMap::<u64, usize>::new();

        loop {
            // In each iteration, the remainder is multiplied by 10
            numerator *= 10;

            // println!("{}/{}: {}", numerator, denominator, result);

            let idx = *table.entry(numerator).or_insert(result.len());

            if idx < result.len() {
                // this remainder was seen before, a recurring sequence is found.
                result.reserve_exact(2);
                result.push(')');
                result.insert(idx, '(');
                return result;
            }

            let (q, r) = (numerator / denominator, numerator % denominator);
            assert!(q <= 9);

            result.push((q as u8 + b'0') as char);
            numerator = r;

            if numerator == 0 {
                return result;
            }
        }
    }
}
