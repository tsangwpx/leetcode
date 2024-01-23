mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 909
impl Solution {
    pub fn snakes_and_ladders(board: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = board.len();

        let flatten = {
            let n = board.len();
            let mut flatten = Vec::with_capacity(n * n);

            board.into_iter().rev().enumerate().for_each(|(i, row)| {
                if i % 2 == 0 {
                    flatten.extend_from_slice(&row);
                } else {
                    flatten.extend(row.iter().rev());
                    let p = flatten.len();
                    // flatten[p..].reverse();
                }
            });

            flatten
        };

        let dest = flatten.len();

        let mut best_steps = vec![i16::MAX; dest];
        let mut heap = BinaryHeap::new();
        heap.reserve(n * 2);
        heap.push((Reverse(0), 0));

        while let Some((Reverse(steps), index)) = heap.pop() {
            let curr = index + 1;
            if curr == dest {
                return steps;
            }

            // skip if we dont arrive here earlier
            if steps as i16 >= best_steps[index] {
                continue;
            }
            best_steps[index] = steps as i16;

            let start = index + 1;
            let stop = dest.min(start + 6);

            for mut next_index in start..stop {
                let snake_or_ladder = flatten[next_index];

                if snake_or_ladder >= 1 {
                    next_index = snake_or_ladder as usize - 1;
                }

                let new_steps = steps + 1;
                if new_steps < best_steps[next_index] as i32 {
                    heap.push((Reverse(steps + 1), next_index));
                }
            }
        }

        -1
    }

    pub fn snakes_and_ladders2(board: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        #[inline]
        fn compute_row_col(label: usize, n: usize) -> (usize, usize) {
            assert!(label >= 1);

            let label = label as usize - 1;
            let row = label / n;
            let col = if row & 1 == 0 {
                label % n
            } else {
                n - 1 - label % n
            };
            let row = n - 1 - row;

            (row, col)
        }

        #[inline]
        fn compute_label(row: usize, col: usize, n: usize) -> usize {
            // flip upside-down
            let row = n - 1 - row;
            let mut label = row * n;
            if row & 1 == 0 {
                label += col + 1;
            } else {
                label += n - 1 - col + 1;
            }

            label
        }

        let n = board.len();
        let dest = n * n;
        let mut best_steps = vec![i16::MAX; dest];
        let mut heap = BinaryHeap::new();
        heap.reserve(n * 2);
        heap.push((Reverse(0), n - 1, 0));

        while let Some((Reverse(steps), i, j)) = heap.pop() {
            // println!(
            //     "{}: {:?} {} {:?}",
            //     steps,
            //     (i, j),
            //     compute_label(i, j, n),
            //     compute_row_col(compute_label(i, j, n), n)
            // );

            let curr = compute_label(i, j, n);
            if curr == dest {
                return steps;
            }

            // skip if we dont arrive here earlier
            if steps as i16 >= best_steps[curr - 1] {
                continue;
            }
            best_steps[curr - 1] = steps as i16;

            let start = curr + 1;
            let stop = n.pow(2).min(curr + 6);

            for next in start..=stop {
                let (row, col) = compute_row_col(next, n);
                let snake_or_ladder = board[row][col];

                if snake_or_ladder == -1 {
                    heap.push((Reverse(steps + 1), row, col));
                } else {
                    let (row, col) = compute_row_col(snake_or_ladder as usize, n);
                    heap.push((Reverse(steps + 1), row, col));
                }
            }
        }

        -1
    }
}
