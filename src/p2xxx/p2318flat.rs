mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2318
impl Solution {
    pub fn distinct_sequences(n: i32) -> i32 {
        if n == 1 {
            return 6;
        }

        const MOD: i64 = 10i64.pow(9) + 7;

        const Y: bool = true;
        const N: bool = false;
        const GUARD: [[bool; 6]; 6] = [
            [N, Y, Y, Y, Y, Y],
            [Y, N, Y, N, Y, N],
            [Y, Y, N, Y, Y, N],
            [Y, N, Y, N, Y, N],
            [Y, Y, Y, Y, N, Y],
            [Y, N, N, N, Y, N],
        ];

        let n = n as usize;

        let mut dp0: &mut [i64; 36] = &mut [
            0, 1, 1, 1, 1, 1, //
            1, 0, 1, 0, 1, 0, //
            1, 1, 0, 1, 1, 0, //
            1, 0, 1, 0, 1, 0, //
            1, 1, 1, 1, 0, 1, //
            1, 0, 0, 0, 1, 0, //
        ];
        let mut dp1 = &mut dp0.clone();

        for _ in 3..=n {
            for j in 0..6 {
                for k in 0..6 {
                    dp1[j * 6 + k] = 0;

                    if dp0[j * 6 + k] != 0 {
                        for i in 0..6 {
                            dp1[j * 6 + k] += dp0[i * 6 + j];
                        }

                        dp1[j * 6 + k] -= dp0[k * 6 + j];
                        dp1[j * 6 + k] %= MOD;
                    }
                }
            }

            // println!("{:?}", dp1);

            std::mem::swap(&mut dp0, &mut dp1);
        }

        let sum = dp0.iter().copied().sum::<i64>();
        (sum % MOD) as i32
    }
}
