mod leetcode_prelude;

use std::iter::FromIterator;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 3015
impl Solution {
    pub fn count_of_pairs(n: i32, x: i32, y: i32) -> Vec<i32> {
        let n = n as usize;
        let x = x as usize - 1;
        let y = y as usize - 1;

        assert!(x < n && y < n);

        let mut dp = vec![vec![u8::MAX; n]; n];

        for i in 0..n {
            dp[i][i] = 0;
        }
        for i in 0..(n - 1) {
            dp[i][i + 1] = 1;
            dp[i + 1][i] = 1;
        }
        if x != y {
            dp[x][y] = 1;
            dp[y][x] = 1;
        }

        for k in 0..n {
            for i in 0..n {
                for j in 0..n {
                    if dp[i][k] == u8::MAX || dp[k][j] == u8::MAX {
                        continue;
                    }

                    if dp[i][j] > dp[i][k] + dp[k][j] {
                        dp[i][j] = dp[i][k] + dp[k][j];
                    }
                }
            }
        }

        let mut res = vec![0; n];
        for i in 0..n {
            for j in 0..n {
                if dp[i][j] < u8::MAX && dp[i][j] > 0 {
                    res[dp[i][j] as usize - 1] += 1;
                }
            }
        }
        res
    }
}
