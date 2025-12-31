// Problem 3147
impl Solution {
    pub fn maximum_energy(energy: Vec<i32>, k: i32) -> i32 {
        let k = k as usize;
        let mut cycle = 0;
        let mut dp = vec![i32::MIN; k];

        for delta in energy.iter().copied() {
            if dp[cycle] == i32::MIN {
                dp[cycle] = delta;
            } else {
                dp[cycle] = (dp[cycle] + delta).max(delta);
            }

            cycle += 1;
            if cycle >= k {
                cycle = 0;
            }
        }

        dp.iter().copied().max().unwrap()
    }
}
