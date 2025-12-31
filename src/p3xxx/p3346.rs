// Problem 3346
impl Solution {
    pub fn max_frequency(mut nums: Vec<i32>, k: i32, num_operations: i32) -> i32 {
        /*
         * Observations:
         * 1. Each element is at most selected once. We may sort the array so that
         *    the problem become a sliding window and the window size is the frequency of some computed element.
         * 2. There are two cases: The target element may already exist in the array or not.
         */

        use std::collections::HashMap;

        nums.sort();

        let mut counter = HashMap::<i32, i32>::new();
        let mut left = 0;
        let mut right = 0;

        let mut freq_max = 1;

        for target in nums.iter().copied() {
            // target in array
            // expand the window if possible

            while right < nums.len() && nums[right] <= target + k {
                *counter.entry(nums[right]).or_default() += 1;
                right += 1;
            }

            while left < nums.len() && nums[left] < target - k {
                *counter.entry(nums[left]).or_default() -= 1;
                left += 1;
            }

            freq_max = freq_max.max(
                ((right - left) as i32)
                    .min(num_operations + counter.entry(target).or_default().clone()),
            );
        }

        left = 0;

        for (right, item) in nums.iter().copied().enumerate() {
            // the target not exist in the array
            // the window width is 2k and the window size is limited by num_operations
            while left < nums.len() && (item - nums[left]) > 2 * k {
                left += 1;
            }

            freq_max = freq_max.max(((right - left + 1) as i32).min(num_operations));
        }

        freq_max
    }
}
