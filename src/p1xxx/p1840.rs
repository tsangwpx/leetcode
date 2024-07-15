mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 1840
impl Solution {
    pub fn max_building(n: i32, restrictions: Vec<Vec<i32>>) -> i32 {
        use std::fmt::Debug;

        if restrictions.is_empty() {
            return n - 1;
        }

        #[derive(Clone, Copy)]
        struct Entry {
            id: i32,
            limit: i32,
        }

        impl Debug for Entry {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "({}, {})", self.id, self.limit)
            }
        }

        let mut entries = restrictions
            .into_iter()
            .map(|entry| Entry {
                id: entry[0],
                limit: entry[1],
            })
            .collect::<Vec<Entry>>();
        entries.push(Entry { id: 1, limit: 0 });
        entries.sort_unstable_by_key(|entry| entry.id);

        if entries.last().unwrap().id < n {
            entries.push(Entry {
                id: n,
                limit: n - 1,
            })
        }

        for i in 1..entries.len() {
            let Entry {
                id: prev_id,
                limit: prev_limit,
            } = entries[i - 1];

            entries[i].limit = entries[i].limit.min(prev_limit + entries[i].id - prev_id);
        }

        let mut max_height = entries.last().unwrap().limit;

        for i in (0..entries.len() - 1).rev() {
            let Entry {
                id: next_id,
                limit: next_limit,
            } = entries[i + 1];

            let this_id = entries[i].id;
            let this_limit = entries[i].limit.min(next_limit + next_id - this_id);
            entries[i].limit = this_limit;

            let btw_limit = (next_id - this_id) as i64 + this_limit as i64 + next_limit as i64;
            let btw_limit = btw_limit / 2;
            max_height = max_height.max(this_limit).max(btw_limit as i32);
        }

        max_height
    }
}
