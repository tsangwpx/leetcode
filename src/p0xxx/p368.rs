mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

extern crate rand;

// Problem 368
impl Solution {
    pub fn largest_divisible_subset(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        #[derive(Clone, Copy, Debug, Default)]
        struct Entry {
            prev: u16,
            size: u16,
        }

        let mut dp = vec![Entry::default(); nums.len()];
        let mut best = 0;

        for (i, &number) in nums.iter().enumerate() {
            for j in (0..i).rev() {
                let friend = nums[j];

                if number % friend == 0 && dp[j].size > dp[i].size {
                    dp[i].size = dp[j].size;
                    dp[i].prev = j as u16;
                }
            }

            dp[i].size += 1;

            if dp[i].size > dp[best].size {
                best = i;
            }
        }

        let mut res = vec![];

        loop {
            res.push(nums[best]);

            if dp[best].size == 1 {
                break;
            }

            best = dp[best].prev as usize;
        }

        res
    }

    pub fn largest_divisible_subset2(mut nums: Vec<i32>) -> Vec<i32> {
        nums.sort_unstable();

        let mut dp = vec![vec![]; nums.len()];
        let mut best = 0;

        for (i, &number) in nums.iter().enumerate() {
            for j in (0..i).rev() {
                let (front, back) = dp.split_at_mut(i);
                let number_set = back.first_mut().unwrap();

                let friend = nums[j];
                let friend_set = &front[j];

                if number % friend == 0 && friend_set.len() > number_set.len() {
                    number_set.clone_from(friend_set);
                }
            }

            dp[i].push(number);

            if dp[i].len() > dp[best].len() {
                best = i;
            }
        }

        dp.swap_remove(best)
    }
}
