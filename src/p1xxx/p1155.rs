mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1155
impl Solution {
    pub fn num_rolls_to_target(n: i32, k: i32, target: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let n = n as usize;
        let k = k as usize;
        let target = target as usize;

        if target < n || target > n * k {
            // we have N dice with k faces. there is no way to get a sum < n or > nk.
            return 0;
        }

        assert!(n >= 1 && k >= 1 && target >= 1);

        let len = target + 1;
        let mut dp0 = vec![0i64; len];
        let mut dp1 = vec![0i64; len];

        // init
        dp0[0] = 1;

        for dice in 0..n {
            // this is the (dice+1)-th dice
            // the min and max sum is determiend by (dice+1)*1 and (dice + 1)*k
            // they are bounded by the len of the dp array
            let start = dice + 1; // the minimum sum
            let stop = ((dice + 1) * k + 1).min(len); // maximum sum + 1

            let mut sum = 0i64; // running sum of k-window

            // zero dirt position
            dp1[start.saturating_sub(2)] = 0;
            dp1[start - 1] = 0;

            // build up the sum for the first k counts
            for i in start..(start + k).min(stop) {
                // (s + MOD) % MOD make sure the number is positive
                sum = (sum + dp0[i - 1] + MOD) % MOD;
                dp1[i] = sum;
            }

            // maintain the k-sized window sum
            for i in start + k..stop {
                // (start - 1) is the first item in the previous loop
                // so (start + k - k - 1) is the first in this loop
                sum = (sum - dp0[i - k - 1] + dp0[i - 1] + MOD) % MOD;
                dp1[i] = sum;
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0[target] as i32
    }
}
