// Problem 85
impl Solution {
    pub fn maximal_rectangle(matrix: Vec<Vec<char>>) -> i32 {
        let nrows = matrix.len();
        let ncols = matrix[0].len();

        let mut heights = vec![0; ncols];
        let mut stack: Vec<(i32, i32)> = vec![]; // strictly increasing stack
        let mut max_rect = 0;

        for i in 0..nrows {
            for j in 0..ncols {
                heights[j] = if matrix[i][j] == '1' {
                    heights[j] + 1
                } else {
                    0
                };
            }

            // println!("{:?}", heights);
            stack.clear();

            for (idx, &height) in heights.iter().enumerate() {
                if height == 0 {
                    stack.clear();
                    continue;
                }

                max_rect = max_rect.max(height);

                let stop = idx as i32 + 1;
                for (start, prev) in stack.iter().copied() {
                    max_rect = max_rect.max((stop - start) * prev.min(height));
                }

                let mut popped = None;

                while let Some(&(_, prev)) = stack.last() {
                    if prev >= height {
                        popped = stack.pop();
                    } else {
                        break;
                    }
                }

                if let Some((start, _)) = popped {
                    // popped height must no less than current height
                    // reuse the popped index with our height
                    stack.push((start, height));
                } else {
                    stack.push((idx as i32, height));
                }

                // println!("{:?}", stack);
            }
            // println!("max_rect={}", max_rect);
        }

        max_rect
    }

    pub fn maximal_rectangle_dp3d(matrix: Vec<Vec<char>>) -> i32 {
        let nrows = matrix.len();
        let ncols = matrix[0].len();

        let mut dp = vec![vec![vec![0i32; ncols]; nrows]; nrows];
        let mut max_rect = 0;

        for i in 0..nrows {
            for j in 0..ncols {
                if matrix[i][j] == '1' {
                    if j > 0 {
                        dp[0][i][j] += dp[0][i][j - 1];
                    }

                    dp[0][i][j] += 1;
                    max_rect = max_rect.max(dp[0][i][j]);
                }
            }
        }
        // println!("{}", 0);
        // println!("{:?}", dp[0]);

        for h in 1..nrows {
            let height = h as i32 + 1;

            for i in h..nrows {
                for j in 0..ncols {
                    if matrix[i][j] == '1' {
                        let mut width = dp[h - 1][i - 1][j];
                        if j > 0 {
                            width = width.min(dp[h][i][j - 1] + 1);
                        } else {
                            width = width.min(1);
                        }
                        dp[h][i][j] = width;
                        max_rect = max_rect.max(height * width);
                    }
                }
            }
            // println!("{}", h);
            // println!("{}: {:?}", h, dp[h]);
        }

        max_rect
    }
}
