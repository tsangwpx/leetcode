// Problem 3578
impl Solution {
    pub fn count_partitions(nums: Vec<i32>, k: i32) -> i32 {
        /*
         * 1. Use two monotonic stacks to maintain a sliding window,  [i..j]
         * such that the distance between maximum and minimum is smaller or equal to k
         *
         * 2. In each iteration, push as many items as possible into the window with the distance <= k
         * 3. When push an element into the window, record dp[j + 1] = # of [0..j]
         * 4. Estimate dp[j + 2] = 2 * dp[j + 1]. This overestimate the number and will be corrected later.
         * 4. By the end of iteration, pop the leftmost element i from the window.
         *      a. Update both stacks.
         *      b. The number of ways of [0...j] by deleting dp[i + 1] ways from the estimate of dp[j + 2]
         */

        use std::collections::VecDeque;

        const MOD: i64 = 10i64.pow(9) + 7;

        let mut dp = vec![0; nums.len() + 1];
        let mut right = 0;
        let mut increasing = VecDeque::<usize>::new(); // front is minimum
        let mut decreasing = VecDeque::<usize>::new(); // front is maximum
        let mut acc = 1i64;

        dp[0] = 1;

        for (idx, item) in nums.iter().copied().enumerate() {
            // println!("idx {}", idx);

            while right < nums.len() {
                let new_item = nums[right];
                let new_min = increasing
                    .front()
                    .copied()
                    .map_or(new_item, |s| nums[s].min(new_item));
                let new_max = decreasing
                    .front()
                    .copied()
                    .map_or(new_item, |s| nums[s].max(new_item));

                if new_max - new_min > k {
                    break;
                }

                while let Some(idx2) = increasing.back().copied() {
                    let back = nums[idx2];

                    if back <= new_item {
                        break;
                    }

                    increasing.pop_back();
                }

                increasing.push_back(right);

                while let Some(idx2) = decreasing.back().copied() {
                    let back = nums[idx2];

                    if back >= new_item {
                        break;
                    }

                    decreasing.pop_back();
                }

                decreasing.push_back(right);

                dp[right + 1] = acc;
                acc = (acc + dp[right + 1]) % MOD;

                // println!("acc {} right {}: {:?}", acc, right, dp);
                // println!("inc {:?}", increasing);
                // println!("dec {:?}", decreasing);

                right += 1;
            }

            while increasing.front().copied().map_or(false, |s| s <= idx) {
                increasing.pop_front();
            }

            while decreasing.front().copied().map_or(false, |s| s <= idx) {
                decreasing.pop_front();
            }

            acc = (acc - dp[idx] + MOD) % MOD;
        }

        dp.last().copied().unwrap() as i32
    }
}
