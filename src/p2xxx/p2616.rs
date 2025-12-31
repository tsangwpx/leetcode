// Problem 2616
impl Solution {
    pub fn minimize_max(nums: Vec<i32>, p: i32) -> i32 {
        let mut nums = nums;

        nums.sort_unstable();

        let mut left = 0;
        let mut right = nums.last().copied().unwrap() - nums.first().copied().unwrap();

        while left < right {
            let mid = left + (right - left) / 2;

            let mut index = 1;
            let mut count = 0;
            while index < nums.len() {
                let dist = (nums[index] - nums[index - 1]).abs();
                if dist <= mid {
                    count += 1;
                    index += 2;
                } else {
                    index += 1
                }
            }

            if count < p {
                left = mid + 1;
            } else {
                right = mid;
            }
        }

        left
    }
}
