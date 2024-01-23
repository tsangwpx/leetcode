// mod done;

use std::ops::BitAnd;

fn main() {
    println!("Hello, world!");
}

struct Solution {}

impl Solution {
    // Problem 50
    pub fn my_pow(mut x: f64, n: i32) -> f64 {
        if x == 0.0 {
            return x;
        }
        if n == 0 {
            return 1.0;
        }

        let reciprocal = n < 0;
        let sign = if (n & 1) == 1 { x.signum() } else { 1.0 };
        let mut n = (n as i64).abs() as u32;
        let mut x = x.abs();

        let mut r = 1.0f64;

        while n > 0 {
            if (n & 1) == 1 {
                r *= x;
            }

            x = x * x;
            n = n >> 1;
        }

        r = r.copysign(sign);

        if reciprocal {
            r.recip()
        } else {
            r
        }
    }
}
