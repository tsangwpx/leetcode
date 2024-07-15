mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1824
impl Solution {
    pub fn min_side_jumps(obstacles: Vec<i32>) -> i32 {
        let mut dp0 = &mut [1, 0, 1];
        let mut dp1 = &mut [0; 3];
        const BLOCKED: i32 = 10i32.pow(9);

        let len = obstacles.len();
        for pos in 1..len {
            match obstacles[pos] {
                0 => {
                    dp1[0] = dp0[0].min(dp0[1] + 1).min(dp0[2] + 1);
                    dp1[1] = dp0[1].min(dp0[0] + 1).min(dp0[2] + 1);
                    dp1[2] = dp0[2].min(dp0[0] + 1).min(dp0[1] + 1);
                }
                1 => {
                    dp1[0] = BLOCKED;
                    dp1[1] = dp0[1].min(dp0[2] + 1);
                    dp1[2] = dp0[2].min(dp0[1] + 1);
                }
                2 => {
                    dp1[0] = dp0[0].min(dp0[2] + 1);
                    dp1[1] = BLOCKED;
                    dp1[2] = dp0[2].min(dp0[0] + 1);
                }
                3 => {
                    dp1[0] = dp0[0].min(dp0[1] + 1);
                    dp1[1] = dp0[1].min(dp0[0] + 1);
                    dp1[2] = BLOCKED;
                }
                _ => unreachable!(),
            }

            // println!("{:?}", dp1);
            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0[0].min(dp0[1]).min(dp0[2])
    }
}
