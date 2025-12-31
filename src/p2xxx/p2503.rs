// Problem 2503
impl Solution {
    pub fn max_points(grid: Vec<Vec<i32>>, queries: Vec<i32>) -> Vec<i32> {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let m = grid.len();
        let n = grid[0].len();

        let mut scores = vec![];

        let mut added = vec![false; m * n];
        added[0] = true;

        let mut pq = BinaryHeap::new();
        pq.push((Reverse(grid[0][0]), 0u16, 0u16));

        while !pq.is_empty() {
            let (Reverse(level), _, _) = pq.peek().copied().unwrap();
            let mut points = if let Some(&(_, points)) = scores.last() {
                points
            } else {
                0
            };

            while let Some(&(Reverse(lv), row, col)) = pq.peek() {
                if lv > level {
                    break;
                }

                let row = row as usize;
                let col = col as usize;

                pq.pop();
                points += 1;

                let north = row.wrapping_sub(1);
                let south = row.wrapping_add(1);
                let west = col.wrapping_sub(1);
                let east = col.wrapping_add(1);

                let neighbours = [
                    (row, west),  // left
                    (row, east),  // right
                    (north, col), //top
                    (south, col), // bottom
                ];

                for (next_row, next_col) in neighbours {
                    if next_row >= m || next_col >= n {
                        // out of bounds
                        continue;
                    }
                    if added[next_row * n + next_col] {
                        continue;
                    }
                    added[next_row * n + next_col] = true;
                    pq.push((
                        Reverse(grid[next_row][next_col]),
                        next_row as u16,
                        next_col as u16,
                    ));
                }
            }

            scores.push((level, points));
        }

        // println!("{:?}", scores);

        queries
            .iter()
            .copied()
            .map(|level| {
                let pos = scores.partition_point(|&(lv, _)| lv < level);

                // if pos = 0, no score with higher level, return zero
                // if pos = len, level is overpowered, pick last score
                // otherwise, pick the score at (pos - 1)

                if pos > 0 { scores[pos - 1].1 } else { 0 }
            })
            .collect::<Vec<_>>()
    }
}
