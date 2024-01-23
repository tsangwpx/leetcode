mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 221
impl Solution {
    pub fn maximal_square(matrix: Vec<Vec<char>>) -> i32 {
        use std::mem::swap;

        let m = matrix.len();
        let n = matrix[0].len();

        let mut dp0 = vec![0; n];
        let mut dp1 = vec![0; n];
        let mut side_max = 0;

        for (i, row) in matrix.iter().enumerate() {
            assert!(row.len() == n);

            dp1[0] = if row[0] == '0' { 0 } else { 1 };
            // side_max = side_max.max(dp1[0]);

            for (j, &item) in row.iter().enumerate().skip(1) {
                dp1[j] = if item == '0' {
                    0
                } else {
                    1 + dp1[j - 1].min(dp0[j - 1]).min(dp0[j])
                };
                // side_max = side_max.max(dp1[j]);
            }

            side_max = side_max.max(dp1.iter().max().copied().unwrap());

            swap(&mut dp0, &mut dp1);
        }

        side_max * side_max
    }
}
