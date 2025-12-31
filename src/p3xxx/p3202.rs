// Problem 3202
impl Solution {
    pub fn maximum_length(nums: Vec<i32>, k: i32) -> i32 {
        /*
         * Observations:
         * 1. Fix the equality value by enumerating 0 to k (exclusive), say, c.
         * 2. (a + b) % k == c => a % k = (c - b % k + k) % k
         */

        let mut count = 0;
        for c in 0..k {
            let mut dp = vec![0; k as usize];
            for b in nums.iter().copied() {
                let a = (c - b % k + k) % k;
                dp[(b % k) as usize] = dp[(a % k) as usize] + 1;
                count = count.max(dp[(b % k) as usize]);
            }
        }
        count
    }
}
