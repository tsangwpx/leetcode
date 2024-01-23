mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1239
impl Solution {
    pub fn max_length(arr: Vec<String>) -> i32 {
        use std::collections::HashSet;

        let mut items = HashSet::new();
        let mut work = vec![];
        let mut max_length = 0;

        'outer: for string in arr {
            let mut bits = 0u32;
            for ch in string.bytes() {
                let bit = 1 << (ch - b'a');
                if (bits & bit) == bit {
                    continue 'outer;
                }
                bits |= bit;
            }

            if !items.contains(&bits) {
                max_length = max_length.max(bits.count_ones());
                work.push(bits);

                for other in items.iter() {
                    if (bits & other) == 0 {
                        let new_bits = bits | other;
                        max_length = max_length.max(new_bits.count_ones());
                        work.push(new_bits);
                    }
                }

                items.extend(work.drain(..));
            }
        }

        max_length as i32
    }
}
