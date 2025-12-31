/// Interesting solutions:
/// https://leetcode.com/problems/perfect-squares/submissions/1119137709/
/// Lagrange's Four Square Theorem, Three Square Theorem
///
// Problem 279
impl Solution {
    pub fn num_squares(n: i32) -> i32 {
        let mut dp = vec![n; n as usize + 1];

        dp[0] = 0;
        for i in 1i32.. {
            let product = i * i;
            if product > n {
                break;
            }
            dp[product as usize] = 1;
        }

        if dp[n as usize] == 1 {
            return 1;
        }

        for whole in 2..=n {
            for square in (1..).into_iter().map(|s| s * s) {
                if square >= whole {
                    break;
                }

                let delta = whole - square;
                dp[whole as usize] = dp[whole as usize].min(dp[delta as usize] + 1);
            }
        }

        dp[n as usize]
    }
}
