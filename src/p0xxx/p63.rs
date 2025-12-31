// Problem 63
impl Solution {
    pub fn unique_paths_with_obstacles(obstacle_grid: Vec<Vec<i32>>) -> i32 {
        use std::mem::swap;

        let m = obstacle_grid.len();
        let n = obstacle_grid[0].len();
        assert!(m >= 1 && n >= 1);

        let mut dp0 = vec![0i32; n];
        let mut dp1 = vec![0i32; n];
        dp0[0] = if obstacle_grid[0][0] == 0 { 1 } else { 0 };

        for (i, row) in obstacle_grid.iter().enumerate() {
            assert!(row.len() == n);

            dp1[0] = if row[0] == 0 { dp0[0] } else { 0 };

            for (j, &cell) in row.iter().enumerate().skip(1) {
                dp1[j] = if cell == 0 { dp1[j - 1] + dp0[j] } else { 0 };
            }

            swap(&mut dp0, &mut dp1);
        }

        dp0.last().copied().unwrap()
    }
}
