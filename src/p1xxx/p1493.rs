mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1493
impl Solution {
    pub fn longest_subarray(nums: Vec<i32>) -> i32 {
        let mut window = 0;
        let mut deleted = false;
        let mut longest = 0;
        let mut count = 0;

        for (idx, &number) in nums.iter().enumerate() {
            if number == 1 {
                count += 1;

                if deleted {
                    longest = longest.max(count);
                } else {
                    longest = longest.max(count - 1);
                }
            } else {
                while deleted && window < idx {
                    match nums[window] {
                        0 => {
                            window += 1;
                            deleted = false;
                        }
                        1 => {
                            window += 1;
                            count -= 1;
                        }
                        _ => {}
                    }
                }

                if !deleted {
                    deleted = true;
                }

                longest = longest.max(count);
            }
        }

        longest
    }

    pub fn longest_subarray2(nums: Vec<i32>) -> i32 {
        if !nums.contains(&0) {
            return nums.len() as i32 - 1;
        }

        let mut window = 0;
        let mut deleted = false;
        let mut longest = 0;
        let mut count = 0;

        for (idx, &number) in nums.iter().enumerate() {
            if number == 1 {
                count += 1;
                longest = longest.max(count);
            } else {
                while deleted && window < idx {
                    match nums[window] {
                        0 => {
                            window += 1;
                            deleted = false;
                        }
                        1 => {
                            window += 1;
                            count -= 1;
                        }
                        _ => {}
                    }
                }

                if !deleted {
                    deleted = true;
                }
            }
        }

        longest
    }
}
