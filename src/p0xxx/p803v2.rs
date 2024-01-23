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
    pub fn hit_bricks(mut grid: Vec<Vec<i32>>, hits: Vec<Vec<i32>>) -> Vec<i32> {
        let m = grid.len();
        let n = grid[0].len();

        let mut visited = vec![vec![false; n]; m];

        for target in hits.iter() {
            let col = target[1] as usize;
            let row = target[0] as usize;
            grid[row][col] -= 1;
        }

        for col in 0..n {
            dfs(&grid, &mut visited, 0, col);
        }

        fn dfs(grid: &Vec<Vec<i32>>, visited: &mut Vec<Vec<bool>>, row: usize, col: usize) -> i32 {
            let m = grid.len();
            let n = grid[0].len();
            if row >= m || col >= n || grid[row][col] <= 0 || visited[row][col] {
                return 0;
            }
            if row > 0 {
                // make sure there is stable brick around.

                let neighbours = [
                    (row, col.wrapping_sub(1)),
                    (row, col.wrapping_add(1)),
                    (row.wrapping_sub(1), col),
                    (row.wrapping_add(1), col),
                ];
                let found = neighbours
                    .iter()
                    .any(|&(i, j)| i < m && j < n && grid[i][j] >= 1 && visited[i][j]);
                if !found {
                    return 0;
                }
            }

            visited[row][col] = true;

            let mut count = 1;
            count += dfs(grid, visited, row, col.wrapping_sub(1));
            count += dfs(grid, visited, row, col.wrapping_add(1));
            count += dfs(grid, visited, row.wrapping_sub(1), col);
            count += dfs(grid, visited, row.wrapping_add(1), col);
            count
        }

        let mut res = vec![0; hits.len()];

        // println!("{}: {:?} {:?}", -1, grid, visited);

        for (k, target) in hits.iter().enumerate().rev() {
            let col = target[1] as usize;
            let row = target[0] as usize;
            grid[row][col] += 1;
            if grid[row][col] >= 1 && !visited[row][col] {
                let count = dfs(&grid, &mut visited, row, col);

                if count >= 1 {
                    res[k] = count - 1;
                }
            }
            // println!("{}: {:?} {:?}", k, grid, visited);
        }

        res
    }
}
