// Problem 1293
impl Solution {
    pub fn shortest_path(grid: Vec<Vec<i32>>, k: i32) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let m = grid.len();
        let n = grid[0].len();
        let k = k as usize;

        if k >= m + n - 2 {
            return (m + n - 2) as i32;
        }

        assert!(m >= 1 && n >= 1 && k >= 1);

        let mut best = vec![vec![vec![u16::MAX; k + 1]; n]; m];
        best[0][0].fill(0);

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(0u16), k as u16, 0u8, 0u8));

        while let Some((Reverse(cost), tnt, i, j)) = pq.pop() {
            let i = i as usize;
            let j = j as usize;

            if i + 1 >= m && j + 1 >= n {
                return cost as i32;
            }

            let tnt = tnt as usize;

            if tnt >= k + 1 || cost > best[i][j][tnt] {
                continue;
            }

            let friends = [
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
                (i.wrapping_sub(1), j),
                (i.wrapping_add(1), j),
            ];

            let new_cost = cost + 1;

            for (row, col) in friends {
                if row >= m || col >= n {
                    continue;
                }
                let new_tnt = if grid[row][col] == 0 {
                    tnt
                } else if tnt > 0 {
                    tnt - 1
                } else {
                    // running out of TNTs
                    continue;
                };

                // this means we may arrive this cell with more TNTs
                for p in 0..new_tnt {
                    best[row][col][p] = best[row][col][p].min(new_cost);
                }

                if new_cost < best[row][col][new_tnt] {
                    best[row][col][new_tnt] = new_cost;
                    pq.push((Reverse(new_cost), new_tnt as u16, row as u8, col as u8));
                }
            }
        }

        -1
    }
}
