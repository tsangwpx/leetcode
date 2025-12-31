use std::vec;

// Problem 3098
impl Solution {
    pub fn sum_of_powers(nums: Vec<i32>, k: i32) -> i32 {
        use std::collections::HashMap;

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut nums = nums;
        nums.sort_unstable();

        let k = k as usize;

        // dp[last_index][subsequence len]: dist -> count
        let mut dp = vec![vec![HashMap::new(); k + 1]; nums.len()];

        for (idx, number) in nums.iter().copied().enumerate() {
            let (dp0, dp1) = dp.split_at_mut(idx);
            let dp1 = &mut dp1[0];

            for last in 0..idx {
                let dist = (number - nums[last]).abs();

                for len in 2..k {
                    for (&min_dist, &multiple) in dp0[last][len].iter() {
                        let dist = dist.min(min_dist);

                        dp1[len + 1]
                            .entry(dist)
                            .and_modify(|count| *count = (*count + multiple) % MOD)
                            .or_insert(multiple);
                    }
                }

                dp1[2]
                    .entry(dist)
                    .and_modify(|count2| *count2 += 1i64)
                    .or_insert(1i64);
            }
        }

        let sum = dp
            .iter()
            .flat_map(|s| &s[k])
            .fold(0i64, |mut sum, (&dist, &count)| {
                sum += i64::from(dist) * i64::from(count);
                (sum + MOD) % MOD
            });

        sum as _
    }
}
