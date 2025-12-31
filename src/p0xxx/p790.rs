// Problem 790
impl Solution {
    pub fn num_tilings(n: i32) -> i32 {
        // we may rewrite the dp in recurision formula to boost the speed
        let n = n as usize;
        const MOD: i64 = 10i64.pow(9) + 7;
        let mut dp = vec![0i64; n + 1];
        dp[0] = 1;

        for i in 1..=n {
            dp[i] = dp[i - 1];

            if i >= 2 {
                // pattern:
                // A A
                // B B
                dp[i] += dp[i - 2]
            }

            if i >= 3 {
                // pattern:
                // X Y Y
                // X X Y
                // X C C Y Y
                // X X B B Y

                let mut left = i - 3;

                loop {
                    dp[i] += dp[left] * 2; // upside down

                    if left >= 2 {
                        left -= 2;
                    } else {
                        break;
                    }
                }
            }

            if i >= 4 {
                // pattern:
                // X C C Y
                // X X Y Y
                // X C C B B Y
                // X X A A Y Y

                let mut left = i - 4;

                loop {
                    dp[i] += dp[left] * 2; // upside down

                    if left >= 2 {
                        left -= 2;
                    } else {
                        break;
                    }
                }
            }

            dp[i] %= MOD;
        }

        dp[n] as i32
    }
}
