mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 1535
impl Solution {
    pub fn get_winner(arr: Vec<i32>, k: i32) -> i32 {
        assert!(arr.len() >= 2);
        let mut winner = arr[0];
        let mut win_count = 1;
        if arr[1] > winner {
            winner = arr[1];
        }

        if win_count >= k {
            return winner;
        }

        for challenger in arr[2..].iter().copied() {
            if winner >= challenger {
                win_count += 1;

                if win_count >= k {
                    break;
                }
            } else {
                winner = challenger;
                win_count = 1;
            }
        }

        winner
    }
}
