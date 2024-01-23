mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1004
impl Solution {
    pub fn longest_ones(nums: Vec<i32>, k: i32) -> i32 {
        assert!(k >= 0);
        assert!(nums.len() >= 1);

        let mut flipped = 0;
        let mut maximum = 0;
        let mut one_count = 0;
        let mut window = 0;

        for (idx, &number) in nums.iter().enumerate() {
            if number == 1 {
                one_count += 1;
            } else {
                while flipped == k && window < idx {
                    match nums[window] {
                        0 => {
                            if flipped > 0 {
                                // unflip this one
                                flipped -= 1;
                                one_count -= 1;
                            }
                        }
                        1 => {
                            one_count -= 1;
                        }
                        _ => unreachable!(),
                    }

                    window += 1;
                }

                if k > flipped {
                    flipped += 1;
                    one_count += 1;
                }
            }

            maximum = maximum.max(one_count);
        }

        maximum
    }
}
