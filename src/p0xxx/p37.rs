mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 37
impl Solution {
    pub fn solve_sudoku(board: &mut Vec<Vec<char>>) {
        let mut row_used = [0u16; 9];
        let mut col_used = [0u16; 9];
        let mut blk_used = [0u16; 9];

        #[inline]
        fn blk_index(i: usize, j: usize) -> usize {
            let large_row = i / 3;
            let large_col = j / 3;
            large_row * 3 + large_col
        }

        #[inline]
        fn set_cell(
            row_used: &mut [u16; 9],
            col_used: &mut [u16; 9],
            blk_used: &mut [u16; 9],
            i: usize,
            j: usize,
            shift: u8,
        ) {
            let mask = 1u16 << shift;
            row_used[i] |= mask;
            col_used[j] |= mask;
            blk_used[blk_index(i, j)] |= mask;
        }

        #[inline]
        fn clear_cell(
            row_used: &mut [u16; 9],
            col_used: &mut [u16; 9],
            blk_used: &mut [u16; 9],
            i: usize,
            j: usize,
            shift: u8,
        ) {
            let mask = 1u16 << shift;
            row_used[i] &= !mask;
            col_used[j] &= !mask;
            blk_used[blk_index(i, j)] &= !mask;
        }

        let mut blanks = vec![0u16; 0];

        for i in 0..9 {
            for j in 0..9 {
                match board[i][j] {
                    digit @ '1'..='9' => {
                        let shift = digit as u8 - b'1';
                        set_cell(&mut row_used, &mut col_used, &mut blk_used, i, j, shift);
                    }
                    '.' => {
                        blanks.push((i << 4 | j) as u16);
                    }
                    _ => unreachable!(),
                }
            }
        }

        let mut progress = 0;

        while progress < blanks.len() {
            let id = blanks[progress];
            let i = (id >> 4) as usize;
            let j = (id & 0xF) as usize;
            // println!("{}: {} {} = {}", progress, i, j, board[i][j]);

            let used = row_used[i] | col_used[j] | blk_used[blk_index(i, j)];

            let mut shift = match board[i][j] {
                digit @ '1'..='9' => {
                    let shift = digit as u8 - b'1';
                    clear_cell(&mut row_used, &mut col_used, &mut blk_used, i, j, shift);
                    shift + 1
                }
                _ => 0,
            };

            loop {
                if used & (1 << shift) == 0 || shift >= 9 {
                    break;
                }
                shift += 1;
            }

            if shift >= 9 {
                if progress == 0 {
                    panic!("Oops! the board is not valid!");
                }

                // reset board value to empty
                board[i][j] = '.';
                progress -= 1;
            } else {
                set_cell(&mut row_used, &mut col_used, &mut blk_used, i, j, shift);
                board[i][j] = (shift + b'1') as char;
                progress += 1;
            }
        }
    }
}
