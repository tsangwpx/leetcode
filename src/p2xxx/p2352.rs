mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 2352
impl Solution {
    pub fn equal_pairs(grid: Vec<Vec<i32>>) -> i32 {
        use std::collections::HashMap;

        let n = grid.len();
        assert!(n >= 1);
        grid.iter().for_each(|row| assert!(row.len() == n));

        let mut counter = HashMap::new();

        for row in grid.iter() {
            *counter.entry(row).or_insert(0) += 1
        }

        let mut count = 0;

        for col in 0..n {
            let x = (0..n).map(|row| grid[row][col]).collect::<Vec<_>>();
            if let Some(&count2) = counter.get(&x) {
                count += count2;
            }
        }

        count
    }

    pub fn equal_pairs2(grid: Vec<Vec<i32>>) -> i32 {
        let n = grid.len();
        assert!(n >= 1);
        grid.iter().for_each(|row| assert!(row.len() == n));

        let mut count = 0;

        for i in 0..n {
            for j in 0..n {
                if (0..n).all(|k| grid[i][k] == grid[k][j]) {
                    count += 1;
                }
            }
        }

        count
    }
}
