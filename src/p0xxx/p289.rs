mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 289
impl Solution {
    pub fn game_of_life(board: &mut Vec<Vec<i32>>) {
        const SHIFT: u32 = 8;

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let mut count = 0;
                let r0 = i.saturating_sub(1);
                let r1 = board.len().min(i + 2);
                let c0 = j.saturating_sub(1);
                let c1 = board[i].len().min(j + 2);

                for r in r0..r1 {
                    for c in c0..c1 {
                        if r == i && c == j {
                            continue;
                        }

                        if board[r][c] & 1 == 1 {
                            count += 1;
                        }
                    }
                }
                // println!("{} {}: {}", i, j, count);

                board[i][j] |= count << SHIFT;
            }
        }

        for i in 0..board.len() {
            for j in 0..board[i].len() {
                let count = board[i][j] >> SHIFT;

                board[i][j] = if board[i][j] & 1 == 1 {
                    match count {
                        0..=1 => 0,
                        2..=3 => 1,
                        4..=9 => 0,
                        _ => unreachable!(),
                    }
                } else {
                    match count {
                        3 => 1,
                        _ => 0,
                    }
                };
            }
        }
    }
}
