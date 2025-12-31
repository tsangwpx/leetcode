// Problem 2267
impl Solution {
    pub fn has_valid_path(grid: Vec<Vec<char>>) -> bool {
        use std::collections::HashSet;

        let m = grid.len();
        let n = grid[0].len();

        let steps = m + n - 1;

        if (steps & 1) == 1 {
            // odd steps is impossible
            return false;
        }

        // grid size is at most 100x100
        // the longest valid pair is, for example, "(" x99 + ")" x99
        // use u128 as bit set is just fine!
        let mut dp0 = vec![0u128; n];
        let mut dp1 = dp0.clone();
        dp0[0] = 1;

        for row in grid.iter() {
            for (idx, item) in row.iter().copied().enumerate() {
                if item == '(' {
                    dp1[idx] = dp0[idx] << 1;
                } else {
                    dp1[idx] = dp0[idx] >> 1;
                }

                if idx > 0 {
                    if item == '(' {
                        dp1[idx] |= dp1[idx - 1] << 1;
                    } else {
                        dp1[idx] |= dp1[idx - 1] >> 1;
                    }
                }
            }

            std::mem::swap(&mut dp0, &mut dp1);
        }

        dp0.last().map(|&bits| (bits & 1) == 1).unwrap_or(false)
    }
}
