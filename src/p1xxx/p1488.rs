mod leetcode_prelude;

use std::vec;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 1488
impl Solution {
    pub fn avoid_flood(rains: Vec<i32>) -> Vec<i32> {
        use std::collections::hash_map::Entry::{Occupied, Vacant};
        use std::collections::BTreeSet;
        use std::collections::HashMap;

        let mut dries = BTreeSet::new();
        let mut full = HashMap::<i32, usize>::new();
        let mut ans = vec![-1; rains.len()];

        for (day, lake) in rains.iter().copied().enumerate() {
            if lake == 0 {
                dries.insert(day);
                continue;
            }

            match full.entry(lake) {
                Vacant(entry) => {
                    entry.insert(day);
                }
                Occupied(mut entry) => {
                    let past = *entry.get();
                    if let Some(&dry_day) = dries.range(past..).next() {
                        *entry.get_mut() = day;
                        ans[dry_day] = lake;
                        dries.remove(&dry_day);
                    } else {
                        return vec![];
                    }
                }
            }
        }

        for day in dries.iter().copied() {
            ans[day] = 1;
        }

        ans
    }
}
