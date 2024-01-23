mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 162

impl Solution {
    pub fn find_peak_element(nums: Vec<i32>) -> i32 {
        // by analyzing the location of mid in binary search
        if nums.len() <= 1 {
            return 0;
        }

        // first or last element may be peaks.
        if nums[0] > nums[1] {
            return 0;
        } else if nums[nums.len() - 1] > nums[nums.len() - 2] {
            return nums.len() as i32 - 1;
        }

        let mut left = 1;
        let mut right = nums.len() - 2;

        while left <= right {
            let mid = (left + right) / 2;

            if nums[mid] > nums[mid + 1] && nums[mid] > nums[mid - 1] {
                // mid is local peak
                return mid as i32;
            } else if nums[mid] < nums[mid + 1] {
                // peak is on right
                left = mid + 1;
            } else {
                right = mid - 1;
            }
        }

        // why unreachable?
        unreachable!()
    }
}
