mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 36
impl Solution {
    pub fn is_valid_sudoku(board: Vec<Vec<char>>) -> bool {
        let mut row_constraints = [0u16; 9];
        let mut col_constraints = [0u16; 9];
        let mut blk_constraints = [0u16; 9];

        assert!(board.len() == 9);
        for i in 0..9 {
            assert!(board[i].len() == 9);

            for j in 0..9 {
                match board[i][j] {
                    digit @ '1'..='9' => {
                        let shift = digit as u8 - b'1';
                        let bit = 1u16 << shift;
                        let blk_idx = i / 3 * 3 + j / 3;
                        let used =
                            row_constraints[i] | col_constraints[j] | blk_constraints[blk_idx];

                        if (used & bit) != 0 {
                            return false;
                        }

                        row_constraints[i] |= bit;
                        col_constraints[j] |= bit;
                        blk_constraints[blk_idx] |= bit;
                    }
                    _ => {}
                }
            }
        }

        true
    }
}
