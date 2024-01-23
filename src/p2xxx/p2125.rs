mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2125
impl Solution {
    pub fn number_of_beams(bank: Vec<String>) -> i32 {
        let mut number_of_beams = 0;

        let mut prev_devices = 0;
        for row in bank.iter() {
            let devices = row.bytes().filter(|&s| s == b'1').count() as i32;

            if devices > 0 {
                number_of_beams += prev_devices * devices;
                prev_devices = devices;
            }
        }

        number_of_beams
    }
}
