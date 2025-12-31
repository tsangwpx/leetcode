// Problem 3446
impl Solution {
    pub fn sort_matrix(mut grid: Vec<Vec<i32>>) -> Vec<Vec<i32>> {
        use std::cmp::Reverse;

        let n = grid.len();
        let mut tmp = Vec::with_capacity(n + n - 1);
        let mid = n - 1;

        for line in 0..2 * n - 1 {
            let (u, v) = if line <= mid {
                (mid - line, 0)
            } else {
                (0, line - mid)
            };

            let (mut i, mut j) = (u, v);
            tmp.clear();

            while i < n && j < n {
                tmp.push(grid[i][j]);
                i += 1;
                j += 1;
            }

            if line <= mid {
                tmp.sort_unstable_by_key(|&s| Reverse(s));
            } else {
                tmp.sort_unstable();
            }

            let (mut i, mut j) = (u, v);
            for item in tmp.iter().copied() {
                grid[i][j] = item;
                i += 1;
                j += 1;
            }
        }

        grid
    }
}
