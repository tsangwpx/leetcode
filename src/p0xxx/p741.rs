mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 741
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        let total_steps = 2 * n - 1;

        let mut dp0 = vec![vec![0i16; n]; n];
        let mut dp1 = dp0.clone();

        // image two farmers starting cherry picking from (0, 0)
        // we need (2 * n - 1) steps to reach (n - 1, n - 1)
        for step in 0..total_steps {
            // We parameterize farmer coordinate with the step count.
            // the second coordinate is given by substracting the first from the step count.
            // i.e. i = step - j
            // Some coordinate is invalid.
            // The dp array represent where are the farmers and the maximum number of pickups.
            // Double counting may occur when i == p, which need special handling.

            for i in 0..n {
                for p in 0..n {
                    let j = step.wrapping_sub(i);
                    let q = step.wrapping_sub(p);

                    if j >= n || q >= n || i + j != step || p + q != step {
                        // out of bounds
                        dp1[i][p] = -1;
                        continue;
                    }

                    if grid[i][j] < 0 || grid[p][q] < 0 {
                        // invalid location
                        dp1[i][p] = -1;
                        continue;
                    }

                    // (i, j - 1) -> (i, j); (p, q - 1) -> (p, q)
                    dp1[i][p] = dp0[i][p];

                    if i > 0 {
                        // (i - 1, j) -> (i, j); (p, q - 1) -> (p, q)
                        dp1[i][p] = dp1[i][p].max(dp0[i - 1][p]);
                    }

                    if p > 0 {
                        // (i, j - 1) -> (i, j); (p - 1, q) -> (p, q)
                        dp1[i][p] = dp1[i][p].max(dp0[i][p - 1]);
                    }

                    if i > 0 && p > 0 {
                        // (i - 1, j) -> (i, j); (p - 1, q) -> (p, q)
                        dp1[i][p] = dp1[i][p].max(dp0[i - 1][p - 1]);
                    }

                    // If dp1[i][p] is nonnegative,
                    // both farmers are in valid positions and cherry will be collected.

                    if dp1[i][p] >= 0 {
                        dp1[i][p] += grid[i][j] as i16;

                        if i != p {
                            dp1[i][p] += grid[p][q] as i16;
                        }
                    }
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0[n - 1][n - 1].max(0) as i32
    }
}
