mod leetcode_prelude;

use leetcode_prelude::*;

fn main() {
    println!("123456");

    use std::hint::black_box;

    println!("456789");
}

struct Solution {}

extern crate rand;

// Problem 980
impl Solution {
    pub fn unique_paths_iii(mut grid: Vec<Vec<i32>>) -> i32 {
        let mut start_point = None;
        let mut finish_point = None;
        let mut empty = 2;

        for (i, row) in grid.iter().enumerate() {
            for (j, &item) in row.iter().enumerate() {
                match item {
                    1 => start_point = Some((i, j)),
                    2 => finish_point = Some((i, j)),
                    0 => empty += 1,
                    _ => {}
                }
            }
        }

        let (si, sj) = start_point.unwrap();
        let (ei, ej) = finish_point.unwrap();
        grid[si][sj] = 0;
        grid[ei][ej] = 0;

        fn dfs(
            grid: &mut Vec<Vec<i32>>,
            empty: i32,
            i: usize,
            j: usize,
            ei: usize,
            ej: usize,
        ) -> i32 {
            if grid[i][j] < 0 {
                return 0;
            }

            let empty = empty - 1;
            let hit = i == ei && j == ej;
            if empty == 0 || hit {
                return if empty == 0 && hit { 1 } else { 0 };
            }

            let m = grid.len();
            let n = grid[0].len();
            let neighbors = [
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_add(1), j),
            ];
            let mut res = 0;

            grid[i][j] = -2;

            for (ni, nj) in neighbors {
                if ni >= m || nj >= n {
                    continue;
                }

                res += dfs(grid, empty, ni, nj, ei, ej);
            }
            grid[i][j] = 0;

            res
        }

        dfs(&mut grid, empty, si, sj, ei, ej)
    }
}
