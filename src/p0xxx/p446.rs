// Problem 446
impl Solution {
    pub fn number_of_arithmetic_slices(nums: Vec<i32>) -> i32 {
        use std::collections::HashMap;

        let mut dp = vec![HashMap::<i64, i32>::new(); nums.len()];
        let mut count = 0;

        for i in 1..nums.len() {
            for j in 0..i {
                let delta = i64::from(nums[i]) - i64::from(nums[j]);
                let delta_count = dp[j].get(&delta).copied().unwrap_or(0);

                if delta_count >= 1 {
                    count += delta_count;
                }

                dp[i]
                    .entry(delta)
                    .and_modify(|record| *record += delta_count + 1)
                    .or_insert(delta_count + 1);
            }
        }

        count
    }
}
