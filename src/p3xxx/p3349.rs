// Problem 3349
impl Solution {
    pub fn has_increasing_subarrays(nums: Vec<i32>, k: i32) -> bool {
        let mut last_count = 0;
        let mut curr_count = 0;

        for (idx, item) in nums.iter().copied().enumerate() {
            if idx == 0 || item > nums[idx - 1] {
                curr_count += 1;
            } else {
                last_count = curr_count;
                curr_count = 1;
            }

            if curr_count >= 2 * k || (last_count >= k && curr_count >= k) {
                return true;
            }
        }

        false
    }
}
