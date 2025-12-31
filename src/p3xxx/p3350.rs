// Problem 3350
impl Solution {
    pub fn max_increasing_subarrays(nums: Vec<i32>) -> i32 {
        let mut res = 1;
        let mut last_count = 0;
        let mut curr_count = 0;

        for (idx, item) in nums.iter().copied().enumerate() {
            if idx == 0 || item > nums[idx - 1] {
                curr_count += 1;
            } else {
                last_count = curr_count;
                curr_count = 1;
            }

            res = res.max(curr_count / 2).max(last_count.min(curr_count));
        }

        res
    }
}
