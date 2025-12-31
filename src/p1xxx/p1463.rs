// Problem 1463
impl Solution {
    pub fn cherry_pickup(grid: Vec<Vec<i32>>) -> i32 {
        let rows = grid.len();
        let cols = grid[0].len();

        let mut dp0 = vec![0; cols * cols];
        let mut dp1 = dp0.clone();

        for row in 0..rows {
            // Since items are occupied incrementally, we dont need reset them
            // dp1.fill(0);

            for i in 0..cols.min(row + 1) {
                for j in (cols - 1).saturating_sub(row)..cols {
                    let mut val = 0;

                    for p in i.saturating_sub(1)..(i + 2).min(cols) {
                        for q in j.saturating_sub(1)..(j + 2).min(cols) {
                            val = val.max(dp0[p * cols + q]);
                        }
                    }

                    val += grid[row][i];

                    if i != j {
                        val += grid[row][j];
                    }

                    dp1[i * cols + j] = val;
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.into_iter().max().unwrap()
    }
}
