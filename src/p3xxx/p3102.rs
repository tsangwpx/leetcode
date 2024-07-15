mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 3102
impl Solution {
    pub fn minimum_distance(points: Vec<Vec<i32>>) -> i32 {
        fn find_max(points: &[Vec<i32>], removed: Option<usize>) -> (i32, usize, usize) {
            let mut min_sum = i32::MAX;
            let mut min_sum_idx = 0;
            let mut max_sum = i32::MIN;
            let mut max_sum_idx = 0;
            let mut min_diff = i32::MAX;
            let mut min_diff_idx = 0;
            let mut max_diff = i32::MIN;
            let mut max_diff_idx = 0;

            for idx in 0..points.len() {
                if removed.map(|s| s == idx).unwrap_or(false) {
                    continue;
                }

                let y = points[idx][1];
                let x = points[idx][0];
                let sum = x + y;

                if sum > max_sum {
                    max_sum = sum;
                    max_sum_idx = idx;
                }

                if sum < min_sum {
                    min_sum = sum;
                    min_sum_idx = idx;
                }

                let diff = x - y;
                if diff > max_diff {
                    max_diff = diff;
                    max_diff_idx = idx;
                }
                if diff < min_diff {
                    min_diff = diff;
                    min_diff_idx = idx;
                }
            }

            let sum_dist = max_sum - min_sum;
            let diff_dist = max_diff - min_diff;

            if sum_dist >= diff_dist {
                (sum_dist, min_sum_idx, max_sum_idx)
            } else {
                (diff_dist, min_diff_idx, max_diff_idx)
            }
        }

        let (_, u, v) = find_max(&points, None);
        let (d1, _, _) = find_max(&points, Some(u));
        let (d2, _, _) = find_max(&points, Some(v));

        d1.min(d2)
    }
}
