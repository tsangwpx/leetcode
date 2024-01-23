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

// Problem 171
impl Solution {
    pub fn title_to_number(column_title: String) -> i32 {
        assert!(column_title.len() >= 1);
        let mut colnum = i32::from(column_title.bytes().last().unwrap() - b'A' + 1);

        for (idx, ch) in column_title.bytes().rev().enumerate().skip(1) {
            colnum += (ch - b'A' + 1) as i32 * 26i32.pow(idx as u32);
        }

        colnum
    }
}
