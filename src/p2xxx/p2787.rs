// Problem 2787
impl Solution {
    pub fn number_of_ways(n: i32, x: i32) -> i32 {
        const MOD: i64 = 10i64.pow(9) + 7;

        let mut dp = vec![0i32; 301];
        dp[0] = 1;

        for b in 1..=n {
            let pow = b.pow(x as u32);
            if pow > n {
                break;
            }

            for i in (pow..=n).rev() {
                let j = i - pow;
                // println!("{} {}", i, j);
                dp[i as usize] =
                    ((i64::from(dp[i as usize]) + i64::from(dp[j as usize])) % MOD) as i32;
            }
        }

        dp[n as usize]
    }
}
