// Problem 2976
impl Solution {
    pub fn minimum_cost(
        source: String,
        target: String,
        original: Vec<char>,
        changed: Vec<char>,
        cost: Vec<i32>,
    ) -> i64 {
        assert!(source.len() == target.len());
        assert!(original.len() == changed.len());
        assert!(original.len() == cost.len());

        let mut dp = vec![[i32::MAX; 26]; 26];

        for i in 0..original.len() {
            let src_id = (original[i] as u8 - b'a') as usize;
            let dst_id = (changed[i] as u8 - b'a') as usize;

            if src_id >= 26 || dst_id >= 26 {
                unreachable!("Bad character");
            }

            dp[src_id][dst_id] = dp[src_id][dst_id].min(cost[i]);
        }

        for i in 0..26 {
            dp[i][i] = 0;
        }

        // Floyd-Warshall algorithm
        for k in 0..26 {
            for i in 0..26 {
                for j in 0..26 {
                    if dp[i][k] == i32::MAX || dp[k][j] == i32::MAX {
                        // avoid overflow
                        continue;
                    }

                    if dp[i][j] > dp[i][k] + dp[k][j] {
                        dp[i][j] = dp[i][k] + dp[k][j];
                    }
                }
            }
        }

        let mut minimum_cost = 0i64;
        for (a, b) in source.bytes().zip(target.bytes()) {
            if a == b {
                continue;
            }

            let src_id = (a - b'a') as usize;
            let dst_id = (b - b'a') as usize;
            if src_id >= 26 || dst_id >= 26 {
                unreachable!("Bad character");
            }

            if dp[src_id][dst_id] == i32::MAX {
                return -1i64;
            }

            minimum_cost += i64::from(dp[src_id][dst_id]);
        }

        minimum_cost
    }
}
