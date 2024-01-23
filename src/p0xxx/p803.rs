mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 803
impl Solution {
    pub fn hit_bricks(grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        const PARENT_NULL: u8 = 0; // parent is null, cell is either empty or unlinked.
        const PARENT_TOP: u8 = 1 << 0;
        const PARENT_LEFT: u8 = 1 << 1;
        const PARENT_BOTTOM: u8 = 1 << 2;
        const PARENT_RIGHT: u8 = 1 << 3;
        const PARENT_MASK: u8 = PARENT_TOP | PARENT_LEFT | PARENT_BOTTOM | PARENT_RIGHT;
        const PARENT_SELF: u8 = PARENT_MASK;
        const CHILD_TOP: u8 = 1 << 4;
        const CHILD_LEFT: u8 = 1 << 5;
        const CHILD_BOTTOM: u8 = 1 << 6;
        const CHILD_RIGHT: u8 = 1 << 7;
        const CHILD_MASK: u8 = CHILD_TOP | CHILD_LEFT | CHILD_BOTTOM | CHILD_RIGHT;

        let m = grid.len();
        let n = grid[0].len();

        let mut cells = vec![vec![0u8; n]; m];

        // first, we mark all top bricks are roots
        for j in 0..n {
            if grid[0][j] == 1 {
                cells[0][j] = PARENT_SELF;
            }
        }

        let mut pending = Vec::with_capacity(n * m);
        let mut staging = Vec::with_capacity(n * m);

        // then, gather remaining bricks, from bottom to top
        for i in (1..m).rev() {
            assert!(grid[i].len() == n);

            for j in (0..n).rev() {
                if grid[i][j] == 1 {
                    pending.push((i as u8, j as u8));
                }
            }
        }

        fn merge<'a>(
            cells: &mut Vec<Vec<u8>>,
            mut pending: &'a mut Vec<(u8, u8)>,
            mut unused: &'a mut Vec<(u8, u8)>,
        ) -> i32 {
            let m = cells.len();
            let n = cells[0].len();

            loop {
                // record number of pending bricks
                let count = pending.len();

                'next: while let Some((i, j)) = pending.pop() {
                    let i = i as usize;
                    let j = j as usize;

                    if i == 0 {
                        continue;
                    }

                    let stable_neighbours = [
                        (i.wrapping_sub(1), j, PARENT_TOP | CHILD_BOTTOM), // top
                        (i, j.wrapping_sub(1), PARENT_LEFT | CHILD_RIGHT), // left
                        (i, j.wrapping_add(1), PARENT_RIGHT | CHILD_LEFT), // right
                        (i.wrapping_add(1), j, PARENT_BOTTOM | CHILD_TOP), // bottom
                    ];
                    for (row, col, flags) in stable_neighbours {
                        if row >= m || col >= n {
                            // out of bounds
                            continue;
                        }

                        if cells[row][col] & PARENT_MASK == PARENT_NULL {
                            // this neighbour is not stable
                            continue;
                        } else {
                            cells[i][j] = flags & PARENT_MASK;
                            cells[row][col] |= flags & CHILD_MASK;
                            continue 'next;
                        }
                    }

                    // No stable found so far
                    unused.push((i as u8, j as u8));
                }

                if unused.len() == count {
                    // No change is made
                    break;
                }

                std::mem::swap(&mut pending, &mut unused);
            }

            let gone = unused.len() as i32;
            unused.clear();
            gone
        }

        fn erase<'a>(
            cells: &mut Vec<Vec<u8>>,
            pending: &'a mut Vec<(u8, u8)>,
            staging: &'a mut Vec<(u8, u8)>,
            i: usize,
            j: usize,
        ) {
            let parent_flags = cells[i][j] & PARENT_MASK;

            match parent_flags {
                PARENT_NULL => return,
                PARENT_SELF => {}
                PARENT_TOP => cells[i - 1][j] &= !CHILD_BOTTOM,
                PARENT_LEFT => cells[i][j - 1] &= !CHILD_RIGHT,
                PARENT_RIGHT => cells[i][j + 1] &= !CHILD_LEFT,
                PARENT_BOTTOM => cells[i + 1][j] &= !CHILD_TOP,
                _ => unreachable!("Bad parent flags: {}", parent_flags),
            }

            let mut keep = false;

            staging.push((i as u8, j as u8));

            while let Some((i, j)) = staging.pop() {
                if keep {
                    // dont keep the first item, which is erased
                    pending.push((i as u8, j as u8));
                }
                keep = true;

                let row = i as usize;
                let col = j as usize;

                let child_flags = cells[row][col] & CHILD_MASK;
                cells[row][col] = 0;

                if child_flags == 0 {
                    continue;
                }

                let table = [
                    (CHILD_BOTTOM, (i + 1, j)),
                    (CHILD_LEFT, (i, j - 1)),
                    (CHILD_RIGHT, (i, j + 1)),
                    (CHILD_TOP, (i - 1, j)),
                ];

                for (flag, next) in table {
                    if child_flags & flag == flag {
                        staging.push(next);
                    }
                }
            }
        }

        merge(&mut cells, &mut pending, &mut staging);

        let mut res = vec![0; hits.len()];

        for k in 0..hits.len() {
            let j = hits[k][1] as usize;
            let i = hits[k][0] as usize;

            erase(&mut cells, &mut pending, &mut staging, i, j);
            let gone = merge(&mut cells, &mut pending, &mut staging);
            res[k] = gone;
        }

        res
    }
}
