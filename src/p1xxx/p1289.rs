mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 1289
impl Solution {
    pub fn min_falling_path_sum(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        if grid.len() == 1 {
            return grid[0][0];
        }

        let mut dp0 = [(0i32, 0usize); 2];
        let mut pq = BinaryHeap::<(i32, usize)>::new();

        for i in 0..dp0.len() {
            dp0[i].1 = i;
        }

        for row in grid.iter() {
            pq.clear();
            pq.push((i32::MAX, 0));
            pq.push((i32::MAX, 1));
            pq.push((i32::MAX, 2));

            for (idx, item) in row.iter().copied().enumerate() {
                let base = if dp0[0].1 == idx { dp0[1].0 } else { dp0[0].0 };
                if base == i32::MAX {
                    continue;
                }
                let tuple = (base + item, idx);
                *pq.peek_mut().unwrap() = tuple;
            }

            // println!("{:?} {:?}", dp0, pq);

            while pq.len() > dp0.len() {
                pq.pop();
            }

            for i in (0..dp0.len()).rev() {
                if let Some((value, idx)) = pq.pop() {
                    dp0[i] = (value, idx);
                } else {
                    dp0[i] = (i32::MAX, i);
                }
            }
        }

        dp0[0].0
    }
}
