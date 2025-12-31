// Problem 2327
impl Solution {
    pub fn people_aware_of_secret(n: i32, delay: i32, forget: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let n = n as usize;
        let delay = delay as usize;
        let forget = forget as usize;
        let mut dp = vec![0i64; n + forget + 1];

        dp[0] += 1;

        let mut people = 0;
        let mut share = 0;

        for day in 0..n {
            if day >= delay {
                share += dp[day - delay];
            }

            if day >= forget {
                share += -dp[day - forget] + MOD;
                people += -dp[day - forget] + MOD;
            }

            share %= MOD;

            dp[day] += share;
            dp[day] %= MOD;

            people += dp[day];
            people %= MOD;

            // println!(
            //     "Day {}: people={}, new people={} sharing={}",
            //     day + 1,
            //     people,
            //     dp[day],
            //     share
            // );
        }

        people as i32
    }
}
