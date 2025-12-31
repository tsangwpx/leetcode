// Problem 2563
impl Solution {
    pub fn count_fair_pairs(mut nums: Vec<i32>, lower: i32, upper: i32) -> i64 {
        nums.sort_unstable();

        fn count_le(nums: &[i32], bound: i32) -> i64 {
            let mut count = 0;
            let mut left = 0;
            let mut right = nums.len() - 1;

            while left < right {
                while left < right && nums[left] + nums[right] > bound {
                    right -= 1;
                }

                count += right - left;
                left += 1;
            }

            count as i64
        }

        count_le(&nums, upper) - count_le(&nums, lower - 1)
    }
}
