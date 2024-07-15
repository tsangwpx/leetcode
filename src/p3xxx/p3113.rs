mod leetcode_prelude;

use std::ops::Bound;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3113
impl Solution {
    pub fn number_of_subarrays(nums: Vec<i32>) -> i64 {
        // println!("Hello");
        use std::cmp::Ordering;
        use std::collections::HashMap;
        use std::collections::VecDeque;

        // monotonic index stack of decreasing values
        let mut stack = vec![];

        let mut count = 0;
        let mut counter = HashMap::<i32, VecDeque<usize>>::new();

        for (idx, number) in nums.iter().copied().enumerate() {
            while let Some(pos) = stack.last().copied() {
                let last = nums[pos];

                if number > last {
                    stack.pop();
                } else {
                    break;
                }
            }

            let friends = counter.entry(number).or_default();

            if let Some(limit) = stack.last().copied() {
                let last = nums[limit];

                if last > number {
                    while let Some(prev) = friends.front().copied() {
                        if prev < limit {
                            friends.pop_front();
                        } else {
                            break;
                        }
                    }
                }
            }

            friends.push_back(idx);
            count += friends.len();

            if let Some(pos) = stack.last_mut() {
                let top = nums[*pos];
                if top == number {
                    *pos = idx;
                } else {
                    stack.push(idx);
                }
            } else {
                stack.push(idx);
            }

            // println!("{}: {:?}: {:?}", idx, number, count);
            // println!("{:?}: {:?}", stack, friends);
        }

        count as _
    }
}
