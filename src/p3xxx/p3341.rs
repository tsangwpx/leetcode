// Problem 3341
impl Solution {
    pub fn min_time_to_reach(move_time: Vec<Vec<i32>>) -> i32 {
        use std::cmp::Reverse;
        use std::collections::BinaryHeap;

        let n = move_time.len();
        let m = move_time[0].len();

        let mut cost = vec![vec![-1; m]; n];
        let mut pending = BinaryHeap::new();

        cost[0][0] = 0;
        pending.push((Reverse(0), 0, 0));

        while let Some((Reverse(time), i, j)) = pending.pop() {
            if i + 1 == n && j + 1 == m {
                return time;
            }

            if cost[i][j] >= 0 && time > cost[i][j] {
                continue;
            }

            for (i2, j2) in [
                (i.wrapping_sub(1), j),
                (i.wrapping_add(1), j),
                (i, j.wrapping_sub(1)),
                (i, j.wrapping_add(1)),
            ] {
                if i2 >= n || j2 >= m {
                    continue;
                }

                let time2 = move_time[i2][j2].max(time) + 1;
                if cost[i2][j2] < 0 || time2 < cost[i2][j2] {
                    cost[i2][j2] = time2;
                    pending.push((Reverse(time2), i2, j2));
                }
            }
        }

        -1
    }
}
