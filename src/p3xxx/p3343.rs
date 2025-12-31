// Problem 3343
impl Solution {
    pub fn count_balanced_permutations(num: String) -> i32 {
        // https://leetcode.com/problems/count-number-of-balanced-permutations/solutions/6726791/dp-combinatorics-step-by-step-with-images-example-walkthrough-c-python-java/?envType=daily-question&envId=2025-05-09
        const MOD: i64 = 10i64.pow(9) + 7;

        let len = num.len();
        let half_len = len / 2;

        let sum = num.bytes().fold(0, |acc, ch| (acc + (ch - b'0') as usize));
        let half_sum = sum / 2;

        if sum % 2 == 1 {
            return 0;
        }

        let mut fact = vec![1; len + 1];
        for k in 1..fact.len() {
            fact[k] = fact[k - 1] * k as i64 % MOD;
        }

        // https://cp-algorithms.com/algebra/module-inverse.html
        let mut inv = vec![1; len + 1];
        for k in 2..inv.len() {
            let a = k as i64;

            inv[k] = MOD - (MOD / a) * inv[(MOD % a) as usize] % MOD;
        }

        let mut fact_inv = vec![1; fact.len()];
        for k in 1..fact_inv.len() {
            fact_inv[k] = fact_inv[k - 1] * inv[k] % MOD;
        }

        let mut counter = [0i32; 10];
        let mut dp = vec![vec![0; half_len + 1]; half_sum + 1];
        dp[0][0] = 1;

        for d in num.bytes() {
            let d = (d - b'0') as usize;
            counter[d] += 1;

            for i in (d..=half_sum).rev() {
                for j in (1..=half_len).rev() {
                    dp[i][j] = (dp[i][j] + dp[i - d][j - 1]) % MOD;
                }
            }
        }

        let mut res = dp[half_sum][half_len];

        res *= fact[half_len];
        res %= MOD;

        res *= fact[len - half_len];
        res %= MOD;

        for count in counter.iter().copied() {
            res *= fact_inv[count as usize];
            res %= MOD;
        }

        res as i32
    }
}
