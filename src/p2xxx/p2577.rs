// Problem 2577
impl Solution {
    pub fn minimum_time(grid: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let m = grid.len();
        let n = grid[0].len();
        assert!(m >= 2 && n >= 2);

        if grid[0][1] >= 2 && grid[1][0] >= 2 {
            // we need some space to waste time
            // or fail immediately.
            return -1;
        }

        let mut added = vec![false; m * n];
        added[0] = true;

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0), 0, 0));

        while let Some((Reverse(steps), row, col)) = pq.pop() {
            let row = row as usize;
            let col = col as usize;

            let neighbours = [
                (row, col.wrapping_sub(1)),
                (row, col.wrapping_add(1)),
                (row.wrapping_sub(1), col),
                (row.wrapping_add(1), col),
            ];

            for (i, j) in neighbours {
                if i >= m || j >= n {
                    // out of bounds
                    continue;
                }
                if added[i * n + j] {
                    continue;
                }
                added[i * n + j] = true;

                let mut min_steps = (i + j) as i32;
                let must_odd = (min_steps & 1) == 1;

                min_steps = min_steps.max(grid[i][j]).max(steps + 1);
                let is_odd = (min_steps & 1) == 1;

                if must_odd != is_odd {
                    // Cell with even i + j must be reached in even steps
                    // Cell with odd i + j must be reached in odd steps
                    // adjust min_steps here
                    min_steps += 1;
                }

                if i + 1 == m && j + 1 == n {
                    return min_steps;
                }

                pq.push((Reverse(min_steps), i, j));
            }
        }

        unreachable!("How come?")
    }
}
