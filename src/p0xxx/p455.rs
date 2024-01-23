mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 455
impl Solution {
    pub fn find_content_children(mut children: Vec<i32>, mut cookies: Vec<i32>) -> i32 {
        children.sort_unstable();
        cookies.sort_unstable();

        let mut content = 0;

        let mut i = 0;
        let mut j = 0;

        while i < children.len() && j < cookies.len() {
            let greed = children[i];
            let size = cookies[j];
            if greed <= size {
                i += 1;
                j += 1;
                content += 1;
            } else {
                // let me eat the cookie!
                j += 1;
            }
        }

        content
    }
}
