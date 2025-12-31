// Problem 837
impl Solution {
    pub fn new21_game(n: i32, k: i32, max_pts: i32) -> f64 {
        /*
         * dp[i] is the probability that dp[i] is touched.
         * so dp[0] = 1
         *
         * For index i <= k, dp[i] = (dp[i - w] + ... + dp[i - 1]) / w
         * For index i > k, dp[i] = (dp[i - w] + ... + dp[min(i - 1, k - 1)]) / w
         *
         * we are interested in dp[k] + ... + dp[n], which is the answer
         *
         * Define wsum as the sum of the parenthesis in the dp recursion equation.
         * This is a sum in a sliding window.
         */

        let n = n as usize;
        let k = k as usize;
        let w = max_pts as usize;

        if k == 0 || n >= k + w {
            return 1.0;
        }

        let mut prob = 0.0;
        let mut wsum = 1.0;

        let mut dp = vec![0.0; n + 1];
        dp[0] = 1.0;

        for i in 1..=n {
            dp[i] = wsum / w as f64;
            if i < k {
                wsum += dp[i];
            } else {
                prob += dp[i];
            }
            if i >= w {
                wsum -= dp[i - w];
            }
        }

        prob
    }
}
