mod leetcode_prelude;

use leetcode_prelude::*;

pub fn main() {}

// hello world !!!!

extern crate rand;

// Problem 1011
impl Solution {
    pub fn ship_within_days(weights: Vec<i32>, days: i32) -> i32 {
        let mut max_weight = weights[0];
        let mut pfxsums = weights;
        for i in 1..pfxsums.len() {
            max_weight = max_weight.max(pfxsums[i]);
            pfxsums[i] = pfxsums[i - 1] + pfxsums[i];
        }

        let total_weight = pfxsums.last().copied().unwrap();

        let mut left = max_weight;
        let mut right = total_weight;

        // println!("Pfxsum: {:?}", pfxsums);

        #[inline]
        fn can_ship(pfxsums: &[i32], days: i32, capacity: i32) -> bool {
            let mut days_used = 0;
            let mut offset = 0;
            // println!("Capacity: {}", capacity);

            while days_used < days && offset < pfxsums.len() {
                let base = if offset > 0 { pfxsums[offset - 1] } else { 0 };
                let step = pfxsums[offset..].partition_point(|&s| s <= base + capacity);
                // println!(
                //     "Base {} + {} offset {} step {} ",
                //     base,
                //     base + capacity,
                //     offset,
                //     step
                // );
                offset += step;
                days_used += 1;
            }

            // println!("Days: {}", days_used);

            days_used <= days && offset == pfxsums.len()
        }

        while left < right {
            // println!("{} {}", left, right);
            let capacity = left + (right - left) / 2;

            if can_ship(&pfxsums, days, capacity) {
                right = capacity;
            } else {
                left = capacity + 1;
            }
        }

        left
    }
}
