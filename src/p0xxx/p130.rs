mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 130
impl Solution {
    pub fn solve(board: &mut Vec<Vec<char>>) {
        assert!(board.len() >= 1);

        let m = board.len();
        let n = board[0].len();

        fn replace(board: &mut Vec<Vec<char>>, i: usize, j: usize, old: char, new: char) -> bool {
            // dfs + replace
            // return true if the search did reach boundaries

            if board[i][j] == old {
                board[i][j] = new;
            } else {
                return false;
            }

            let mut open = false;
            open |= i == 0 || i + 1 == board.len();
            open |= j == 0 || j + 1 == board[0].len();

            if i > 0 {
                open |= replace(board, i - 1, j, old, new);
            }

            if i + 1 < board.len() {
                open |= replace(board, i + 1, j, old, new);
            }

            if j > 0 {
                open |= replace(board, i, j - 1, old, new);
            }

            if j + 1 < board[0].len() {
                open |= replace(board, i, j + 1, old, new);
            }

            open
        }

        for i in 0..m {
            for j in 0..n {
                let ch = board[i][j];

                if ch != 'O' {
                    continue;
                }

                let open = replace(board, i, j, 'O', 'P');
                if open {
                    replace(board, i, j, 'P', 'U');
                } else {
                    replace(board, i, j, 'P', 'X');
                }
            }
        }

        for i in 0..m {
            for j in 0..n {
                let ch = board[i][j];

                if ch != 'U' {
                    continue;
                }

                replace(board, i, j, 'U', 'O');
            }
        }
    }
}
